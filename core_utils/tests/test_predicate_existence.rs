use core_utils::predicates;
use core_utils::predicates::predicate::Predicate;
use std::path::Path;

#[test]
fn test_predicate_existence() {
    let predicate_fn = predicates::existence::exists();
    assert_eq!(true, predicate_fn.eval(Path::new("Cargo.toml")));

    let predicate_fn = predicates::existence::missing();
    assert_eq!(false, predicate_fn.eval(Path::new("Cargo.toml")));
}
