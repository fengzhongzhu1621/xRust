use core_utils::path::ToStfu8;
use core_utils::path::{
    PathDir, PathFile, PathInfo, PathMut, PathOps, PathSer, PathType,
};
use serde_json;
use std::path::Path;

#[cfg(any(target_os = "wasi", unix))]
static SERIALIZED: &str = "[\
                           {\"type\":\"file\",\"path\":\"{0}/foo.txt\"},\
                           {\"type\":\"dir\",\"path\":\"{0}/bar\"},\
                           {\"type\":\"dir\",\"path\":\"{0}/foo/bar\"}\
                           ]";

#[cfg(windows)]
static SERIALIZED: &str = "[\
                           {\"type\":\"file\",\"path\":\"{0}\\\\foo.txt\"},\
                           {\"type\":\"dir\",\"path\":\"{0}\\\\bar\"},\
                           {\"type\":\"dir\",\"path\":\"{0}\\\\foo\\\\bar\"}\
                           ]";

#[test]
fn sanity_serde() {
    use serde_json;
    use tempfile::TempDir;

    let tmp_dir = TempDir::new().expect("create temp dir");
    let tmp_abs = PathDir::new(tmp_dir.path()).expect("tmp_abs");

    let _ser_from_str = PathSer::from("example");
    let _ser_from_tmp_abs = PathSer::from(tmp_abs.as_path());

    let foo =
        PathFile::create(tmp_abs.concat("foo.txt").unwrap()).expect("foo.txt");
    let bar_dir =
        PathDir::create(tmp_abs.concat("bar").unwrap()).expect("bar");
    let foo_bar_dir = PathDir::create_all(
        tmp_abs.concat("foo").unwrap().concat("bar").unwrap(),
    )
    .expect("foo/bar");

    let expected = vec![
        PathType::File(foo),
        PathType::Dir(bar_dir),
        PathType::Dir(foo_bar_dir),
    ];

    let expected_str = SERIALIZED
        .replace("{0}", &tmp_abs.to_stfu8())
        // JSON needs backslashes escaped. Be careful not to invoke BA'AL:
        // https://xkcd.com/1638/)
        .replace(r"\", r"\\");

    println!("### EXPECTED:\n{}", expected_str);
    let result_str = serde_json::to_string(&expected).unwrap();
    println!("### RESULT:\n{}", result_str);
    assert_eq!(expected_str, result_str);

    let result: Vec<PathType> = serde_json::from_str(&result_str).unwrap();
    assert_eq!(expected, result);
}

#[test]
/// Just test that it has all the methods.
fn sanity_ser() {
    let mut path = PathSer::from("example/path");
    assert_eq!(
        path.join("joined").as_path(),
        Path::new("example/path/joined")
    );
    assert_eq!(path.is_absolute(), false);

    path.append("appended").unwrap();
    assert_eq!(path.as_path(), Path::new("example/path/appended"));
    path.pop_up().unwrap();
    assert_eq!(path.as_path(), Path::new("example/path"));

    assert_eq!(
        path.concat("/concated").unwrap().as_path(),
        Path::new("example/path/concated")
    );
}
