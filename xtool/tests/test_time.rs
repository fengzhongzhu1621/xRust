use chrono::prelude::*;
use std::time::{Duration, SystemTime};

pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[test]
fn test_timestamp_to_systemtime() {
    let timestamp = 1623731234;
    let system_time = SystemTime::UNIX_EPOCH + Duration::from_secs(timestamp);
    println!("System time from timestamp: {:?}", system_time);
}

#[test]
fn test_from_local_datetime() {
    let no_timezone =
        NaiveDateTime::parse_from_str("2023-01-01 00:00:00", DATETIME_FORMAT)
            .unwrap();
    let _now: SystemTime =
        Local.from_local_datetime(&no_timezone).unwrap().into();
}
