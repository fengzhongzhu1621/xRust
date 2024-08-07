/// 测试使用 sum 对迭代器求和
#[test]
fn test_sum() {
    let sum: u32 = (1..=100).filter(|&x| x % 2 == 0).map(|x| x * x).sum();
    assert_eq!(sum, 171700);
}
