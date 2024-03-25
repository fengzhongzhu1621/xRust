use core_utils::path::PathMut;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

#[test]
fn test_pathmut_append() {
    let mut actual = PathBuf::from("foo");
    actual.append(Path::new("bar")).expect("Could not append paths?");
    let expected = PathBuf::from("foo/bar");
    assert_eq!(actual, expected);

    let mut actual = PathBuf::from("foo");
    actual
        .append(Path::new("bar/../baz"))
        .expect("Could not append path with ..?");
    let expected = PathBuf::from("foo/baz");
    assert_eq!(actual, expected);

    let mut actual = PathBuf::from("foo");
    actual.append("..").expect("Could not cancel path with ..?");
    let expected = PathBuf::from(r"");
    assert_eq!(actual, expected);

    let mut actual = PathBuf::from("foo");
    actual.append("../..").expect("Could not escape prefix with ..?");
    let expected = PathBuf::from("../");
    assert_eq!(actual, expected);

    let actual = PathBuf::from("/foo")
        .append("../..")
        .expect_err("Could escape root with ..?");
    assert_eq!(actual.io_error().kind(), ErrorKind::NotFound);
    assert_eq!(actual.action(), "truncating to parent");
    assert_eq!(actual.path(), Path::new("/"));

    let mut actual = PathBuf::from("foo");
    actual
        .append(Path::new("/etc/passwd"))
        .expect("Could not append RootDir to path?");
    let expected: PathBuf = PathBuf::from("foo/etc/passwd");

    assert_eq!(actual, expected);
}

#[test]
fn test_pathmut_pop_up() {
    let mut p = PathBuf::from("/foo/bar");
    p.pop_up().expect("could not find parent?");

    assert_eq!(p.as_path(), Path::new("/foo"));

    let mut p = PathBuf::from("/");
    let actual = p.pop_up().expect_err("root has a parent?");

    assert_eq!(actual.io_error().kind(), ErrorKind::NotFound);
    assert_eq!(actual.action(), "truncating to parent");
    assert_eq!(actual.path(), Path::new("/"));
}

#[test]
fn test_pathmut_truncate_to_root() {
    let mut p = PathBuf::from("/foo/bar");
    p.truncate_to_root();
    assert_eq!(p.as_path(), Path::new("/"));

    let mut p = PathBuf::from("foo/bar");
    p.truncate_to_root();
    assert_eq!(p.as_path(), Path::new(""));
}

#[cfg(windows)]
#[test]
fn _test_pathmut_truncate_to_root() {
    let mut p = PathBuf::from(r"C:\foo\bar");
    p.truncate_to_root();
    assert_eq!(p.as_path(), Path::new(r"C:\"));

    let mut p = PathBuf::from(r"C:foo");
    p.truncate_to_root();
    assert_eq!(p.as_path(), Path::new(r"C:"));

    let mut p = PathBuf::from(r"\foo");
    p.truncate_to_root();
    assert_eq!(p.as_path(), Path::new(r"\"));

    let mut p = PathBuf::from(r"foo");
    p.truncate_to_root();
    assert_eq!(p.as_path(), Path::new(r""));
}
