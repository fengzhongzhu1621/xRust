use core_utils::predicates::{
    self, core::Predicate, function, PredicateBooleanExt,
};

#[test]
fn test_predicate_function() {
    struct Example {
        string: String,
        number: i32,
    }

    let string_check = predicates::function(|x: &Example| x.string == "hello");
    let number_check = predicates::function(|x: &Example| x.number == 42);
    let predicate_fn = string_check.and(number_check);
    let good_example = Example { string: "hello".into(), number: 42 };
    assert_eq!(true, predicate_fn.eval(&good_example));
    let bad_example = Example { string: "goodbye".into(), number: 0 };
    assert_eq!(false, predicate_fn.eval(&bad_example));
}

#[test]
fn test_function() {
    let f = function(|x: &str| x == "hello");
    assert!(f.eval("hello"));
    assert!(!f.eval("goodbye"));
}
