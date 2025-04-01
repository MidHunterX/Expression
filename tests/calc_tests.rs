use chrono::{Local, TimeZone};
use expression::utils::calc::{refresh_time, wait_time};

// █░█░█ ▄▀█ █ ▀█▀   ▀█▀ █ █▀▄▀█ █▀▀
// ▀▄▀▄▀ █▀█ █ ░█░   ░█░ █ █░▀░█ ██▄

#[test]
fn test_wait_time_exact_interval_start() {
    let interval = 60; // 1 hour
    let now = Local.with_ymd_and_hms(2025, 3, 31, 14, 0, 0).unwrap();

    let wait_seconds = wait_time(interval, now);

    assert_eq!(wait_seconds, 3600); // Full hour left
}

#[test]
fn test_wait_time_halfway_through_interval() {
    let interval = 60;
    let now = Local.with_ymd_and_hms(2025, 3, 31, 14, 30, 0).unwrap();

    let wait_seconds = wait_time(interval, now);

    assert_eq!(wait_seconds, 1800); // 30 minutes left
}

#[test]
fn test_wait_time_one_minute_before_next_interval() {
    let interval = 60;
    let now = Local.with_ymd_and_hms(2025, 3, 31, 14, 59, 0).unwrap();

    let wait_seconds = wait_time(interval, now);

    assert_eq!(wait_seconds, 60); // 1 minute left
}

#[test]
fn test_wait_time_one_second_before_next_interval() {
    let interval = 60;
    let now = Local.with_ymd_and_hms(2025, 3, 31, 14, 59, 59).unwrap();

    let wait_seconds = wait_time(interval, now);

    assert_eq!(wait_seconds, 1); // Only 1 second left
}

#[test]
fn test_wait_time_random_case() {
    let interval = 60;
    let now = Local.with_ymd_and_hms(2025, 3, 31, 14, 45, 20).unwrap();

    let wait_seconds = wait_time(interval, now);

    assert_eq!(wait_seconds, 880); // 14 minutes, 40 seconds left
}

#[test]
fn test_wait_time_interval_not_60() {
    let interval = 30; // Every 30 minutes
    let now = Local.with_ymd_and_hms(2025, 3, 31, 14, 15, 0).unwrap();

    let wait_seconds = wait_time(interval, now);

    assert_eq!(wait_seconds, 900); // 15 minutes left (30 - 15)
}

#[test]
fn test_wait_time_exactly_at_interval_boundary() {
    let interval = 30; // Every 30 minutes
    let now = Local.with_ymd_and_hms(2025, 3, 31, 14, 30, 0).unwrap();

    let wait_seconds = wait_time(interval, now);

    assert_eq!(wait_seconds, 1800); // 30 minutes left
}

// █▀█ █▀▀ █▀▀ █▀█ █▀▀ █▀ █░█   ▀█▀ █ █▀▄▀█ █▀▀
// █▀▄ ██▄ █▀░ █▀▄ ██▄ ▄█ █▀█   ░█░ █ █░▀░█ ██▄

#[test]
fn test_refresh_time_same_hour() {
    let interval = 60; // 1 hour
    let old_now = Local.with_ymd_and_hms(2025, 3, 31, 14, 30, 0).unwrap();
    let new_now = Local.with_ymd_and_hms(2025, 3, 31, 14, 45, 0).unwrap();

    let (is_hour_changed, wait_seconds) = refresh_time(interval, old_now, new_now);

    assert_eq!(is_hour_changed, false);
    assert_eq!(wait_seconds, 900); // 15 minutes
}

#[test]
fn test_refresh_time_hour_changed() {
    let interval = 60;
    let old_now = Local.with_ymd_and_hms(2025, 3, 31, 14, 55, 0).unwrap();
    let new_now = Local.with_ymd_and_hms(2025, 3, 31, 15, 0, 5).unwrap();

    let (is_hour_changed, wait_seconds) = refresh_time(interval, old_now, new_now);

    assert_eq!(is_hour_changed, true);
    assert_eq!(wait_seconds, 0);
}

#[test]
fn test_refresh_time_midnight_change() {
    let interval = 60;
    let old_now = Local.with_ymd_and_hms(2025, 3, 31, 23, 58, 30).unwrap();
    let new_now = Local.with_ymd_and_hms(2025, 4, 1, 0, 1, 0).unwrap();

    let (is_hour_changed, wait_seconds) = refresh_time(interval, old_now, new_now);

    assert_eq!(is_hour_changed, true);
    assert_eq!(wait_seconds, 0);
}

#[test]
fn test_refresh_time_time_travel() {
    let interval = 60;
    let old_now = Local.with_ymd_and_hms(2025, 3, 31, 15, 0, 0).unwrap();
    let new_now = Local.with_ymd_and_hms(1969, 3, 31, 15, 55, 0).unwrap(); // Time travel back

    let (is_hour_changed, wait_seconds) = refresh_time(interval, old_now, new_now);

    assert_eq!(is_hour_changed, false);
    assert_eq!(wait_seconds, 300); // 5 minutes remaining
}

#[test]
fn test_refresh_time_exact_interval() {
    let interval = 60;
    let old_now = Local.with_ymd_and_hms(2025, 3, 31, 12, 0, 0).unwrap();
    let new_now = Local.with_ymd_and_hms(2025, 3, 31, 12, 30, 0).unwrap();

    let (is_hour_changed, wait_seconds) = refresh_time(interval, old_now, new_now);

    assert_eq!(is_hour_changed, false);
    assert_eq!(wait_seconds, 1800); // 30 minutes
}
