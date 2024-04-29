use core_utils::predicates::{self, core::Predicate};

#[test]
fn test_predicate_str_is_empty() {
    let predicate_fn = predicates::str::is_empty();
    assert_eq!(true, predicate_fn.eval(""));
    assert_eq!(false, predicate_fn.eval("Food World"));

    let expect = r#"
Some(
    Case {
        predicate: "Some(var.is_empty())",
        result: false,
        products: [
            ("var", Food World),
        ],
        children: [],
    },
)
"#
    .trim();
    assert_eq!(
        format!("{:#?}", predicate_fn.find_case(false, "Food World")),
        expect
    );
}
