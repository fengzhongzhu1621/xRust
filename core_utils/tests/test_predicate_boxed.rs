use core_utils::predicates::core::Predicate;
use core_utils::predicates::{self, PredicateBoxExt};

#[test]
fn test_predicate_boxed() {
    let predicate_list =
        vec![predicates::always().boxed(), predicates::never().boxed()];
    assert_eq!(true, predicate_list[0].eval(&4));
    assert_eq!(false, predicate_list[1].eval(&4));
}
