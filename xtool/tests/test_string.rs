#[test]
fn test_deref() {
    let s = "hello".to_string();
    let t = &*s;
    assert_eq!(t, "hello");
}