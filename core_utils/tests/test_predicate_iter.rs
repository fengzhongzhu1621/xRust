use core_utils::predicates::{self, core::Predicate};

#[test]
fn test_predicate_in_iter_sort() {
    let predicate_fn = predicates::in_iter(vec![1, 3, 5]).sort();
    assert_eq!(true, predicate_fn.eval(&1));
    assert_eq!(false, predicate_fn.eval(&2));
    assert_eq!(true, predicate_fn.eval(&3));

    let predicate_fn = predicates::in_iter(vec!["a", "c", "e"]).sort();
    assert_eq!(true, predicate_fn.eval("a"));
    assert_eq!(false, predicate_fn.eval("b"));
    assert_eq!(true, predicate_fn.eval("c"));

    let predicate_fn = predicates::in_iter(vec![
        String::from("a"),
        String::from("c"),
        String::from("e"),
    ])
    .sort();
    assert_eq!(true, predicate_fn.eval("a"));
    assert_eq!(false, predicate_fn.eval("b"));
    assert_eq!(true, predicate_fn.eval("c"));
}

#[test]
fn test_predicate_in_iter() {
    let predicate_fn = predicates::in_iter(vec![1, 3, 5]);
    assert_eq!(true, predicate_fn.eval(&1));
    assert_eq!(false, predicate_fn.eval(&2));
    assert_eq!(true, predicate_fn.eval(&3));

    let predicate_fn = predicates::in_iter(vec!["a", "c", "e"]);
    assert_eq!(true, predicate_fn.eval("a"));
    assert_eq!(false, predicate_fn.eval("b"));
    assert_eq!(true, predicate_fn.eval("c"));

    let predicate_fn = predicates::in_iter(vec![
        String::from("a"),
        String::from("c"),
        String::from("e"),
    ]);
    assert_eq!(true, predicate_fn.eval("a"));
    assert_eq!(false, predicate_fn.eval("b"));
    assert_eq!(true, predicate_fn.eval("c"));
}

fn test_predicate_in_hash() {
    let predicate_fn = predicates::in_hash(vec![1, 3, 5]);
    assert_eq!(true, predicate_fn.eval(&1));
    assert_eq!(false, predicate_fn.eval(&2));
    assert_eq!(true, predicate_fn.eval(&3));

    let predicate_fn = predicates::in_hash(vec!["a", "c", "e"]);
    assert_eq!(true, predicate_fn.eval("a"));
    assert_eq!(false, predicate_fn.eval("b"));
    assert_eq!(true, predicate_fn.eval("c"));

    let predicate_fn = predicates::in_hash(vec![
        String::from("a"),
        String::from("c"),
        String::from("e"),
    ]);
    assert_eq!(true, predicate_fn.eval("a"));
    assert_eq!(false, predicate_fn.eval("b"));
    assert_eq!(true, predicate_fn.eval("c"));
}
