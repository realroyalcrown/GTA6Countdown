//! Menu bar presence: a live ticking countdown plus the widget's controls.

use std::time::Duration;

use chrono::{Local, Timelike};
use tauri::menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder, PredefinedMenuItem};
use tauri::tray::{TrayIcon, TrayIconBuilder};
use tauri::{include_image, AppHandle, Manager, Runtime};
use tauri_plugin_opener::OpenerExt;

use crate::countdown;
use crate::desktop_window::StackingMode;
use crate::state::AppState;
use crate::widget_size::WidgetSize;

pub const TRAY_ID: &str = "gta6-countdown-tray";

const AUTHOR_URL: &str = "https://realroyalcrown.eu";

pub fn build<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<TrayIcon<R>> {
    let tray = TrayIconBuilder::with_id(TRAY_ID)
        // A template image lets AppKit tint the mark for light and dark menu bars.
        .icon(include_image!("./icons/tray.png"))
        .icon_as_template(true)
        .title(tray_title())
        .tooltip("Grand Theft Auto VI — November 19, 2026")
        .menu(&menu(app)?)
        .on_menu_event(handle_menu_event)
        .build(app)?;

    crate::menu_bar::apply_system_clock_style();

    Ok(tray)
}

fn menu<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<tauri::menu::Menu<R>> {
    let state = app.state::<AppState>();
    let active_size = state.size();
    let stacking = state.stacking();
    let visible = state.visible();

    let mut builder = MenuBuilder::new(app).item(
        &MenuItemBuilder::with_id("header", "Grand Theft Auto VI")
            .enabled(false)
            .build(app)?,
    );

    builder = builder.item(
        &MenuItemBuilder::with_id("release-date", "Releases November 19, 2026")
            .enabled(false)
            .build(app)?,
    );
    builder = builder.item(&PredefinedMenuItem::separator(app)?);

    for size in WidgetSize::ALL {
        builder = builder.item(
            &CheckMenuItemBuilder::with_id(format!("size:{}", size.id()), size.label())
                .checked(size == active_size)
                .build(app)?,
        );
    }

    builder = builder.item(&PredefinedMenuItem::separator(app)?);
    builder = builder.item(
        &CheckMenuItemBuilder::with_id("toggle-visibility", "Show on Desktop")
            .checked(visible)
            .build(app)?,
    );
    builder = builder.item(
        &CheckMenuItemBuilder::with_id("toggle-stacking", "Float Above Windows")
            .checked(stacking == StackingMode::AlwaysOnTop)
            .build(app)?,
    );
    builder = builder.item(
        &MenuItemBuilder::with_id("center-widget", "Move to Screen Center").build(app)?,
    );

    builder = builder.item(&PredefinedMenuItem::separator(app)?);
    builder = builder
        .item(&MenuItemBuilder::with_id("author-site", "RealRoyalCrown.eu").build(app)?);

    builder = builder.item(&PredefinedMenuItem::separator(app)?);
    builder = builder.item(&MenuItemBuilder::with_id("quit", "Quit").build(app)?);

    builder.build()
}

/// Rebuilds the menu so the check marks reflect the current state.
pub fn refresh_menu<R: Runtime>(app: &AppHandle<R>) {
    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        if let Ok(menu) = menu(app) {
            let _ = tray.set_menu(Some(menu));
        }
    }
}

fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, event: tauri::menu::MenuEvent) {
    let id = event.id().0.as_str();

    match id {
        "quit" => app.exit(0),
        "toggle-visibility" => {
            let _ = crate::commands::toggle_visibility(app.clone());
        }
        "toggle-stacking" => {
            let _ = crate::commands::toggle_stacking(app.clone());
        }
        "center-widget" => {
            let _ = crate::commands::center_widget(app.clone());
        }
        "author-site" => {
            let _ = app.opener().open_url(AUTHOR_URL, None::<&str>);
        }
        other => {
            if let Some(size) = other.strip_prefix("size:").and_then(WidgetSize::from_id) {
                let _ = crate::commands::apply_size(app.clone(), size);
            }
        }
    }
}

/// Ticks the menu bar title once per second, aligned to the wall clock so the
/// displayed seconds change exactly when the system clock does.
pub fn spawn_ticker<R: Runtime>(app: AppHandle<R>) {
    std::thread::spawn(move || loop {
        std::thread::sleep(millis_until_next_second());

        let Some(tray) = app.tray_by_id(TRAY_ID) else {
            break;
        };
        let _ = tray.set_title(Some(tray_title()));
    });
}

/// AppKit sets the title flush against the icon, which reads as one blob. The
/// leading space restores the gap the surrounding status items have; AppKit
/// offers no spacing property for a button-hosted status item.
fn tray_title() -> String {
    format!(" {}", countdown::menu_bar_label(countdown::remaining_now()))
}

fn millis_until_next_second() -> Duration {
    let now = Local::now();
    let elapsed = u64::from(now.nanosecond() / 1_000_000).min(999);
    Duration::from_millis(1_000 - elapsed)
}
