#[test]
fn test_ref_u8() {
    let str1 = "Linux is the best operation!".as_bytes();
    assert_eq!(&str1[6..], "is the best operation!".as_bytes());
}
