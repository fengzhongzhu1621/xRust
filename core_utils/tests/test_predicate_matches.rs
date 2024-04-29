use core_utils::predicates::{self, core::Predicate};

#[test]
fn test_predicate_str_matches() {
    let predicate_fn = predicates::str::MatchesPredicate {
        pattern: "Two".to_owned(),
        count: 2,
    };
    let expect = r#"
Some(
    Case {
        predicate: "Some(var.contains(Two))",
        result: true,
        products: [
            ("var", One Two Two Three),
            ("actual count", 2),
        ],
        children: [],
    },
)
"#
    .trim();
    assert_eq!(
        format!("{:#?}", predicate_fn.find_case(true, "One Two Two Three")),
        expect
    );
}
