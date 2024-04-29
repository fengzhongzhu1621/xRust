use core_utils::predicates::str::PredicateStrExt;
use core_utils::predicates::{self, core::Predicate};

#[test]
fn test_predicate_normalized() {
    let input =
        "This is a string \n with \r some \n\r\n random newlines\r\r\n\n";
    let predicate_fn = predicates::eq(
        "This is a string \n with \n some \n\n random newlines\n\n\n",
    )
    .normalize();
    assert_eq!(true, predicate_fn.eval(input));

    let predicate_fn = predicates::eq("Hello World!\n").normalize();
    assert_eq!(true, predicate_fn.eval("Hello World!\n"));
    assert_eq!(true, predicate_fn.eval("Hello World!\r"));
    assert_eq!(true, predicate_fn.eval("Hello World!\r\n"));
    assert_eq!(false, predicate_fn.eval("Goodbye"));
}
