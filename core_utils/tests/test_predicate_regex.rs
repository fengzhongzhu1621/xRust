use core_utils::predicates::{self, core::predicate::Predicate};

#[test]
fn test_predicate_regex() {
    let predicate_fn = predicates::str::is_match("^Hello.*$").unwrap();
    assert_eq!(true, predicate_fn.eval("Hello World"));
    assert_eq!(false, predicate_fn.eval("Food World"));

    let predicate_fn = predicates::str::is_match("T[a-z]*").unwrap().count(3);
    assert_eq!(true, predicate_fn.eval("One Two Three Two One"));
    assert_eq!(false, predicate_fn.eval("One Two Three"));
}
