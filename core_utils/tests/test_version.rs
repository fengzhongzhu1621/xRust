use core_utils::version::*;
use pretty_assertions::assert_eq;

#[test]
fn test_parse_semantic_version() {
    let data = [
        ("", None),
        ("version", None),
        ("1", Some((1, 0, 0))),
        ("1.", Some((1, 0, 0))),
        ("1.2", Some((1, 2, 0))),
        ("1.2.", Some((1, 2, 0))),
        ("1.2.3", Some((1, 2, 3))),
        ("1.2.3.", Some((1, 2, 3))),
        ("1.2.3.  ", Some((1, 2, 3))),
        ("   1.2.3.", Some((1, 2, 3))),
        ("   1.2.3.  ", Some((1, 2, 3))),
        ("1.2.3.4", None),
        ("1.2.3.4.5.6.7.8.9", None),
    ];

    for (s, expected) in &data {
        let result = parse_version(s);
        assert_eq!(expected, &result);
    }
}

#[test]
fn from_string() {
    let custom_version = "some version";
    let data = [
        ("", Version::Unknown),
        ("1.2.3", Version::Semantic(1, 2, 3)),
        (custom_version, Version::Custom(custom_version.to_owned())),
    ];

    for (s, expected) in &data {
        let version = Version::from_string(*s);
        assert_eq!(expected, &version);
    }
}

#[test]
fn default() {
    assert_eq!(Version::Unknown, Version::default());
}

#[test]
fn display() {
    let data = [
        (Version::Unknown, "Unknown"),
        (Version::Semantic(1, 5, 0), "1.5.0"),
        (Version::Rolling(None), "Rolling Release"),
        (
            Version::Rolling(Some("date".to_owned())),
            "Rolling Release (date)",
        ),
    ];

    for (version, expected) in &data {
        assert_eq!(expected, &version.to_string());
    }
}