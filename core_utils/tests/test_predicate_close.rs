use core_utils::predicates::{self, core::predicate::Predicate};

#[test]
fn test_predicate_is_close() {
    let a = 0.15_f64 + 0.15_f64 + 0.15_f64; // 0.45
    let b = 0.1_f64 + 0.1_f64 + 0.25_f64; // 0.45
    let c = 0.451_f64; // 0.451

    // 在默认误差范围内比较 a 和 b 的大小
    let predicate_fn = predicates::float::is_close(a);
    assert_eq!(true, predicate_fn.eval(&b));

    // 在误差范围是 0 时，比较 a 和 b 的大小
    assert_eq!(false, predicate_fn.distance(0).eval(&b));

    // 比较 a 和 c
    assert_eq!(false, predicate_fn.distance(1).eval(&c));
    assert_eq!(false, predicate_fn.distance(2).eval(&c));
    assert_eq!(false, predicate_fn.distance(3).eval(&c));
    assert_eq!(false, predicate_fn.distance(4).eval(&c));
    assert_eq!(false, predicate_fn.distance(5).eval(&c));
    assert_eq!(false, predicate_fn.distance(10).eval(&c));
    assert_eq!(false, predicate_fn.distance(100).eval(&c));
    assert_eq!(false, predicate_fn.distance(100000).eval(&c));

    println!("{}", predicate_fn);
    println!("{:#}", predicate_fn);
    println!("{:#?}", predicate_fn);

    println!("{:#?}", predicate_fn.find_case(true, &b))

    // var != 0.44999999999999996
    // var != 0.44999999999999996 // 有颜色
    // IsClosePredicate {
    //     target: 0.44999999999999996,
    //     epsilon: 4.440892098500626e-16,
    //     ulps: 2,
    // }

    // Some(
    //     Case {
    //         predicate: "Some(var != 0.44999999999999996)",
    //         result: true,
    //         products: [
    //             ("actual epsilon", 0.00000000000000005551115123125783),
    //             ("actual ulps", 1),
    //         ],
    //         children: [],
    //     },
    // )
}
