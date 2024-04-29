use core_utils::predicates::{self, core::Predicate, PredicateBooleanExt};

#[test]
fn test_predicate_always() {
    let predicate_fn = predicates::always();
    assert_eq!(true, predicate_fn.eval(&5));
    assert_eq!(true, predicate_fn.eval(&10));
    assert_eq!(true, predicate_fn.eval(&15));

    assert_eq!(true, predicate_fn.eval("hello"))
}

#[test]
fn test_predicate_neven() {
    let predicate_fn = predicates::never();
    assert_eq!(false, predicate_fn.eval(&5));
    assert_eq!(false, predicate_fn.eval(&10));
    assert_eq!(false, predicate_fn.eval(&15));
    assert_eq!(false, predicate_fn.eval("hello"))
}

#[test]
fn test_and() {
    let predicate_fn1 = predicates::always().and(predicates::always());
    let predicate_fn2 = predicates::always().and(predicates::never());
    assert_eq!(true, predicate_fn1.eval(&4));
    assert_eq!(false, predicate_fn2.eval(&4));
}

#[test]
fn test_or() {
    let predicate_fn1 = predicates::always().or(predicates::always());
    let predicate_fn2 = predicates::always().or(predicates::never());
    let predicate_fn3 = predicates::never().or(predicates::never());
    assert_eq!(true, predicate_fn1.eval(&4));
    assert_eq!(true, predicate_fn2.eval(&4));
    assert_eq!(false, predicate_fn3.eval(&4));
}

#[test]
fn test_not() {
    let predicate_fn1 = predicates::always().not();
    let predicate_fn2 = predicates::never().not();
    assert_eq!(false, predicate_fn1.eval(&4));
    assert_eq!(true, predicate_fn2.eval(&4));
}

#[test]
fn find_case_true() {
    assert!(predicates::always()
        .or(predicates::always())
        .find_case(true, &5)
        .is_some());
}

#[test]
fn find_case_true_left_fail() {
    assert!(predicates::never()
        .or(predicates::always())
        .find_case(true, &5)
        .is_some());
}

#[test]
fn find_case_true_right_fail() {
    assert!(predicates::always()
        .or(predicates::never())
        .find_case(true, &5)
        .is_some());
}

#[test]
fn find_case_true_fails() {
    assert!(predicates::never()
        .or(predicates::never())
        .find_case(true, &5)
        .is_none());
}

#[test]
fn find_case_false() {
    assert!(predicates::never()
        .or(predicates::never())
        .find_case(false, &5)
        .is_some());
}

#[test]
fn find_case_false_fails() {
    assert!(predicates::always()
        .or(predicates::always())
        .find_case(false, &5)
        .is_none());
}

#[test]
fn find_case_false_left_fail() {
    assert!(predicates::never()
        .or(predicates::always())
        .find_case(false, &5)
        .is_none());
}

#[test]
fn find_case_false_right_fail() {
    assert!(predicates::always()
        .or(predicates::never())
        .find_case(false, &5)
        .is_none());
}
