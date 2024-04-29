use core_utils::predicates::{self, core::Predicate};

#[test]
fn test_predicate_str_contains() {
    let predicate_fn = predicates::str::contains("Two");
    let expect = r#"
Some(
    Case {
        predicate: "Some(var.contains(Two))",
        result: true,
        products: [
            ("var", One Two Three),
        ],
        children: [],
    },
)
"#
    .trim();
    assert_eq!(
        format!("{:#?}", predicate_fn.find_case(true, "One Two Three")),
        expect
    );

    let predicate_fn = predicates::str::contains("Two").count(2);
    assert_eq!(true, predicate_fn.eval("One Two Three Two One"));
    assert_eq!(false, predicate_fn.eval("One Two Three One"));
}
