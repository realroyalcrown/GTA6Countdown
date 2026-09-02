mod commands;
mod countdown;
mod desktop_window;
mod menu_bar;
mod placement;
mod settings;
mod state;
mod tray;
mod typeface;
mod widget_size;

use std::time::Duration;

use tauri::{Manager, WindowEvent};

use crate::state::AppState;

/// Long enough that dragging the widget does not write on every frame, short
/// enough that a quick move followed by a crash still survives.
const POSITION_SAVE_INTERVAL: Duration = Duration::from_millis(800);

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::get_release_target,
            commands::get_display_typeface,
            commands::get_widget_state,
            commands::set_widget_size,
            commands::set_stacking_mode,
            commands::hide_widget,
            commands::center_window,
        ])
        .setup(|app| {
            let handle = app.handle().clone();

            // A desktop widget belongs in the menu bar, not the Dock or the
            // app switcher.
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let saved = settings::load(&handle);
            let state = app.state::<AppState>();
            state.set_size(saved.size);
            state.set_stacking(saved.stacking);

            if let Some(window) = app.get_webview_window(commands::MAIN_WINDOW) {
                let dimensions = saved.size.dimensions();
                window.set_size(tauri::LogicalSize::new(dimensions.0, dimensions.1))?;
                placement::restore(&window, saved.position, dimensions)?;
                desktop_window::apply(&window, saved.stacking)?;
            }

            tray::build(&handle)?;
            tray::spawn_ticker(handle);

            Ok(())
        })
        .on_window_event(|window, event| {
            if matches!(event, WindowEvent::Moved(_)) {
                let app = window.app_handle();
                if app.state::<AppState>().persist_due(POSITION_SAVE_INTERVAL) {
                    settings::save(app);
                }
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            // The throttled Moved handler can miss the final drag position.
            if matches!(event, tauri::RunEvent::Exit) {
                settings::save(app);
            }
        });
}
