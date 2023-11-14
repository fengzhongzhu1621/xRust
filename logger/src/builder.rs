use crate::{filter, fmt, Env};
use crate::fmt::Formatter;
use log::{LevelFilter, Record, SetLoggerError, Log, Metadata};
use std::io;
use std::cell::RefCell;

#[derive(Default)]
pub struct Builder {
    filter: filter::Builder,
    writer: fmt::writer::WriteBuilder,
    format: fmt::Builder,
    built: bool,
}

impl Builder {
    pub fn new() -> Builder {
        Default::default()
    }

    pub fn from_env<'a, E>(env: E) -> Self
    where
        E: Into<Env<'a>>,
    {
        let mut builder = Builder::new();
        builder.parse_env(env);
        builder
    }

    pub fn parse_env<'a, E>(&mut self, env: E) -> &mut Self
    where
        E: Into<Env<'a>>,
    {
        // env 指的是日志过滤级别 -> 转换为 Env 对象
        let env = env.into();

        // 获取环境变量的值
        if let Some(s) = env.get_filter() {
            self.parse_filters(&s);
        }

        if let Some(s) = env.get_write_style() {
            self.parse_write_style(&s);
        }

        self
    }

    pub fn from_default_env() -> Self {
        Self::from_env(Env::default())
    }

    /// Parses the directives string in the same form as the `RUST_LOG`
    /// environment variable.
    ///
    /// See the module documentation for more details.
    pub fn parse_filters(&mut self, filters: &str) -> &mut Self {
        self.filter.parse(filters);
        self
    }

    /// Parses whether or not to write styles in the same form as the `RUST_LOG_STYLE`
    /// environment variable.
    ///
    /// See the module documentation for more details.
    pub fn parse_write_style(&mut self, write_style: &str) -> &mut Self {
        self.writer.parse_write_style(write_style);
        self
    }

        /// Sets the format function for formatting the log output.
    pub fn format<F: 'static>(&mut self, format: F) -> &mut Self
    where
        F: Fn(&mut Formatter, &Record) -> io::Result<()> + Sync + Send,
    {
        self.format.custom_format = Some(Box::new(format));
        self
    }

        /// Use the default format.
    ///
    /// This method will clear any custom format set on the builder.
    pub fn default_format(&mut self) -> &mut Self {
        self.format = Default::default();
        self
    }

    /// Whether or not to write the level in the default format.
    pub fn format_level(&mut self, write: bool) -> &mut Self {
        self.format.format_level = write;
        self
    }

    /// Whether or not to write the module path in the default format.
    pub fn format_module_path(&mut self, write: bool) -> &mut Self {
        self.format.format_module_path = write;
        self
    }

    /// Whether or not to write the target in the default format.
    pub fn format_target(&mut self, write: bool) -> &mut Self {
        self.format.format_target = write;
        self
    }
        /// Configures the amount of spaces to use to indent multiline log records.
    /// A value of `None` disables any kind of indentation.
    pub fn format_indent(&mut self, indent: Option<usize>) -> &mut Self {
        self.format.format_indent = indent;
        self
    }

    /// Configures if timestamp should be included and in what precision.
    pub fn format_timestamp(&mut self, timestamp: Option<fmt::TimestampPrecision>) -> &mut Self {
        self.format.format_timestamp = timestamp;
        self
    }

    /// Configures the timestamp to use second precision.
    pub fn format_timestamp_secs(&mut self) -> &mut Self {
        self.format_timestamp(Some(fmt::TimestampPrecision::Seconds))
    }

    /// Configures the timestamp to use millisecond precision.
    pub fn format_timestamp_millis(&mut self) -> &mut Self {
        self.format_timestamp(Some(fmt::TimestampPrecision::Millis))
    }

    /// Configures the timestamp to use microsecond precision.
    pub fn format_timestamp_micros(&mut self) -> &mut Self {
        self.format_timestamp(Some(fmt::TimestampPrecision::Micros))
    }

    /// Configures the timestamp to use nanosecond precision.
    pub fn format_timestamp_nanos(&mut self) -> &mut Self {
        self.format_timestamp(Some(fmt::TimestampPrecision::Nanos))
    }

    /// Configures the end of line suffix.
    pub fn format_suffix(&mut self, suffix: &'static str) -> &mut Self {
        self.format.format_suffix = suffix;
        self
    }

    /// Adds a directive to the filter for a specific module.
    pub fn filter_module(&mut self, module: &str, level: LevelFilter) -> &mut Self {
        self.filter.filter_module(module, level);
        self
    }

    /// Adds a directive to the filter for all modules.
    pub fn filter_level(&mut self, level: LevelFilter) -> &mut Self {
        self.filter.filter_level(level);
        self
    }

    /// Adds filters to the logger.
    pub fn filter(&mut self, module: Option<&str>, level: LevelFilter) -> &mut Self {
        self.filter.filter(module, level);
        self
    }

    /// Sets the target for the log output.
    pub fn target(&mut self, target: fmt::Target) -> &mut Self {
        self.writer.target(target);
        self
    }

    /// Sets whether or not styles will be written.
    pub fn write_style(&mut self, write_style: fmt::WriteStyle) -> &mut Self {
        self.writer.write_style(write_style);
        self
    }


    /// Initializes the global logger with the built env logger.
    pub fn try_init(&mut self) -> Result<(), SetLoggerError> {
        let logger = self.build();

        let max_level = logger.filter();
        let r = log::set_boxed_logger(Box::new(logger));

        if r.is_ok() {
            log::set_max_level(max_level);
        }

        r
    }

    /// Initializes the global logger with the built env logger.
    pub fn init(&mut self) {
        self.try_init()
            .expect("Builder::init should not be called after logger initialized");
    }

    /// Build an env logger.
    pub fn build(&mut self) -> Logger {
        assert!(!self.built, "attempt to re-use consumed builder");
        self.built = true;

        Logger {
            writer: self.writer.build(),
            filter: self.filter.build(),
            format: self.format.build(),
        }
    }

}

/// Create a new builder with the default environment variables.
pub fn builder() -> Builder {
    Builder::from_default_env()
}

impl std::fmt::Debug for Builder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.built {
            f.debug_struct("Logger").field("built", &true).finish()
        } else {
            f.debug_struct("Logger")
                .field("filter", &self.filter)
                .field("writer", &self.writer)
                .finish()
        }
    }
}

/// Create a builder from the given environment variables.
///
/// The builder can be configured before being initialized.
#[deprecated(
    since = "0.8.0",
    note = "Prefer `env_logger::Builder::from_env()` instead."
)]
pub fn from_env<'a, E>(env: E) -> Builder
where
    E: Into<Env<'a>>,
{
    Builder::from_env(env)
}


pub struct Logger {
    writer: fmt::Writer,
    filter: filter::Filter,
    format: fmt::FormatFn,
}

impl Logger {

    pub fn from_env<'a, E>(env: E) -> Self
    where
        E: Into<Env<'a>>,
    {
        Builder::from_env(env).build()
    }

    pub fn from_default_env() -> Self {
        Builder::from_default_env().build()
    }

    /// Returns the maximum `LevelFilter` that this env logger instance is
    /// configured to output.
    pub fn filter(&self) -> LevelFilter {
        self.filter.filter()
    }

    /// Checks if this record matches the configured filter.
    pub fn matches(&self, record: &Record) -> bool {
        self.filter.matches(record)
    }
}

impl std::fmt::Debug for Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Logger")
            .field("filter", &self.filter)
            .finish()
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if self.matches(record) {
            // Log records are written to a thread-local buffer before being printed
            // to the terminal. We clear these buffers afterwards, but they aren't shrunk
            // so will always at least have capacity for the largest log record formatted
            // on that thread.
            //
            // If multiple `Logger`s are used by the same threads then the thread-local
            // formatter might have different color support. If this is the case the
            // formatter and its buffer are discarded and recreated.

            thread_local! {
                static FORMATTER: RefCell<Option<Formatter>> = RefCell::new(None);
            }

            // 定义日志写方法
            let print = |formatter: &mut Formatter, record: &Record| {
                let _ =
                    (self.format)(formatter, record).and_then(|_| formatter.print(&self.writer));

                // Always clear the buffer afterwards
                formatter.clear();
            };

            let printed = FORMATTER
                .try_with(|tl_buf| {
                    match tl_buf.try_borrow_mut() {
                        // There are no active borrows of the buffer
                        Ok(mut tl_buf) => match *tl_buf {
                            // We have a previously set formatter
                            Some(ref mut formatter) => {
                                // Check the buffer style. If it's different from the logger's
                                // style then drop the buffer and recreate it.
                                if formatter.write_style() != self.writer.write_style() {
                                    *formatter = Formatter::new(&self.writer);
                                }

                                print(formatter, record);
                            }
                            // We don't have a previously set formatter
                            None => {
                                let mut formatter = Formatter::new(&self.writer);
                                print(&mut formatter, record);

                                *tl_buf = Some(formatter);
                            }
                        },
                        // There's already an active borrow of the buffer (due to re-entrancy)
                        Err(_) => {
                            print(&mut Formatter::new(&self.writer), record);
                        }
                    }
                })
                .is_ok();

            if !printed {
                // The thread-local storage was not available (because its
                // destructor has already run). Create a new single-use
                // Formatter on the stack for this call.
                print(&mut Formatter::new(&self.writer), record);
            }
        }
    }

    fn flush(&self) {}
}

/// Initializes the global logger with an env logger.
pub fn init() {
    try_init().expect("env_logger::init should not be called after logger initialized");
}

/// Attempts to initialize the global logger with an env logger.
pub fn try_init() -> Result<(), SetLoggerError> {
    try_init_from_env(Env::default())
}

pub fn try_init_from_env<'a, E>(env: E) -> Result<(), SetLoggerError>
where
    E: Into<Env<'a>>,
{
    let mut builder = Builder::from_env(env);

    builder.try_init()
}

pub fn init_from_env<'a, E>(env: E)
where
    E: Into<Env<'a>>,
{
    try_init_from_env(env)
        .expect("env_logger::init_from_env should not be called after logger initialized");
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn builder_parse_env_overrides_existing_filters() {
        env::set_var(
            "builder_parse_default_env_overrides_existing_filters",
            "debug",
        );
        let env = Env::new().filter("builder_parse_default_env_overrides_existing_filters");

        let mut builder = Builder::new();
        builder.filter_level(LevelFilter::Trace);
        // Overrides global level to debug
        builder.parse_env(env);

        assert_eq!(builder.filter.build().filter(), LevelFilter::Debug);
    }
}