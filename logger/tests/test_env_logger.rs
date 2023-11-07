use log::{Level, LevelFilter, Record};
use logger;
use logger::filter::{enabled, Builder, Directive, Filter};
use logger::fmt::{
    is_stderr, is_stdout, BufferWriter, DefaultFormat, Formatter,
    WritableTarget, WriteStyle,
};
use std::fmt;

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

#[test]
#[cfg(feature = "auto-color")]
fn test_is_stdout() {
    assert_eq!(is_stdout(), true);
}

#[test]
#[cfg(feature = "auto-color")]
fn test_is_stderr() {
    assert_eq!(is_stderr(), true);
}

#[test]
#[cfg(not(feature = "auto-color"))]
fn test_is_stdout() {
    assert_eq!(is_stdout(), false);
}

#[test]
#[cfg(not(feature = "auto-color"))]
fn test_is_stderr() {
    assert_eq!(is_stderr(), false);
}

#[test]
#[cfg(not(feature = "auto-color"))]
fn test_buffer() {
    let mut buffer = Buffer(vec![1, 2, 3]);
    assert_eq!(buffer.flush().unwrap(), ());

    assert_eq!(buffer.write(&[4, 5]).unwrap(), 2);
    assert_eq!(buffer.bytes().len(), 5);

    buffer.clear();
    assert_eq!(buffer.bytes().len(), 0);
}

#[test]
#[cfg(not(feature = "auto-color"))]
fn test_buffer_writer() {
    let buffer_writer = BufferWriter { target: WritableTarget::Stdout };
    let sparkle_heart = vec![240, 159, 146, 150];
    let buffer = Buffer(sparkle_heart);
    let _x = buffer_writer.print(&buffer);
    println!("{:?}", WritableTarget::Stdout);
}

fn write_record(record: Record, fmt: DefaultFormat) -> String {
    let buf = fmt.buf.buf.clone();

    fmt.write(&record).expect("failed to write record");

    let buf = buf.borrow();
    String::from_utf8(buf.bytes().to_vec()).expect("failed to read record")
}

fn write_target(target: &str, fmt: DefaultFormat) -> String {
    write_record(
        Record::builder()
            .args(format_args!("log\nmessage"))
            .level(Level::Info)
            .file(Some("test.rs"))
            .line(Some(144))
            .module_path(Some("test::path"))
            .target(target)
            .build(),
        fmt,
    )
}

fn write(fmt: DefaultFormat) -> String {
    write_target("", fmt)
}

#[test]
fn test_format_with_header() {
    // 使用 Builder 创建 Writer
    // 设置 print styles 为 Never
    // target 默认为 Stderr
    let writer = logger::fmt::WriteBuilder::new()
        .write_style(WriteStyle::Never)
        .build();

    // 根据 Writer 创建格式化器 Formatter
    let mut f = Formatter::new(&writer);

    let written = write(DefaultFormat {
        timestamp: None,
        module_path: true,
        target: false,
        level: true,
        written_header_value: false,
        indent: None,
        suffix: "\n",
        buf: &mut f,
    });

    assert_eq!("[INFO  test::path] log\nmessage\n", written);
}
