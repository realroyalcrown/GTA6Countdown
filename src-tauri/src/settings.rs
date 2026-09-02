//! Remembers where the user put the widget and how they configured it.
//!
//! A single small JSON file is enough here; a store plugin would add a
//! dependency for one struct.

use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};

use crate::desktop_window::StackingMode;
use crate::state::AppState;
use crate::widget_size::WidgetSize;

const FILE_NAME: &str = "widget.json";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PersistedSettings {
    pub size: WidgetSize,
    pub stacking: StackingMode,
    /// Physical pixels, so the widget lands in the same spot on mixed-DPI setups.
    pub position: Option<(i32, i32)>,
}

impl Default for PersistedSettings {
    fn default() -> Self {
        Self {
            size: WidgetSize::Medium,
            stacking: StackingMode::Desktop,
            position: None,
        }
    }
}

pub fn load<R: Runtime>(app: &AppHandle<R>) -> PersistedSettings {
    let Some(path) = file_path(app) else {
        return PersistedSettings::default();
    };

    fs::read_to_string(path)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

/// Captures the live window geometry and state, then writes it out.
pub fn save<R: Runtime>(app: &AppHandle<R>) {
    let Some(path) = file_path(app) else { return };
    let state = app.state::<AppState>();

    let position = app
        .get_webview_window(crate::commands::MAIN_WINDOW)
        .and_then(|window| window.outer_position().ok())
        .map(|point| (point.x, point.y));

    let settings = PersistedSettings {
        size: state.size(),
        stacking: state.stacking(),
        position,
    };

    let Ok(serialized) = serde_json::to_string_pretty(&settings) else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, serialized);
}

fn file_path<R: Runtime>(app: &AppHandle<R>) -> Option<PathBuf> {
    app.path()
        .app_config_dir()
        .ok()
        .map(|dir| dir.join(FILE_NAME))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_json() {
        let settings = PersistedSettings {
            size: WidgetSize::Large,
            stacking: StackingMode::AlwaysOnTop,
            position: Some((120, 140)),
        };

        let encoded = serde_json::to_string(&settings).expect("serialises");
        let decoded: PersistedSettings = serde_json::from_str(&encoded).expect("deserialises");

        assert_eq!(decoded.size, settings.size);
        assert_eq!(decoded.stacking, settings.stacking);
        assert_eq!(decoded.position, settings.position);
    }

    /// A config written by an older build must not wipe the user's setup.
    #[test]
    fn fills_in_missing_fields_from_defaults() {
        let decoded: PersistedSettings =
            serde_json::from_str(r#"{"size":"small"}"#).expect("deserialises");

        assert_eq!(decoded.size, WidgetSize::Small);
        assert_eq!(decoded.stacking, StackingMode::Desktop);
        assert_eq!(decoded.position, None);
    }
}
