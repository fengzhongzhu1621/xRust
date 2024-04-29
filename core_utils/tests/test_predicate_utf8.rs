use core_utils::predicates::str::PredicateStrExt;
use core_utils::predicates::{self, core::Predicate, PredicateBooleanExt};
use std::ffi::OsStr;

#[test]
fn test_predicate_utf8() {
    let predicate_fn = predicates::str::is_empty().not().from_utf8();
    assert_eq!(true, predicate_fn.eval(OsStr::new("Hello")));
    assert_eq!(false, predicate_fn.eval(OsStr::new("")));
    let variable: &[u8] = b"";
    assert_eq!(false, predicate_fn.eval(variable));
}
