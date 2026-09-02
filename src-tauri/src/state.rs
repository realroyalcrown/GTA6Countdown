//! Shared widget state. Small enough that a mutex beats a message channel.

use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde::Serialize;

use crate::desktop_window::StackingMode;
use crate::widget_size::WidgetSize;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetStateSnapshot {
    pub size: WidgetSize,
    pub stacking: StackingMode,
    pub visible: bool,
}

#[derive(Debug)]
pub struct AppState {
    inner: Mutex<WidgetStateSnapshot>,
    last_persist: Mutex<Instant>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            inner: Mutex::new(WidgetStateSnapshot {
                size: WidgetSize::Medium,
                stacking: StackingMode::Desktop,
                visible: true,
            }),
            last_persist: Mutex::new(Instant::now()),
        }
    }
}

impl AppState {
    /// Rate limits the position writes that a window drag would otherwise
    /// trigger on every frame.
    pub fn persist_due(&self, min_interval: Duration) -> bool {
        let Ok(mut last) = self.last_persist.lock() else {
            return false;
        };

        if last.elapsed() < min_interval {
            return false;
        }

        *last = Instant::now();
        true
    }

    pub fn snapshot(&self) -> WidgetStateSnapshot {
        *self.lock()
    }

    pub fn size(&self) -> WidgetSize {
        self.lock().size
    }

    pub fn stacking(&self) -> StackingMode {
        self.lock().stacking
    }

    pub fn visible(&self) -> bool {
        self.lock().visible
    }

    pub fn set_size(&self, size: WidgetSize) {
        self.lock().size = size;
    }

    pub fn set_stacking(&self, stacking: StackingMode) {
        self.lock().stacking = stacking;
    }

    pub fn set_visible(&self, visible: bool) {
        self.lock().visible = visible;
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, WidgetStateSnapshot> {
        self.inner.lock().unwrap_or_else(|poisoned| {
            self.inner.clear_poison();
            poisoned.into_inner()
        })
    }
}
