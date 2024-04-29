use core_utils::predicates::{self, core::Predicate};
use std::path::Path;

#[test]
fn test_predicate_existence_exists() {
    let predicate_fn = predicates::path::exists();
    assert_eq!(true, predicate_fn.eval(Path::new("Cargo.toml")));
    let expect = r#"
Some(
    Case {
        predicate: "Some(exists(var))",
        result: true,
        products: [
            ("var", Cargo.toml),
        ],
        children: [],
    },
)"#
    .trim();
    assert_eq!(
        format!(
            "{:#?}",
            predicate_fn.find_case(true, Path::new("Cargo.toml"))
        ),
        expect
    );
}

#[test]
fn test_predicate_existence_missing() {
    let predicate_fn = predicates::path::missing();
    assert_eq!(false, predicate_fn.eval(Path::new("Cargo.toml")));
    let expect = r#"
Some(
    Case {
        predicate: "Some(missing(var))",
        result: false,
        products: [
            ("var", Cargo.toml),
        ],
        children: [],
    },
)"#
    .trim();
    assert_eq!(
        format!(
            "{:#?}",
            predicate_fn.find_case(false, Path::new("Cargo.toml"))
        ),
        expect
    );
}
