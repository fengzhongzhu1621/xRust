
use std::cmp;
use std::fmt;
use std::str::FromStr;
use std::error;
use core::*;


/// 定义日志级别的名称
static LOG_LEVEL_NAMES: [&str; 6] = ["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

/// 日志级别的名称错误时的提示信息
static LEVEL_PARSE_ERROR: &str =
    "attempted to convert a string that doesn't match an existing log level";

/// The type returned by [`from_str`] when the string doesn't match any of the log levels.
/// 
/// [`from_str`]: https://doc.rust-lang.org/std/str/trait.FromStr.html#tymethod.from_str
#[allow(missing_copy_implementations)]
#[derive(Debug, PartialEq, Eq)]
pub struct ParseLevelError(pub());

impl error::Error for ParseLevelError {}

impl fmt::Display for ParseLevelError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(LEVEL_PARSE_ERROR)
    }
}

/// An enum representing the available verbosity levels of the logger.
/// repr: 内存对齐
#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Level {
    /// The "error" level.
    ///
    /// Designates very serious errors.
    // This way these line up with the discriminants for LevelFilter below
    // This works because Rust treats field-less enums the same way as C does:
    // https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-field-less-enumerations
    Error = 1,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug,
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace,
}


/// 比较 Level 和 LevelFilter
impl PartialEq<LevelFilter> for Level {
    #[inline]
    fn eq(&self, other: &LevelFilter) -> bool {
        // 比较枚举的值
        *self as usize == *other as usize
    }
}

impl PartialOrd<LevelFilter> for Level {
    #[inline]
    fn partial_cmp(&self, other: &LevelFilter) -> Option<cmp::Ordering> {
        Some((*self as usize).cmp(&(*other as usize)))
    }
}

/// 构造函数，将无符号整数转换为 Level类型
impl Level {
    fn from_usize(u: usize) -> Option<Level> {
        match u {
            1 => Some(Level::Error),
            2 => Some(Level::Warn),
            3 => Some(Level::Info),
            4 => Some(Level::Debug),
            5 => Some(Level::Trace),
            _ => None,
        }
    }
}

/// 实现类型转换函数 .parse()，将字符串转换为 Level 类型
impl FromStr for Level {
    type Err = ParseLevelError;

    fn from_str(level: &str) -> Result<Level, Self::Err> {
        core::ok_or(
            LOG_LEVEL_NAMES
                .iter()
                .position(|&name| name.eq_ignore_ascii_case(level))
                .into_iter()
                .filter(|&idx| idx != 0)
                .map(|idx| Level::from_usize(idx).unwrap())
                .next(),
            ParseLevelError(()),
        )
    }
}

/// An enum representing the available verbosity level filters of the logger.
///
/// A `LevelFilter` may be compared directly to a [`Level`]. Use this type
/// to get and set the maximum log level with [`max_level()`] and [`set_max_level`].
///
/// [`Level`]: enum.Level.html
/// [`max_level()`]: fn.max_level.html
/// [`set_max_level`]: fn.set_max_level.html
#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum LevelFilter {
    /// A level lower than all log levels.
    Off,
    /// Corresponds to the `Error` log level.
    Error,
    /// Corresponds to the `Warn` log level.
    Warn,
    /// Corresponds to the `Info` log level.
    Info,
    /// Corresponds to the `Debug` log level.
    Debug,
    /// Corresponds to the `Trace` log level.
    Trace,
}
