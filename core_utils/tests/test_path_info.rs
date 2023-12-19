use core_utils::path::PathInfo;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

#[test]
fn test_pathinfo_is_absolute() {
    let p = PathBuf::from("/foo/bar");

    let expected = !cfg!(windows);
    assert_eq!(<PathBuf as PathInfo>::is_absolute(&p), expected);
}

#[test]
fn test_pathinfo_parent() {
    let p = PathBuf::from("/foo/bar");

    let actual =
        <PathBuf as PathInfo>::parent(&p).expect("could not find parent?");
    let expected = PathBuf::from("/foo");
    assert_eq!(actual, expected);

    let p = PathBuf::from("/");
    let actual =
        <PathBuf as PathInfo>::parent(&p).expect_err("root has a parent?");

    assert_eq!(actual.io_error().kind(), ErrorKind::NotFound);
    assert_eq!(actual.action(), "truncating to parent");
    assert_eq!(actual.path(), Path::new("/"));
}

#[test]
fn test_pathinfo_starts_with() {
    let p = PathBuf::from("foo/bar");

    assert_eq!(<PathBuf as PathInfo>::starts_with(&p, Path::new("foo")), true,);
    assert_eq!(
        <PathBuf as PathInfo>::starts_with(&p, Path::new("bar")),
        false,
    );
}

#[test]
fn test_pathinfo_ends_with() {
    let p = PathBuf::from("foo/bar");

    assert_eq!(<PathBuf as PathInfo>::ends_with(&p, Path::new("foo")), false,);
    assert_eq!(<PathBuf as PathInfo>::ends_with(&p, Path::new("bar")), true,);
}
