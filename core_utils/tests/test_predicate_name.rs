use core_utils::predicates::{
    self, core::Predicate, PredicateBooleanExt, PredicateNameExt,
};

#[test]
fn test_predicate_name() {
    let predicate_fn = predicates::str::is_empty().not().name("non-empty");
    println!("{:#?}", predicate_fn);
}
