use core_utils::path::{PathAbs, PathDir, PathFile, PathOps, PathType};
use std::collections::HashSet;
use tempfile::TempDir;

#[test]
fn sanity_list() {
    let tmp_dir = TempDir::new().expect("create temp dir");
    let tmp_abs = PathDir::new(tmp_dir.path()).unwrap();

    let foo_path = tmp_abs.concat("foo").expect("path foo");
    let foo_dir = PathDir::create(foo_path).unwrap();

    let bar_path = tmp_abs.concat("bar").expect("path bar");
    let bar_file = PathFile::create(bar_path).unwrap();

    let mut result = HashSet::new();
    for p in tmp_abs.list().unwrap() {
        result.insert(p.unwrap());
    }

    let mut expected = HashSet::new();
    expected.insert(PathType::Dir(foo_dir.clone()));
    expected.insert(PathType::File(bar_file.clone()));

    assert_eq!(expected, result);

    // just ensure that this compiles
    let _: PathAbs = foo_dir.into();
    let _: PathAbs = bar_file.into();
}
