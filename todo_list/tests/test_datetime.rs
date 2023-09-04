use todo_list::datetime;

#[test]
fn test_format_datetime() {
    let now = datetime::format_datetime();
    let actual = now.len();
    let expect = 19;

    println!("test_format_datetime actual is {}", now);
    assert_eq!(actual, expect);
}

#[test]
fn test_format_date() {
    let now = datetime::format_date();
    let actual = now.len();
    let expect = 10;

    println!("test_format_date actual is {}", now);
    assert_eq!(actual, expect);
}

#[test]
fn test_get_weekday_index() {
    let now_str = "2023-01-08";
    let actual = datetime::get_weekday_index(&now_str);
    let expect = 2;
    assert_eq!(actual, expect);
}
