use chrono::prelude::*;

pub fn format_datetime() -> String {
    let now = Utc::now();
    return now.format("%Y-%m-%d %H:%M:%S").to_string();
}

pub fn format_date() -> String {
    let now = Utc::now();
    return now.format("%Y-%m-%d").to_string();
}

pub fn get_weekday_index(date: &str) -> u32 {
    // 将日期字符串转换为日期对象
    let now = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    // 获取天索引
    let day_index = now.ordinal();

    return (day_index / 7) + 1;
}
