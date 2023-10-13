use std::sync::atomic::{AtomicBool, Ordering};

/// 消息显示开关
static MESSAGES: AtomicBool = AtomicBool::new(false);
static IGNORE_MESSAGES: AtomicBool = AtomicBool::new(false);
static ERRORED: AtomicBool = AtomicBool::new(false);

/// Like eprintln, but locks STDOUT to prevent interleaving lines.
/// 打印日志到标准错误输出
#[macro_export]
macro_rules! eprintln_locked {
    // 参数可以出现 0 次或多次
    // 指示符: tt (标记树 token tree)
    ($($tt:tt)*) => {{
        {
            // This is a bit of an abstraction violation because we explicitly
            // lock STDOUT before printing to STDERR. This avoids interleaving
            // lines within ripgrep because `search_parallel` uses `termcolor`,
            // which accesses the same STDOUT lock when writing lines.
            // 打印到标准错误输出时,先对标准输出加锁
            let stdout = std::io::stdout();
            let _handle = stdout.lock();
            // 打印到标准错误输出
            eprintln!($($tt)*);
        }
    }}
}

/// Emit a non-fatal error message, unless messages were disabled.
/// 当日志开关打开时，打印日志到标准错误输出
#[macro_export]
macro_rules! message {
    ($($tt:tt)*) => {
        //  crate:: 表示绝对路径
        if crate::messages::messages() {
            eprintln_locked!($($tt)*);
        }
    }
}

/// Like message, but sets ripgrep's "errored" flag, which controls the exit
/// status.
/// 当日志开关打开时，打印日志到标准错误输出，且设置错误标识，用于判断程序是否退出
#[macro_export]
macro_rules! err_message {
    ($($tt:tt)*) => {
        crate::messages::set_errored();
        message!($($tt)*);
    }
}

/// Emit a non-fatal ignore-related error message (like a parse error), unless
/// ignore-messages were disabled.
/// 当日志开关打开时且“忽略”日志开关打开，除了正常的日志外，还打印被标记为忽略状态的消息到标准错误输出
#[macro_export]
macro_rules! ignore_message {
    ($($tt:tt)*) => {
        if crate::messages::messages() && crate::messages::ignore_messages() {
            eprintln_locked!($($tt)*);
        }
    }
}

/// Returns true if and only if messages should be shown.
/// 返回消息开关
pub fn messages() -> bool {
    MESSAGES.load(Ordering::SeqCst)
}

/// Set whether messages should be shown or not.
///
/// By default, they are not shown.
pub fn set_messages(yes: bool) {
    MESSAGES.store(yes, Ordering::SeqCst)
}

/// Returns true if and only if "ignore" related messages should be shown.
pub fn ignore_messages() -> bool {
    IGNORE_MESSAGES.load(Ordering::SeqCst)
}

/// Set whether "ignore" related messages should be shown or not.
///
/// By default, they are not shown.
///
/// Note that this is overridden if `messages` is disabled. Namely, if
/// `messages` is disabled, then "ignore" messages are never shown, regardless
/// of this setting.
pub fn set_ignore_messages(yes: bool) {
    IGNORE_MESSAGES.store(yes, Ordering::SeqCst)
}

/// Returns true if and only if ripgrep came across a non-fatal error.
pub fn errored() -> bool {
    ERRORED.load(Ordering::SeqCst)
}

/// Indicate that ripgrep has come across a non-fatal error.
pub fn set_errored() {
    ERRORED.store(true, Ordering::SeqCst);
}
