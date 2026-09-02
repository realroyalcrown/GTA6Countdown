//! Everything that mutates the widget, callable from both the tray menu and
//! the React frontend so the two can never drift apart.

use tauri::{AppHandle, Emitter, LogicalSize, Manager, Runtime};

use crate::countdown::{self, ReleaseTarget};
use crate::desktop_window::{self, StackingMode};
use crate::placement;
use crate::settings;
use crate::state::{AppState, WidgetStateSnapshot};
use crate::tray;
use crate::typeface;
use crate::widget_size::WidgetSize;

pub const MAIN_WINDOW: &str = "main";

/// Emitted after any state change so the UI can re-render its layout.
const STATE_EVENT: &str = "widget://state-changed";

fn broadcast<R: Runtime>(app: &AppHandle<R>) {
    let snapshot = app.state::<AppState>().snapshot();
    let _ = app.emit(STATE_EVENT, snapshot);
    tray::refresh_menu(app);
    settings::save(app);
}

pub fn apply_size<R: Runtime>(app: AppHandle<R>, size: WidgetSize) -> tauri::Result<()> {
    let Some(window) = app.get_webview_window(MAIN_WINDOW) else {
        return Ok(());
    };

    let (width, height) = size.dimensions();
    window.set_size(LogicalSize::new(width, height))?;
    app.state::<AppState>().set_size(size);
    broadcast(&app);

    Ok(())
}

pub fn toggle_visibility<R: Runtime>(app: AppHandle<R>) -> tauri::Result<()> {
    let Some(window) = app.get_webview_window(MAIN_WINDOW) else {
        return Ok(());
    };

    let state = app.state::<AppState>();
    let next = !state.visible();

    if next {
        window.show()?;
        // Re-assert the window level: AppKit resets it when a window is
        // ordered back in.
        desktop_window::apply(&window, state.stacking())?;
    } else {
        window.hide()?;
    }

    state.set_visible(next);
    broadcast(&app);

    Ok(())
}

pub fn toggle_stacking<R: Runtime>(app: AppHandle<R>) -> tauri::Result<()> {
    let Some(window) = app.get_webview_window(MAIN_WINDOW) else {
        return Ok(());
    };

    let state = app.state::<AppState>();
    let next = state.stacking().toggled();
    desktop_window::apply(&window, next)?;
    state.set_stacking(next);
    broadcast(&app);

    Ok(())
}

pub fn center_widget<R: Runtime>(app: AppHandle<R>) -> tauri::Result<()> {
    let Some(window) = app.get_webview_window(MAIN_WINDOW) else {
        return Ok(());
    };

    let size = app.state::<AppState>().size().dimensions();
    placement::center(&window, size)?;
    settings::save(&app);

    Ok(())
}

#[tauri::command]
pub fn get_release_target() -> ReleaseTarget {
    countdown::release_target()
}

#[tauri::command]
pub fn get_display_typeface() -> Option<typeface::TypefaceAsset> {
    typeface::find()
}

#[tauri::command]
pub fn get_widget_state(state: tauri::State<'_, AppState>) -> WidgetStateSnapshot {
    state.snapshot()
}

#[tauri::command]
pub fn set_widget_size(app: AppHandle, size: WidgetSize) -> Result<(), String> {
    apply_size(app, size).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_stacking_mode(app: AppHandle, mode: StackingMode) -> Result<(), String> {
    let Some(window) = app.get_webview_window(MAIN_WINDOW) else {
        return Ok(());
    };

    desktop_window::apply(&window, mode).map_err(|error| error.to_string())?;
    app.state::<AppState>().set_stacking(mode);
    broadcast(&app);

    Ok(())
}

#[tauri::command]
pub fn hide_widget(app: AppHandle) -> Result<(), String> {
    if app.state::<AppState>().visible() {
        toggle_visibility(app).map_err(|error| error.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn center_window(app: AppHandle) -> Result<(), String> {
    center_widget(app).map_err(|error| error.to_string())
}
