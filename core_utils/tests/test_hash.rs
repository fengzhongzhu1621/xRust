use core_utils::hash;

#[test]
fn test_hash_u8() {
    let value = b"123456";
    let actual = hash::hash_u8(value);
    let expect = "6c29cbb19c07d36d".to_string();
    assert_eq!(actual, expect);
}
