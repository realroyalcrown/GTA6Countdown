//! The three widget presets. Sizes live here so that the tray menu, the window
//! geometry and the React layout all agree on one definition.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WidgetSize {
    Small,
    Medium,
    Large,
}

impl WidgetSize {
    pub const ALL: [WidgetSize; 3] = [WidgetSize::Small, WidgetSize::Medium, WidgetSize::Large];

    /// Logical points, mirroring the proportions of native macOS widgets:
    /// a square small tile, a roughly 2:1 medium, and a wider large.
    pub fn dimensions(self) -> (f64, f64) {
        match self {
            WidgetSize::Small => (240.0, 240.0),
            WidgetSize::Medium => (400.0, 190.0),
            WidgetSize::Large => (540.0, 300.0),
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            WidgetSize::Small => "small",
            WidgetSize::Medium => "medium",
            WidgetSize::Large => "large",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            WidgetSize::Small => "Small",
            WidgetSize::Medium => "Medium",
            WidgetSize::Large => "Large",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|size| size.id() == id)
    }
}
