use core_utils::predicates::{self, core::Predicate};
use std::path::Path;

#[test]
fn test_predicate_is_file() {
    let predicate_fn = predicates::path::is_file();
    assert_eq!(true, predicate_fn.eval(Path::new("Cargo.toml")));
    assert_eq!(false, predicate_fn.eval(Path::new("src")));
    assert_eq!(false, predicate_fn.eval(Path::new("non-existent-file.foo")));
}

#[test]
fn test_predicate_is_dir() {
    let predicate_fn = predicates::path::is_dir();
    assert_eq!(false, predicate_fn.eval(Path::new("Cargo.toml")));
    assert_eq!(true, predicate_fn.eval(Path::new("src")));
    assert_eq!(false, predicate_fn.eval(Path::new("non-existent-file.foo")));
}

#[test]
fn test_predicate_is_symlink() {
    let predicate_fn = predicates::path::is_symlink();
    assert_eq!(false, predicate_fn.eval(Path::new("Cargo.toml")));
    assert_eq!(false, predicate_fn.eval(Path::new("src")));
    assert_eq!(false, predicate_fn.eval(Path::new("non-existent-file.foo")));
}
