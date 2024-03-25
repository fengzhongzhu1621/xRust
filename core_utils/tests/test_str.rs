use core_utils::str;

#[test]
fn test_chunks() {
    let s = "你好, hello！".to_string();
    let actual = str::chunks(s, 2);
    let expect: Vec<String> = vec![
        "你好".to_string(),
        ", ".to_string(),
        "he".to_string(),
        "ll".to_string(),
        "o！".to_string(),
    ];
    assert_eq!(actual, expect);
}
