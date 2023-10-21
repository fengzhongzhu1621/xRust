use log::{Level, LevelFilter};
use logger::filter::{enabled, Builder, Directive, Filter};

fn make_logger_filter(dirs: Vec<Directive>) -> Filter {
    let mut logger = Builder::new().build();
    logger.directives = dirs;
    logger
}

#[test]
fn filter_info() {
    // 创建 Filter 对象
    let filter = Builder::new().filter(None, LevelFilter::Info).build();
    assert!(enabled(&filter.directives, Level::Info, "crate1"));
    assert!(!enabled(&filter.directives, Level::Debug, "crate1"));
}

#[test]
fn filter_beginning_longest_match() {
    // 创建 Filter 并按名称排序
    let filter = Builder::new()
        .filter(Some("crate2"), LevelFilter::Info)
        .filter(Some("crate2::mod"), LevelFilter::Debug)
        .filter(Some("crate1::mod1"), LevelFilter::Warn)
        .build();
    // 逆序匹配到了 crate2::mod, Level::Debug == LevelFilter::Debug
    assert!(enabled(&filter.directives, Level::Debug, "crate2::mod1"));
    // 逆序匹配到了 crate2, 但是 Level::Debug > LevelFilter::Info,超过了最高的日志级别 Info,被过滤不会打印
    assert!(!enabled(&filter.directives, Level::Debug, "crate2"));
}

#[test]
fn parse_default() {
    // info,crate1::mod1=warn 只包含 mod
    let logger = Builder::new().parse("info,crate1::mod1=warn").build();
    // 匹配 crate1::mod1=warn
    assert!(enabled(&logger.directives, Level::Warn, "crate1::mod1"));
    // 匹配 info
    assert!(enabled(&logger.directives, Level::Info, "crate2::mod2"));
}

#[test]
fn parse_default_bare_level_off_lc() {
    let logger = Builder::new().parse("off").build();
    assert!(!enabled(&logger.directives, Level::Error, ""));
    assert!(!enabled(&logger.directives, Level::Warn, ""));
    assert!(!enabled(&logger.directives, Level::Info, ""));
    assert!(!enabled(&logger.directives, Level::Debug, ""));
    assert!(!enabled(&logger.directives, Level::Trace, ""));
}

#[test]
fn parse_default_bare_level_off_uc() {
    let logger = Builder::new().parse("OFF").build();
    assert!(!enabled(&logger.directives, Level::Error, ""));
    assert!(!enabled(&logger.directives, Level::Warn, ""));
    assert!(!enabled(&logger.directives, Level::Info, ""));
    assert!(!enabled(&logger.directives, Level::Debug, ""));
    assert!(!enabled(&logger.directives, Level::Trace, ""));
}

#[test]
fn parse_default_bare_level_error_lc() {
    let logger = Builder::new().parse("error").build();
    assert!(enabled(&logger.directives, Level::Error, ""));
    assert!(!enabled(&logger.directives, Level::Warn, ""));
    assert!(!enabled(&logger.directives, Level::Info, ""));
    assert!(!enabled(&logger.directives, Level::Debug, ""));
    assert!(!enabled(&logger.directives, Level::Trace, ""));
}
