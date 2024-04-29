use core_utils::predicates::{self, core::Predicate};
use std::path::Path;

#[test]
fn test_predicate_fs() {
    let predicate_file =
        predicates::path::eq_file(Path::new("Cargo.toml")).utf8().unwrap();
    assert_eq!(true, predicate_file.eval(Path::new("Cargo.toml")));
    assert_eq!(false, predicate_file.eval(Path::new("Cargo.lock")));
    assert_eq!(false, predicate_file.eval(Path::new("src")));

    assert_eq!(
        false,
        predicate_file.eval("Not a real Cargo.toml file content")
    );
}
