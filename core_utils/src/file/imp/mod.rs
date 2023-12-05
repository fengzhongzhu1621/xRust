//! 根据操作系统加载不同的模块

cfg_if::cfg_if! {
    if #[cfg(any(unix, target_os = "redox", target_os = "wasi"))] {
        mod unix;
        pub use self::unix::*;
    } else if #[cfg(windows)] {
        mod windows;
        pub use self::windows::*;
    } else {
        mod other;
        pub use self::other::*;
    }
}
