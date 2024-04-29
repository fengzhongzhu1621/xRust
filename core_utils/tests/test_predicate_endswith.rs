use core_utils::predicates::{self, core::Predicate};

#[test]
fn test_predicate_str_endswith() {
    let predicate_fn = predicates::str::ends_with("World");
    let expect = r#"
Some(
    Case {
        predicate: "Some(var.ends_with(\"World\"))",
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
