//! Makes the countdown in the menu bar look like the system clock next to it.
//!
//! Tauri exposes a tray title as a plain string and leaves the styling to
//! AppKit's defaults, which renders the text in the standard menu bar font but
//! with proportional figures — so a ticking countdown visibly shifts every
//! second. Reaching the underlying `NSStatusBarButton` lets us keep Apple's own
//! font and size while switching to monospaced digits, which is what the HIG
//! recommends for numbers that change in place.

#[cfg(target_os = "macos")]
pub fn apply_system_clock_style() {
    use objc2::rc::autoreleasepool;
    use objc2::runtime::AnyObject;
    use objc2::{class, msg_send};

    /// `NSFontWeightRegular`, matching the clock rather than a bolder status item.
    const REGULAR_WEIGHT: f64 = 0.0;
    /// `NSTextAlignment.center`.
    const ALIGN_CENTER: isize = 2;

    autoreleasepool(|_| unsafe {
        let Some(button) = status_bar_button() else {
            return;
        };

        // Ask AppKit for the menu bar's own size instead of hard-coding 14pt,
        // which changes with the "Larger Text" accessibility setting.
        let menu_font: *mut AnyObject = msg_send![class!(NSFont), menuBarFontOfSize: 0.0f64];
        if menu_font.is_null() {
            return;
        }
        let point_size: f64 = msg_send![menu_font, pointSize];

        let font: *mut AnyObject = msg_send![
            class!(NSFont),
            monospacedDigitSystemFontOfSize: point_size,
            weight: REGULAR_WEIGHT
        ];
        if font.is_null() {
            return;
        }

        let _: () = msg_send![button, setFont: font];
        let _: () = msg_send![button, setAlignment: ALIGN_CENTER];
    });
}

/// Walks the app's windows for the one AppKit created to host our status item.
#[cfg(target_os = "macos")]
unsafe fn status_bar_button() -> Option<*mut objc2::runtime::AnyObject> {
    use objc2::runtime::AnyObject;
    use objc2::{class, msg_send};

    let app: *mut AnyObject = msg_send![class!(NSApplication), sharedApplication];
    if app.is_null() {
        return None;
    }

    let windows: *mut AnyObject = msg_send![app, windows];
    if windows.is_null() {
        return None;
    }
    let count: usize = msg_send![windows, count];

    for index in 0..count {
        let window: *mut AnyObject = msg_send![windows, objectAtIndex: index];
        if window.is_null() {
            continue;
        }

        let view: *mut AnyObject = msg_send![window, contentView];
        if view.is_null() {
            continue;
        }

        let is_status_button: bool = msg_send![view, isKindOfClass: class!(NSStatusBarButton)];
        if is_status_button {
            return Some(view);
        }
    }

    None
}

#[cfg(not(target_os = "macos"))]
pub fn apply_system_clock_style() {}
