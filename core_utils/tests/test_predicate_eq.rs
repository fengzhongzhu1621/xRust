use core_utils::predicates::{self, core::Predicate};

#[test]
fn test_predicate_eq() {
    let predicate_fn = predicates::eq(5);
    assert_eq!(true, predicate_fn.eval(&5));
    assert_eq!(false, predicate_fn.eval(&10));
    let expect = r#"
Some(
    Case {
        predicate: "Some(var == 5)",
        result: true,
        products: [
            ("var", 5),
        ],
        children: [],
    },
)"#
    .trim();
    assert_eq!(format!("{:#?}", predicate_fn.find_case(true, &5)), expect);
}
