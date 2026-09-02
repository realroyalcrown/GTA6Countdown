//! Single source of truth for the release target and the remaining-time math.
//!
//! Release date is the officially announced Grand Theft Auto VI launch:
//! November 19, 2026. Rockstar publishes launch times as local midnight, so the
//! target is resolved in the machine's own time zone.

use chrono::{DateTime, Datelike, Local, TimeZone};
use serde::Serialize;

const RELEASE_YEAR: i32 = 2026;
const RELEASE_MONTH: u32 = 11;
const RELEASE_DAY: u32 = 19;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseTarget {
    /// Unix epoch milliseconds of the release moment, for the frontend clock.
    pub epoch_ms: i64,
    pub iso: String,
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Remaining {
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
    pub released: bool,
}

pub fn target_datetime() -> DateTime<Local> {
    Local
        .with_ymd_and_hms(RELEASE_YEAR, RELEASE_MONTH, RELEASE_DAY, 0, 0, 0)
        .single()
        // A DST transition can make local midnight ambiguous or nonexistent;
        // fall back to the first valid instant of that day.
        .unwrap_or_else(|| {
            Local
                .with_ymd_and_hms(RELEASE_YEAR, RELEASE_MONTH, RELEASE_DAY, 1, 0, 0)
                .earliest()
                .expect("release date must resolve to a valid local instant")
        })
}

pub fn release_target() -> ReleaseTarget {
    let target = target_datetime();
    ReleaseTarget {
        epoch_ms: target.timestamp_millis(),
        iso: target.to_rfc3339(),
        year: target.year(),
        month: target.month(),
        day: target.day(),
    }
}

pub fn remaining_now() -> Remaining {
    let delta = target_datetime() - Local::now();
    let total = delta.num_seconds();

    if total <= 0 {
        return Remaining {
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            released: true,
        };
    }

    Remaining {
        days: total / 86_400,
        hours: (total % 86_400) / 3_600,
        minutes: (total % 3_600) / 60,
        seconds: total % 60,
        released: false,
    }
}

/// Compact label for the macOS menu bar, e.g. `79d 14:22:05`.
pub fn menu_bar_label(remaining: Remaining) -> String {
    if remaining.released {
        return "OUT NOW".to_string();
    }

    if remaining.days > 0 {
        format!(
            "{}d {:02}:{:02}:{:02}",
            remaining.days, remaining.hours, remaining.minutes, remaining.seconds
        )
    } else {
        format!(
            "{:02}:{:02}:{:02}",
            remaining.hours, remaining.minutes, remaining.seconds
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_is_the_announced_release_day() {
        let target = target_datetime();
        assert_eq!(target.year(), 2026);
        assert_eq!(target.month(), 11);
        assert_eq!(target.day(), 19);
    }

    #[test]
    fn label_omits_days_on_the_final_day() {
        let remaining = Remaining {
            days: 0,
            hours: 4,
            minutes: 7,
            seconds: 9,
            released: false,
        };
        assert_eq!(menu_bar_label(remaining), "04:07:09");
    }

    #[test]
    fn label_includes_days_when_present() {
        let remaining = Remaining {
            days: 79,
            hours: 14,
            minutes: 22,
            seconds: 5,
            released: false,
        };
        assert_eq!(menu_bar_label(remaining), "79d 14:22:05");
    }
}
