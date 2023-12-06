use chrono::{DateTime, Local};
use std::env;
use std::ffi::OsStr;
use std::fs;

#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[test]
fn test_read_dir() {
    let dir = fs::read_dir(".").unwrap();

    for entry in dir {
        let entry = entry.unwrap();

        let path = entry.path();
        let file_type = entry.file_type().unwrap();
        let metadata = entry.metadata().unwrap();
        let file_name = entry.file_name();

        println!("{:?}|{:?}|{:?}|{:?}", path, file_type, metadata, file_name);
    }

    let new_path = Path::new("/usr/path.txt");
    println!("{:?}", new_path);

    for component in new_path.components() {
        println!("comp {:?}", component);
    }
}

#[test]
fn test_path_new() {
    let string = String::from("foo.txt");
    let from_string = Path::new(&string);
    let from_path = Path::new(&from_string);
    assert_eq!(from_string, from_path);
}

#[test]
fn test_path_display() {
    // 从 `&'static str` 创建一个 `Path`
    let path = Path::new("/usr/path.txt");

    // `display` 方法返回一个可显示（showable）的结构体
    let display = path.display();

    // 不能直接比较
    // assert_eq!(display, "/usr/path.txt");
    println!("{}", display);
    println!("{:?}", display);
}

#[test]
fn test_parent() {
    // Note: 这个例子可以在 Windows 上使用
    let path = Path::new("./foo/bar.txt");

    let parent = path.parent();
    assert_eq!(parent, Some(Path::new("./foo")));
}

#[test]
fn test_file_stem() {
    // Note: 这个例子可以在 Windows 上使用
    let path = Path::new("./foo/bar.txt");
    let file_stem = path.file_stem();
    assert_eq!(file_stem, Some(OsStr::new("bar")));

    let path = Path::new("./foo/bar");
    let file_stem = path.file_stem();
    assert_eq!(file_stem, Some(OsStr::new("bar")));

    let path = Path::new("./");
    let file_stem = path.file_stem();
    assert_eq!(file_stem, None);

    let path = Path::new(r"C:\");
    let file_stem = path.file_stem();
    assert_eq!(file_stem, None);
}

#[test]
fn test_file_name() {
    let path = Path::new("./foo/bar.txt");
    let file_stem = path.file_name();
    assert_eq!(file_stem, Some(OsStr::new("bar.txt")));

    let path = Path::new("./foo/bar");
    let file_stem = path.file_name();
    assert_eq!(file_stem, Some(OsStr::new("bar")));

    let path = Path::new("./");
    let file_stem = path.file_name();
    assert_eq!(file_stem, None);

    let path = Path::new(r"C:\");
    let file_stem = path.file_name();
    assert_eq!(file_stem, None);
}

#[test]
fn test_extension() {
    // Note: 这个例子可以在 Windows 上使用
    let path = Path::new("./foo/bar.txt");
    let extension = path.extension();
    assert_eq!(extension, Some(OsStr::new("txt")));

    let path = Path::new("./foo/bar");
    let extension = path.extension();
    assert_eq!(extension, None);
}

#[test]
/// 如果 Path 是有效的 unicode，则产生 &str 切片。
/// 此转换可能需要检查 UTF-8 有效性。 请注意，执行验证是因为非 UTF-8 字符串对于某些 OS 完全有效。
fn test_to_str() {
    let path = Path::new("foo.txt");
    assert_eq!(path.to_str(), Some("foo.txt"));
}

#[test]
fn test_to_path_buf() {
    let path_buf = Path::new("foo.txt").to_path_buf();
    assert_eq!(path_buf, std::path::PathBuf::from("foo.txt"));
}

#[test]
fn test_creation_time() {
    let source_path = Path::new("Cargo.toml");
    let path = env::current_dir().unwrap().join(source_path);
    let absolute_path = path.as_path();

    // 获得文件创建时间
    if let Ok(metadata) = absolute_path.metadata() {
        let creation_time = metadata.created().unwrap();
        let local_datetime: DateTime<Local> = creation_time.clone().into();
        let datetime_str = local_datetime.format(DATETIME_FORMAT).to_string();

        println!("{}", datetime_str);
    }
}

#[test]
fn test_path_push() {
    let mut path = PathBuf::from("./foo/");
    path.push("a");
    path.push("b");
    assert_eq!(path, PathBuf::from(r"./foo/a\\b"));
}

#[test]
fn test_str_to_asref_path() {
    // impl AsRef<Path> for str {
    //     #[inline]
    //     fn as_ref(&self) -> &Path {
    //         Path::new(self)
    //     }
    // }
    // pub fn new<S: AsRef<OsStr> + ?Sized>(s: &S) -> &Path {
    //     unsafe { &*(s.as_ref() as *const OsStr as *const Path) }
    // }
    let s = "/a/b/c";
    let path = unsafe { &*(s.as_ref() as *const OsStr as *const Path) };
    assert_eq!(path, Path::new(s));
}
