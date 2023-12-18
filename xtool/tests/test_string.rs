#[test]
fn test_deref() {
    let s = "hello".to_string();
    let t = &*s;
    assert_eq!(t, "hello");
}

#[test]
fn test_split() {
    let source = String::from("a,b,c");
    let values: Vec<_> = source.split(',').collect::<Vec<&str>>();
    assert_eq!(values, vec!["a", "b", "c"]);
}
