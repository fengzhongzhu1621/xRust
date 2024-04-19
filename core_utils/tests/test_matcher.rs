use core_utils::matcher::*;
use pretty_assertions::assert_eq;

#[test]
fn test_trimmed() {
    let data = [
        ("", Some("")),
        ("test", Some("test")),
        (" 		 test", Some("test")),
        ("test  	   ", Some("test")),
        ("  test 	", Some("test")),
    ];

    // 用于去掉空白字符
    let matcher = Matcher::AllTrimmed;

    for (input, expected) in &data {
        let result = matcher.find(input);
        // Option<String> -> Option<&str>
        assert_eq!(result.as_deref(), *expected);
    }
}

#[test]
fn test_prefixed_word() {
    let data = [
        ("", None),
        ("test", Some("")),
        ("test1", Some("1")),
        ("test 1", Some("1")),
        (" test 1", Some("1")),
        ("test 1.2.3", Some("1.2.3")),
        (" 		test 1.2.3", Some("1.2.3")),
    ];

    // 查找前缀匹配的第一个单词
    let matcher = Matcher::PrefixedWord { prefix: "test" };

    for (input, expected) in &data {
        let result = matcher.find(input);
        assert_eq!(result.as_deref(), *expected);
    }
}

#[test]
fn test_prefixed_version() {
    let data = [
        ("", None),
        ("test", Some("")),
        ("test 1", Some("1")),
        ("test .1", None),
        ("test 1.", None),
        ("test .1.", None),
        (" test 1", Some("1")),
        ("test 1.2.3", Some("1.2.3")),
        (" 		test 1.2.3", Some("1.2.3")),
    ];

    // 查找前缀匹配的第一个单词（是版本字符串）
    let matcher = Matcher::PrefixedVersion { prefix: "test" };

    for (input, expected) in &data {
        let result = matcher.find(input);
        assert_eq!(result.as_deref(), *expected);
    }
}

#[test]
fn test_key_value() {
    let data = [
        ("", None),
        ("key", None),
        ("key=value", Some("value")),
        ("key=1", Some("1")),
        ("key=\"1\"", Some("1")),
        ("key=\"CentOS Linux\"", Some("CentOS Linux")),
    ];

    let matcher = Matcher::KeyValue { key: "key" };

    for (input, expected) in &data {
        let result = matcher.find(input);
        assert_eq!(result.as_deref(), *expected);
    }
}
