/*
This internal module contains the terminal detection implementation.

If the `auto-color` feature is enabled then we detect whether we're attached to a particular TTY.
Otherwise, assume we're not attached to anything. This effectively prevents styles from being
printed.
*/

#[cfg(feature = "auto-color")]
mod imp {
    use is_terminal::IsTerminal;

    /// 判断是否是一个标准输出终端
    pub fn is_stdout() -> bool {
        std::io::stdout().is_terminal()
    }

    /// 判断是否是一个标准错误终端
    pub fn is_stderr() -> bool {
        std::io::stderr().is_terminal()
    }
}

#[cfg(not(feature = "auto-color"))]
mod imp {
    pub fn is_stdout() -> bool {
        false
    }

    pub(in crate::fmt) fn is_stderr() -> bool {
        false
    }
}

// 采用 use self:: 的方式根据特性引入不同的方法
pub use self::imp::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_stdout() {
        assert_eq!(is_stdout(), true);
    }

    #[test]
    fn test_is_stderr() {
        assert_eq!(is_stderr(), true);
    }
}
