use core_utils::predicates::path::PredicateFileContentExt;
use core_utils::predicates::str::PredicateStrExt;
use core_utils::predicates::{self, core::Predicate, PredicateBooleanExt};
use std::env;

#[test]
fn test_predicate_fc() {
    let path = env::current_dir().unwrap().join("tests/test_predicate_fc.rs");
    let predicate_fn =
        predicates::str::is_empty().not().from_utf8().from_file_path();
    assert_eq!(true, predicate_fn.eval(&path));
}
