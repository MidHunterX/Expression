use chrono::{DateTime, Local, Timelike};

/// Calculates the current wallpaper group index based on the current time.
///
/// This function divides a 1-hour window (3600 seconds) evenly across the total number
/// of wallpaper groups, and determines which group corresponds to the current time.
///
/// # Example
///
/// ```
/// use expression::utils::calc::get_group_index;
///
/// let now = chrono::Local::now();
/// let index = get_group_index(now, 19);
/// println!("Current group: {}", index);
/// ```
pub fn get_group_index(now: DateTime<Local>, total_groups: usize) -> usize {
    let seconds = now.minute() * 60 + now.second();
    let total_slots = 60 * 60; // number of seconds in an hour
    let interval = total_slots as f64 / total_groups as f64;
    ((seconds as f64 / interval).floor() as usize).min(total_groups - 1)
}

/// Returns time to wait until next wallpaper refresh time
///
/// # Examples
///
/// ```rust
/// use chrono::{Local, TimeZone};
/// use expression::utils::calc::wait_time;
///
/// let interval = 60.0; // 1 hour
/// let now = Local.with_ymd_and_hms(2025, 3, 31, 14, 45, 0).unwrap();
///
/// let wait_seconds = wait_time(interval, now);
///
/// assert_eq!(wait_seconds, 900); // 15 minutes
/// ```
pub fn wait_time(interval: f64, now: DateTime<Local>) -> u64 {
    let current = now.minute() * 60 + now.second();
    let next = (interval * 60.0).ceil() as u32;
    let remaining = current % next;
    let wait = next - remaining;
    wait as u64
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
/// let interval = 60.0; // 1 hour
/// let old_now = Local.with_ymd_and_hms(2025, 3, 31, 14, 40, 0).unwrap();
/// let new_now = Local.with_ymd_and_hms(2025, 3, 31, 14, 45, 0).unwrap();
///
/// let (is_hour_changed, wait_seconds) = refresh_time(interval, old_now, new_now);
///
/// assert_eq!(is_hour_changed, false);
/// assert_eq!(wait_seconds, 900); // 15 minutes
/// ```
pub fn refresh_time(
    interval: f64,
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
use log2::debug;
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
pub fn refresh_tlog2(interval: f64, start_time: DateTime<Local>, wait_seconds: u64) {
    let mut previous_wait = wait_seconds;
    let mut current_wait = wait_seconds;

    while current_wait > 1 {
        // Calculate refresh seconds
        current_wait /= 2;
        debug!(
            "Rechecking in {}...",
            if current_wait > 60 {
                format!(
                    "{} {}",
                    format!("{}m", current_wait / 60).cyan(),
                    format!("{}s", current_wait % 60).cyan()
                )
            } else {
                format!("{}s", format!("{current_wait}").cyan())
            }
        );
        sleep(current_wait);

        // Recalculate total wait seconds
        let now = Local::now();
        let (hour_changed, new_wait) = refresh_time(interval, start_time, now);

        // If the hour is changed, it probably means it's the next hour
        // So, break the wait cycle for executing the next wallpaper.
        if hour_changed {
            debug!("Hour changed: {}", now.hour());
            break;
        }

        if new_wait < previous_wait {
            current_wait = new_wait;
            previous_wait = new_wait;
        }
    }

    // Final short sleep to make sure wait time is met
    sleep(1);
}
