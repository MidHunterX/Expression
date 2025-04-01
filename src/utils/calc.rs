// NOTE: Using u64 because thread::sleep only accepts u64

use chrono::{DateTime, Local, Timelike};

/// Returns time to wait until next wallpaper refresh time
///
/// # Examples
///
/// ```rust
/// use chrono::{Local, TimeZone};
/// use expression::utils::calc::wait_time;
///
/// let interval = 60; // 1 hour
/// let now = Local.with_ymd_and_hms(2025, 3, 31, 14, 45, 0).unwrap();
///
/// let wait_seconds = wait_time(interval, now);
///
/// assert_eq!(wait_seconds, 900); // 15 minutes
/// ```
pub fn wait_time(interval: u32, now: DateTime<Local>) -> u64 {
    // let secs = now.second();
    // let mins = now.minute();
    // let remaining_seconds = 60 - secs;
    // let remaining_minutes = mins % interval;
    // let wait_minutes = interval - remaining_minutes - 1;
    // ((wait_minutes * 60) + remaining_seconds) as u64

    (((interval - (now.minute() % interval) - 1) * 60) + (60 - now.second())) as u64
}

/// Re-calculates refresh time.
/// Returns `(is_hour_changed, new_wait_seconds)`.
///
/// # Examples
///
/// ```rust
/// use chrono::{DateTime, Local, TimeZone};
/// use expression::utils::calc::refresh_time;
///
/// let interval = 60; // 1 hour
/// let old_now = Local.with_ymd_and_hms(2025, 3, 31, 14, 40, 0).unwrap();
/// let new_now = Local.with_ymd_and_hms(2025, 3, 31, 14, 45, 0).unwrap();
///
/// let (is_hour_changed, wait_seconds) = refresh_time(interval, old_now, new_now);
///
/// assert_eq!(is_hour_changed, false);
/// assert_eq!(wait_seconds, 900); // 15 minutes
/// ```
pub fn refresh_time(
    interval: u32,
    old_now: DateTime<Local>,
    new_now: DateTime<Local>,
) -> (bool, u64) {
    let mut is_hour_changed = false;

    // Edge Case: Time Travel (old_now > new_now)
    // It's ok, we cool with Time Travellers

    // Edge Case: Hour Change
    if new_now.hour() != old_now.hour() {
        is_hour_changed = true;
        return (is_hour_changed, 0);
    }

    // Re-calculate refresh time
    let new_wait_seconds: u64 = wait_time(interval, new_now);

    return (is_hour_changed, new_wait_seconds);
}

// █▀█ █▀▀ █▀▀ █▀█ █▀▀ █▀ █░█
// █▀▄ ██▄ █▀░ █▀▄ ██▄ ▄█ █▀█

use colored::Colorize;
use log::debug;
use std::{thread, time::Duration};

/// Refresh: T Strategy
/// Simply waits until next wallpaper refresh time
pub fn sleep(wait_seconds: u64) {
    thread::sleep(Duration::from_secs(wait_seconds));
}

/// Refresh: T/2 Strategy
/// Re-calculates refresh time every T/2 seconds
/// Mitigates the Sleep/Hibernate issue to an extent without much wakeup calls
/// Time : Max Polling Rate [log2(refresh_seconds)]
/// 1m  : 6     |    1h  : 12
/// 2m  : 7     |    2h  : 13
/// 4m  : 8     |    4h  : 14
/// 8m  : 9     |    8h  : 15
/// 16m : 10    |    16h : 16
/// 32m : 11    |    32h : 17
pub fn refresh_t2(interval: u32, now: DateTime<Local>, wait_seconds: u64) {
    let mut ori_refresh_seconds = wait_seconds;
    let mut refresh_seconds = wait_seconds;

    while refresh_seconds > 1 {
        refresh_seconds /= 2;

        debug!(
            "Recalculating in {}...",
            if refresh_seconds > 60 {
                format!(
                    "{} {}",
                    format!("{}m", refresh_seconds / 60).cyan(),
                    format!("{}s", refresh_seconds % 60).cyan()
                )
            } else {
                format!("{}s", format!("{}", refresh_seconds).cyan())
            }
        );

        sleep(refresh_seconds);

        // Re-calculate refresh time
        let new_now = Local::now();
        let (is_hour_changed, new_wait_seconds) = refresh_time(interval, now, new_now);

        if is_hour_changed {
            debug!("Hour Changed: {}", new_now.hour());
            break;
        }

        if new_wait_seconds < ori_refresh_seconds {
            refresh_seconds = new_wait_seconds;
            ori_refresh_seconds = new_wait_seconds;
        }
    }
    sleep(1);
}
