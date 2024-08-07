/// 测试 vec 迭代需要添加 iter()方法#[test]
#[test]
fn test_vec_iter() {
    let numbers = vec![1, 2, 3, 4, 5];
    let iter = numbers.iter();

    for &number in iter {
        println!("{}", number);
    }
}
