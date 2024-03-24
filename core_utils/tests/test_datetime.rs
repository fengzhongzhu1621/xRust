use core_utils::datetime::*;
use std::time::SystemTime;

#[test]
fn test_format_datetime() {
    let now = format_datetime();
    let actual = now.len();
    let expect = 19;

    println!("test_format_datetime actual is {}", now);
    assert_eq!(actual, expect);
}

#[test]
fn test_format_date() {
    let now = format_date();
    let actual = now.len();
    let expect = 10;

    println!("test_format_date actual is {}", now);
    assert_eq!(actual, expect);
}

#[test]
fn test_get_weekday_index() {
    let now_str = "2024-01-08";
    let actual = get_weekday_index(&now_str);
    let expect = 2;
    assert_eq!(actual, expect);
}

#[test]
fn test_to_timestamp() {
    let st = SystemTime::now();
    println!("{:?} {:?}", st, to_seconds(st));

    println!("{:?} {:?}", st, to_mill_seconds(st));
    println!("{:?}", now_to_seconds());
}

#[test]
fn test_to_system_time() {
    let str = "2024-01-02 03:04:05";
    let st = to_system_time(str);
    assert_eq!(to_seconds(st), 1704135845);
}
