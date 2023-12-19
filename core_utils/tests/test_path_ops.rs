use core_utils::path::PathOps;
use std::ffi;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

#[test]
fn test_pathops_concat() {
    let actual = Path::new("foo")
        .concat(Path::new("bar"))
        .expect("Could not concat paths?");
    let expected = PathBuf::from("foo/bar");

    assert_eq!(actual, expected);

    let actual = Path::new("foo")
        .concat(Path::new("bar/../baz"))
        .expect("Could not concat path with ..?");
    let expected = PathBuf::from("foo/baz");

    assert_eq!(actual, expected);

    let actual =
        Path::new("foo").concat("..").expect("Could not cancel path with ..?");
    let expected = PathBuf::from(r"");

    assert_eq!(actual, expected);

    let actual =
        Path::new("foo").concat("../..").expect("Could not prefix with ..?");
    let expected = PathBuf::from(r"../");
    assert_eq!(actual, expected);

    let actual = Path::new("/foo")
        .concat("../..")
        .expect_err("Could escape root with ..?");

    assert_eq!(actual.io_error().kind(), ErrorKind::NotFound);
    assert_eq!(actual.action(), "truncating to parent");
    assert_eq!(actual.path(), Path::new("/"));

    let actual = Path::new("foo")
        .concat(Path::new("/etc/passwd"))
        .expect("Could not concat RootDir to path?");
    let expected: PathBuf = PathBuf::from("foo/etc/passwd");

    assert_eq!(actual, expected);
}

#[test]
fn test_pathops_concat_relative() {
    let actual = Path::new("../foo")
        .concat("bar")
        .expect("Could not create relative path with concat");
    let expected = PathBuf::from(r"../foo/bar");
    assert_eq!(actual, expected);

    let actual = Path::new("../foo")
        .concat("..")
        .expect("Could not create relative path with concat");
    let expected = PathBuf::from(r"..");
    assert_eq!(actual, expected);

    let actual = Path::new("../foo")
        .concat("../..")
        .expect("Could not create relative path with concat");
    let expected = PathBuf::from(r"../..");
    assert_eq!(actual, expected);

    let actual = Path::new("../foo/../bar")
        .concat("../..")
        .expect("Could not create relative path with concat");
    let expected = PathBuf::from(r"../..");
    assert_eq!(actual, expected);

    let actual = Path::new("../foo/../bar/..")
        .concat("../..")
        .expect("Could not create relative path with concat");
    let expected = PathBuf::from(r"../../..");
    assert_eq!(actual, expected);

    let actual = PathBuf::from("../foo/..")
        .concat("../../baz")
        .expect("Could not create relative path with concat");
    let expected = PathBuf::from(r"../../../baz");
    assert_eq!(actual, expected);
}

#[test]
fn test_pathops_concat_cur() {
    // just check that pahts don't normalize...
    let actual = Path::new("foo/././..").as_os_str();
    let expected = ffi::OsStr::new("foo/././..");
    assert_eq!(actual, expected);

    let actual = PathBuf::from("././foo/././..")
        .concat("../bar")
        .expect("Could not create relative path with concat");
    let expected = PathBuf::from(r"../bar");
    assert_eq!(actual, expected);
}

#[test]
fn test_pathops_concat_consume() {
    let actual = Path::new("foo")
        .concat("../../bar")
        .expect("Could not create relative path with concat");
    let expected = PathBuf::from(r"../bar");
    assert_eq!(actual, expected);
}
