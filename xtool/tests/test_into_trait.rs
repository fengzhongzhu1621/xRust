#[test]
fn test_into_type() {
    // Into<Vec<u8>> 表示可以接收任意一个能够转换为Vec<u8>的类型
    fn is_hello<T: Into<Vec<u8>>>(s: T) {
        let bytes = b"hello".to_vec();
        assert_eq!(bytes, s.into());
    }

    let s = "hello".to_string();
    is_hello(s);
}
