/// 返回包含的 Ok 值或从闭包计算它
#[test]
fn test_unwrap_or_else() {
    fn count(x: &str) -> usize { x.len() }

    // 如果是 Ok，则返回包含的值
    assert_eq!(Ok(2).unwrap_or_else(count), 2);
    // 如果是 Err，则返回闭包计算的值
    assert_eq!(Err("foo").unwrap_or_else(count), 3);
}