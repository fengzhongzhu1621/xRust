use crate::Level;

/// Metadata about a log message.
///
/// # Use
///
/// `Metadata` structs are created when users of the library use
/// logging macros.
///
/// They are consumed by implementations of the `Log` trait in the
/// `enabled` method.
///
/// `Record`s use `Metadata` to determine the log message's severity
/// and target.
///
/// Users should use the `log_enabled!` macro in their code to avoid
/// constructing expensive log messages.
///
/// # Examples
///
/// ```edition2018
/// use log::{Record, Level, Metadata};
///
/// struct MyLogger;
///
/// impl log::Log for MyLogger {
///     fn enabled(&self, metadata: &Metadata) -> bool {
///         metadata.level() <= Level::Info
///     }
///
///     fn log(&self, record: &Record) {
///         if self.enabled(record.metadata()) {
///             println!("{} - {}", record.level(), record.args());
///         }
///     }
///     fn flush(&self) {}
/// }
///
/// # fn main(){}
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Metadata<'a> {
    level: Level,    // 日志级别
    target: &'a str, // 目标名
}

/// Metadata 方法
impl<'a> Metadata<'a> {
    /// The verbosity level of the message.
    #[inline]
    pub fn level(&self) -> Level {
        self.level
    }

    /// The name of the target of the directive.
    #[inline]
    pub fn target(&self) -> &'a str {
        self.target
    }
}

/// Metadata 类型转换
impl<'a> Metadata<'a> {
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> MetadataBuilder<'a> {
        MetadataBuilder::new()
    }
}

/// Builder for [`Metadata`](struct.Metadata.html).
///
/// Typically should only be used by log library creators or for testing and "shim loggers".
/// The `MetadataBuilder` can set the different parameters of a `Metadata` object, and returns
/// the created object when `build` is called.
///
/// # Example
///
/// ```edition2018
/// let target = "myApp";
/// use log::{Level, MetadataBuilder};
/// let metadata = MetadataBuilder::new()
///                     .level(Level::Debug)
///                     .target(target)
///                     .build();
/// ```
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MetadataBuilder<'a> {
    metadata: Metadata<'a>,
}

/// MetadataBuilder 构造函数
impl<'a> MetadataBuilder<'a> {
    /// Construct a new `MetadataBuilder`.
    ///
    /// The default options are:
    ///
    /// - `level`: `Level::Info`
    /// - `target`: `""`
    #[inline]
    pub fn new() -> MetadataBuilder<'a> {
        MetadataBuilder {
            metadata: Metadata { level: Level::Info, target: "" },
        }
    }
}
impl<'a> Default for MetadataBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// MetadataBuilder 方法
impl<'a> MetadataBuilder<'a> {
    /// Setter for [`level`](struct.Metadata.html#method.level).
    #[inline]
    pub fn level(&mut self, arg: Level) -> &mut MetadataBuilder<'a> {
        self.metadata.level = arg;
        self
    }

    /// Setter for [`target`](struct.Metadata.html#method.target).
    #[inline]
    pub fn target(&mut self, target: &'a str) -> &mut MetadataBuilder<'a> {
        self.metadata.target = target;
        self
    }
}

/// MetadataBuilder 类型转换
impl<'a> MetadataBuilder<'a> {
    /// Returns a `Metadata` object.
    /// 每次执行都创建一个新的 Metadata 对象
    #[inline]
    pub fn build(&self) -> Metadata<'a> {
        self.metadata.clone()
    }
}
