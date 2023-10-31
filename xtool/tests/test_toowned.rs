#[test]
fn test_string_borrow() {
    let s = "hello";
    let t: String = s.to_owned();
    assert_eq!(t, s.to_string());

    let s = "world";
    let t: String = s.to_owned();
    assert_eq!(t, s.to_string());
}

/// 使用借来的数据替换拥有的数据，通常是通过克隆。
/// 这是 Clone::clone_from 的 borrow-generalized 版本。
#[test]
fn test_clone_into() {
    let mut s: String = String::new();
    "hello".clone_into(&mut s);

    let mut v: Vec<i32> = Vec::new();
    [1, 2][..].clone_into(&mut v);
}
