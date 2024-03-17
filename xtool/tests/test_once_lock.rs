use std::sync::{
    atomic::{AtomicBool, Ordering},
    OnceLock,
};

/// 定义全局变量，智能初始化一次
static KILL_BEFORE_DEV_FLAG: OnceLock<AtomicBool> = OnceLock::new();
static KILL_BEFORE_DEV_FLAG_2: OnceLock<AtomicBool> = OnceLock::new();

#[test]
fn test_set() {
    // 第一次设置
    KILL_BEFORE_DEV_FLAG.set(AtomicBool::default()).unwrap();

    let actual = KILL_BEFORE_DEV_FLAG.get().unwrap().load(Ordering::Relaxed);
    assert_eq!(actual, false);

    // 第二次设置，值不生效
    let result = KILL_BEFORE_DEV_FLAG.set(AtomicBool::new(true));
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_get() {
    assert_eq!(KILL_BEFORE_DEV_FLAG_2.get().is_none(), true);

    KILL_BEFORE_DEV_FLAG_2.set(AtomicBool::new(true)).unwrap();

    assert_eq!(KILL_BEFORE_DEV_FLAG_2.get().is_some(), true);
}
