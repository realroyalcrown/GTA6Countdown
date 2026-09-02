//! Where the widget sits on screen.
//!
//! Tauri's own `Window::center` measures the window as the window server
//! currently knows it, which is wrong right after a resize, and it centres
//! against the full display rather than the usable area. With two monitors and
//! a menu bar that lands the widget noticeably off. Both are handled here
//! instead: the caller passes the size it just asked for, and the maths runs
//! against the active monitor's work area.

use tauri::{PhysicalPosition, Runtime, WebviewWindow};

/// Centres the window in the usable area of the monitor it currently occupies.
pub fn center<R: Runtime>(
    window: &WebviewWindow<R>,
    logical_size: (f64, f64),
) -> tauri::Result<()> {
    // Right after a resize the window server can briefly report no current
    // monitor, so fall back rather than let Tauri centre against the full
    // display — that is the off-by-a-menu-bar result we are avoiding.
    let monitor = match window.current_monitor()? {
        Some(monitor) => monitor,
        None => match active_monitor(window)? {
            Some(monitor) => monitor,
            None => return window.center(),
        },
    };

    let area = monitor.work_area();
    let scale = monitor.scale_factor();
    let width = (logical_size.0 * scale).round() as i32;
    let height = (logical_size.1 * scale).round() as i32;

    window.set_position(PhysicalPosition::new(
        area.position.x + (area.size.width as i32 - width) / 2,
        area.position.y + (area.size.height as i32 - height) / 2,
    ))
}

/// Restores a remembered position, falling back to centring when those
/// coordinates no longer land on a connected display.
pub fn restore<R: Runtime>(
    window: &WebviewWindow<R>,
    position: Option<(i32, i32)>,
    logical_size: (f64, f64),
) -> tauri::Result<()> {
    let Some((x, y)) = position else {
        return center(window, logical_size);
    };

    let on_screen = window
        .available_monitors()
        .map(|monitors| monitors.iter().any(|monitor| contains(monitor, x, y)))
        .unwrap_or(false);

    if on_screen {
        window.set_position(PhysicalPosition::new(x, y))
    } else {
        center(window, logical_size)
    }
}

fn active_monitor<R: Runtime>(window: &WebviewWindow<R>) -> tauri::Result<Option<tauri::Monitor>> {
    if let Some(primary) = window.primary_monitor()? {
        return Ok(Some(primary));
    }
    Ok(window.available_monitors()?.into_iter().next())
}

fn contains(monitor: &tauri::Monitor, x: i32, y: i32) -> bool {
    let origin = monitor.position();
    let size = monitor.size();

    x >= origin.x
        && y >= origin.y
        && x < origin.x + size.width as i32
        && y < origin.y + size.height as i32
}
