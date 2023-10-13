use log::{self, Log};



///定义一个 Logger，是空的元祖结构体，用于将日志输出到标准输出
#[derive(Debug)]
pub struct Logger(());


// 定义一个全局日志对象
const LOGGER: &'static Logger = &Logger(());


/// set_logger(logger: &'static dyn Log) 需要的参数 logger，必须实现 Log trait 的方法
/// Sync + Send 
/// Sync: 允许多线程同时访问，标记了实现它的类型可以安全的在线程间共享访问
/// Send: 允许线程间转移所有权，并发中需要安全传递值都需要被标记实现 Send，否则编译器会报错
impl Log for Logger {
    /// Determines if a log message with the specified metadata would be
    /// logged.
    ///
    /// This is used by the `log_enabled!` macro to allow callers to avoid
    /// expensive computation of log message arguments if the message would be
    /// This method isn't called automatically by the `log!` macros.
    /// It's up to an implementation of the `Log` trait to call `enabled` in its own
    /// `log` method implementation to guarantee that filtering is applied.
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        // We set the log level via log::set_max_level, so we don't need to
        // implement filtering here.
        true
    }

    /// Note that `enabled` is *not* necessarily called before this method.
    /// Implementations of `log` should perform all necessary filtering
    /// internally.
    fn log(&self, record: &log::Record) {
        // 匹配日志所在的文件和行号
        match (record.file(), record.line()) {
            // 文件和行号都存在
            (Some(file), Some(line)) => {
                eprintln_locked!(
                    "{}|{}|{}:{}: {}",
                    record.level(),
                    record.target(),
                    file,
                    line,
                    record.args()
                );
            }
            // 只有文件存在
            (Some(file), None) => {
                eprintln_locked!(
                    "{}|{}|{}: {}",
                    record.level(),
                    record.target(),
                    file,
                    record.args()
                );
            }
            _ => {
                eprintln_locked!(
                    "{}|{}: {}",
                    record.level(),
                    record.target(),
                    record.args()
                );
            }
        }
    }

    /// Flushes any buffered records.
    fn flush(&self) {
        // We use eprintln_locked! which is flushed on every call.
    }
}

/// Logger的构造函数
impl Logger {
    /// Create a new logger that logs to stderr and initialize it as the
    /// global logger. If there was a problem setting the logger, then an
    /// error is returned.
    pub fn init() -> Result<(), log::SetLoggerError> {
        log::set_logger(LOGGER)
    }
}
