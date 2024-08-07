/// 测试无限循环迭代器
#[test]
fn test_natural_numbers() {
    // 定义一个无限循环迭代器
    let natural_numbers = 0..;

    for (i, n) in natural_numbers.take(10).enumerate() {
        println!("{}: {}", i, n);
    }
}
