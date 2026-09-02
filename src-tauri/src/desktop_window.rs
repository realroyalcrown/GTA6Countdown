//! Turns an ordinary Tauri window into a macOS desktop widget.
//!
//! A desktop widget lives above the desktop picture and its icons but below
//! every application window, and it stays put across Spaces. AppKit exposes
//! that through the window level and collection behaviour, neither of which
//! Tauri wraps, so both are set through the underlying `NSWindow`.

use serde::{Deserialize, Serialize};

/// `CGWindowLevelForKey(kCGDesktopIconWindowLevelKey) + 1` — just above the
/// desktop icons, far below `kCGNormalWindowLevel` (0).
#[cfg(target_os = "macos")]
const DESKTOP_WIDGET_LEVEL: isize = -2_147_483_602;

/// `kCGNormalWindowLevel + 3`, the level AppKit uses for floating panels.
#[cfg(target_os = "macos")]
const FLOATING_LEVEL: isize = 3;

#[cfg(target_os = "macos")]
mod collection_behavior {
    pub const CAN_JOIN_ALL_SPACES: usize = 1 << 0;
    pub const STATIONARY: usize = 1 << 4;
    pub const IGNORES_CYCLE: usize = 1 << 6;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StackingMode {
    /// Pinned to the desktop, behind all app windows.
    Desktop,
    /// Floating above other windows.
    AlwaysOnTop,
}

impl StackingMode {
    pub fn toggled(self) -> Self {
        match self {
            StackingMode::Desktop => StackingMode::AlwaysOnTop,
            StackingMode::AlwaysOnTop => StackingMode::Desktop,
        }
    }
}

#[cfg(target_os = "macos")]
pub fn apply<R: tauri::Runtime>(
    window: &tauri::WebviewWindow<R>,
    mode: StackingMode,
) -> tauri::Result<()> {
    use objc2::msg_send;
    use objc2::runtime::AnyObject;

    let ns_window = window.ns_window()? as *mut AnyObject;
    if ns_window.is_null() {
        return Ok(());
    }

    let level = match mode {
        StackingMode::Desktop => DESKTOP_WIDGET_LEVEL,
        StackingMode::AlwaysOnTop => FLOATING_LEVEL,
    };
    let behavior = collection_behavior::CAN_JOIN_ALL_SPACES
        | collection_behavior::STATIONARY
        | collection_behavior::IGNORES_CYCLE;

    unsafe {
        let _: () = msg_send![ns_window, setLevel: level];
        let _: () = msg_send![ns_window, setCollectionBehavior: behavior];
    }

    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub fn apply<R: tauri::Runtime>(
    window: &tauri::WebviewWindow<R>,
    mode: StackingMode,
) -> tauri::Result<()> {
    window.set_always_on_top(matches!(mode, StackingMode::AlwaysOnTop))
}
