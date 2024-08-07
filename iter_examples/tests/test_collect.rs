use std;

/// 测试将字符串数组转换为整数数组
#[test]
fn test_parse_to_int() {
    let strings = vec!["1", "2", "3"];

    let numbers: Result<Vec<i32>, std::num::ParseIntError> =
        strings.iter().map(|s| s.parse::<i32>()).collect();

    match numbers {
        Ok(numbers) => assert_eq!(numbers, [1, 2, 3]),
        Err(err) => println!("字符串转换数字失败 {}", err),
    }
}
