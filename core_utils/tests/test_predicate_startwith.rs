use core_utils::predicates::{self, core::Predicate};

#[test]
fn test_predicate_str_startwith() {
    let predicate_fn = predicates::str::starts_with("Hello");
    let expect = r#"
Some(
    Case {
        predicate: "Some(var.starts_with(\"Hello\"))",
        result: true,
        products: [
            ("var", Hello World),
        ],
        children: [],
    },
)
"#
    .trim();
    assert_eq!(
        format!("{:#?}", predicate_fn.find_case(true, "Hello World")),
        expect
    );
}
