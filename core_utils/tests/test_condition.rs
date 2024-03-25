use core_utils::condition::Condition;

#[test]
fn test_condition_debug() {
    let condition = Condition::DEFAULT;
    assert_eq!(format!("{:?}", condition), "Condition::DEFAULT");

    let condition = Condition(Condition::always);
    assert_eq!(format!("{:?}", condition), "Condition::ALWAYS");

    // 打印函数的地址
    #[allow(dead_code)]
    fn other() -> bool {
        false
    }
    // let condition = Condition(other);
    // assert_eq!(format!("{:?}", condition), "Condition(0x100521a90)");
}
