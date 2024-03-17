use core_utils::file;
use std::env;

#[test]
fn test_md5() {
    let path = env::current_dir().unwrap().join("tests/data.txt");

    let actual = file::md5(path).unwrap();
    let expect = "0401d7b371d25d5999e456d4cc8366ac".to_string();
    assert_eq!(actual, expect);
}

#[test]
fn test_sha1() {
    let path = env::current_dir().unwrap().join("tests/data.txt");

    let actual = file::sha1(path).unwrap();
    let expect = "09b97787f67e6470945da3db502bf12c0012ff5c".to_string();
    assert_eq!(actual, expect);
}
