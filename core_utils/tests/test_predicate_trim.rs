use core_utils::predicates::str::PredicateStrExt;
use core_utils::predicates::{self, core::Predicate};

#[test]
fn test_predicate_trim() {
    let predicate_fn = predicates::str::is_empty().trim();
    assert_eq!(true, predicate_fn.eval("    "));
    assert_eq!(false, predicate_fn.eval("    Hello    "));
}
