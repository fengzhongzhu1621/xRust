use chrono::prelude::*;
use std::time::SystemTime;

const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// 将当前时间转换为UTC时区的字符串格式
pub fn format_datetime() -> String {
    let now = Utc::now();
    return now.format("%Y-%m-%d %H:%M:%S").to_string();
}

pub fn format_date() -> String {
    let now = Utc::now();
    return now.format("%Y-%m-%d").to_string();
}

/// 将 SystemTime 转换为字符串格式
pub fn format_system_time(st: SystemTime) -> String {
    // 获得本机时间
    let local_datetime: DateTime<Local> = st.clone().into();
    // 将本机时间格式化为字符串
    local_datetime.format(DATETIME_FORMAT).to_string()
}

/// 根据日期字符串获得是第几个星期
pub fn get_weekday_index(date: &str) -> u32 {
    // 将日期字符串转换为日期对象
    let now = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    // 获取天索引
    let day_index = now.ordinal();

    return (day_index / 7) + 1;
}

/// 将 SystemTime 转换为UNIX时间戳的秒表示
pub fn to_seconds(st: SystemTime) -> i64 {
    let local_datetime: DateTime<Local> = st.clone().into();
    local_datetime.timestamp()
}

/// 将 SystemTime 转换为UNIX时间戳的毫秒表示
pub fn to_mill_seconds(st: SystemTime) -> i64 {
    let local_datetime: DateTime<Local> = st.clone().into();
    local_datetime.timestamp_millis()
}

/// 获得当前时间戳
pub fn now_to_seconds() -> i64 {
    let now = Local::now();
    now.timestamp()
}

/// 将当前时区的时间字符串转换为 SystenTime
pub fn to_system_time(datetime_str: &str) -> SystemTime {
    let no_timezone =
        NaiveDateTime::parse_from_str(datetime_str, DATETIME_FORMAT).unwrap();
    Local.from_local_datetime(&no_timezone).unwrap().into()
}
