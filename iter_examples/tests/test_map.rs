/// 测试实现异或操作
#[test]
fn test_xor() {
    let rand_char = 100 as u8;
    let message: [u8; 3] = [1, 2, 3];
    let actual = message.iter().map(|&x| (rand_char ^ x)).collect::<Vec<u8>>();
    let expect = [101, 102, 103];
    assert_eq!(actual, expect);
}
