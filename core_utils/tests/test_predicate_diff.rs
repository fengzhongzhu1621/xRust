use core_utils::predicates::{self, core::Predicate};

#[test]
fn test_predicate_diff() {
    let predicate_fn = predicates::str::diff("Hello World");
    assert_eq!(true, predicate_fn.eval("Hello World"));
    assert!(predicate_fn.find_case(false, "Hello World").is_none());
    assert_eq!(false, predicate_fn.eval("Goodbye World"));
    assert!(predicate_fn.find_case(false, "Goodbye World").is_some());

    println!("{:#?}", predicate_fn.find_case(false, "Goodbye World"));

    // Some(
    //     Case {
    //         predicate: "Some(diff original var)",
    //         result: true,
    //         products: [
    //             ("diff", 
    //             ---         orig
    //             +++         var
    //             @@ -1 +1 @@
    //             -Hello World
    //             +Goodbye World
    //             ),
    //         ],
    //         children: [],
    //     },
    // )
}

#[cfg(not(feature = "color"))]
fn colorize_diff(lines: Vec<String>, _palette: crate::Palette) -> Vec<String> {
    lines
}
