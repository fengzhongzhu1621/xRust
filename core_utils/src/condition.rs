use crate::cached_bool::CachedBool;
use core::sync::atomic::{AtomicPtr, Ordering};
use std::fmt;

// #[repr(transparent)]只能在只有一个字段的结构体(struct)上或只有一个变体的枚举(enum)上使用。
// 目标是使单个成员字段和结构体之间的转换成为可能。
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Condition(
    /// The function that gets called to check the condition.
    pub fn() -> bool,
);

impl Condition {
    /// A condition that evaluates to `true` if the OS supports coloring.
    ///
    /// Uses [`Condition::os_support()`]. On Windows, this condition tries to
    /// enable coloring support on the first call and caches the result for
    /// subsequent calls. Outside of Windows, this always evaluates to `true`.
    pub const DEFAULT: Condition = Condition(Condition::os_support);

    /// A condition that always evaluates to `true`.
    pub const ALWAYS: Condition = Condition(Condition::always);

    /// A condition that always evaluated to `false`.
    pub const NEVER: Condition = Condition(Condition::never);
}

impl Condition {
    /// The backing function for [`Condition::ALWAYS`]. Returns `true` always.
    pub const fn always() -> bool {
        true
    }

    /// The backing function for [`Condition::NEVER`]. Returns `false` always.
    pub const fn never() -> bool {
        false
    }

    /// The backing function for [`Condition::DEFAULT`].
    ///
    /// Returns `true` if the current OS supports ANSI escape sequences for
    /// coloring. Outside of Windows, this always returns `true`. On Windows,
    /// the first call to this function attempts to enable support and returns
    /// whether it was successful every time thereafter.
    pub fn os_support() -> bool {
        // windows 开启终端颜色支持
        crate::console::cache_enable()
    }
}

impl Default for Condition {
    fn default() -> Self {
        Condition::DEFAULT
    }
}

/// 解引用取第一个值
impl core::ops::Deref for Condition {
    type Target = fn() -> bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Condition::DEFAULT {
            f.write_str("Condition::DEFAULT")
        } else if *self == Condition::ALWAYS {
            f.write_str("Condition::ALWAYS")
        } else if *self == Condition::NEVER {
            f.write_str("Condition::NEVER")
        } else {
            // 打印函数的地址，e.g. "Condition(0x100521a90)"
            f.debug_tuple("Condition").field(&self.0).finish()
        }
    }
}

#[repr(transparent)]
pub struct AtomicCondition(AtomicPtr<()>);

impl AtomicCondition {
    pub const DEFAULT: AtomicCondition =
        AtomicCondition::from(Condition::DEFAULT);

    pub const fn from(cond: Condition) -> Self {
        AtomicCondition(AtomicPtr::new(cond.0 as *mut ()))
    }

    pub fn store(&self, cond: Condition) {
        // Release 任何读和写操作（不限于对当前原子变量）都不能被reorder到该写操作之后。并且所有当前线程中在该原子操作之前的所有写操作（不限于对当前原子变量）都对另一个对同一个原子变量使用Acquire Ordering读操作的线程可见。
        self.0.store(cond.0 as *mut (), Ordering::Release)
    }

    pub fn read(&self) -> bool {
        // Acquire 任何读和写操作（不限于对当前原子变量）都不能被reorder到该读操作之前。并且当前线程可以看到另一个线程对同一个原子变量使用Release Ordering写操作之前的所有写操作（不限于对当前原子变量）。
        let condition = unsafe {
            // *mut () => fn() -> bool
            Condition(core::mem::transmute(self.0.load(Ordering::Acquire)))
        };

        condition()
    }
}

// cargo expand --package=core_utils --lib condition | more
// * 说明之前的模式可以出现零次也可以任意次
// ? 表示一个可选的匹配段，可以出现零次或一次。
// meta: 属性，属性中的内容
// expr: 表达式
// ident: 标识符或关键字
macro_rules! conditions {
    ($feat:meta $($f:expr, $CACHED:ident: $cached:ident, $LIVE:ident: $live:ident),* $(,)?) => (
        #[cfg($feat)]
        #[cfg_attr(feature = "_nightly", doc(cfg($feat)))]
        /// Feature dependent conditions.
        ///
        /// Available when compiled with
        #[doc = concat!('`', stringify!($feat), "`.")]
        impl Condition {
            $(
                /// Evaluates to `true` if
                #[doc = concat!('`', stringify!($f), "`.")]
                ///
                /// The result of the first check is cached for subsequent
                /// checks. Internally uses
                #[doc = concat!("[`", stringify!($cached), "`](Condition::", stringify!($cached), ").")]
                pub const $CACHED: Condition = Condition(Condition::$cached);
            )*

            $(
                /// Evaluates to `true` if
                #[doc = concat!('`', stringify!($f), "`.")]
                ///
                /// A call is dispatched each time the condition is checked.
                /// This is expensive, so prefer to use
                #[doc = concat!("[`", stringify!($CACHED), "`](Condition::", stringify!($CACHED), ")")]
                /// instead.
                ///
                /// Internally uses
                #[doc = concat!("[`", stringify!($live), "`](Condition::", stringify!($live), ").")]
                pub const $LIVE: Condition = Condition(Condition::$live);
            )*

            $(
                /// Returns `true` if
                #[doc = concat!('`', stringify!($f), "`.")]
                ///
                /// The result of the first check is cached for subsequent
                /// checks. This is the backing function for
                #[doc = concat!("[`", stringify!($CACHED), "`](Condition::", stringify!($CACHED), ").")]
                pub fn $cached() -> bool {
                    static IS_TTY: CachedBool = CachedBool::new();
                    IS_TTY.get_or_init(Condition::$live)
                }
            )*

            $(
                /// Returns `true` if
                #[doc = concat!('`', stringify!($f), "`.")]
                ///
                /// This is the backing function for
                #[doc = concat!("[`", stringify!($LIVE), "`](Condition::", stringify!($LIVE), ").")]
                pub fn $live() -> bool {
                    $f
                }
            )*
        }
    )
}

#[cfg(feature = "detect-tty")]
use is_terminal::is_terminal as is_tty;

conditions! { feature = "detect-tty"
    is_tty(&std::io::stdout()),
        STDOUT_IS_TTY: stdout_is_tty,
        STDOUT_IS_TTY_LIVE: stdout_is_tty_live,

    is_tty(&std::io::stderr()),
        STDERR_IS_TTY: stderr_is_tty,
        STDERR_IS_TTY_LIVE: stderr_is_tty_live,

    is_tty(&std::io::stdin()),
        STDIN_IS_TTY: stdin_is_tty,
        STDIN_IS_TTY_LIVE: stdin_is_tty_live,

    is_tty(&std::io::stdout()) && is_tty(&std::io::stderr()),
        STDOUTERR_ARE_TTY: stdouterr_are_tty,
        STDOUTERR_ARE_TTY_LIVE: stdouterr_are_tty_live,
}

// #[cfg(feature = "detect-tty")]
// /// Feature dependent conditions.
// ///
// /// Available when compiled with
// ///`feature = "detect-tty"`.
// impl Condition {
//     /// Evaluates to `true` if
//     ///`is_tty(&std::io::stdout())`.
//     ///
//     /// The result of the first check is cached for subsequent
//     /// checks. Internally uses
//     ///[`stdout_is_tty`](Condition::stdout_is_tty).
//     pub const STDOUT_IS_TTY: Condition = Condition(Condition::stdout_is_tty);
//     /// Evaluates to `true` if
//     ///`is_tty(&std::io::stderr())`.
//     ///
//     /// The result of the first check is cached for subsequent
//     /// checks. Internally uses
//     ///[`stderr_is_tty`](Condition::stderr_is_tty).
//     pub const STDERR_IS_TTY: Condition = Condition(Condition::stderr_is_tty);
//     /// Evaluates to `true` if
//     ///`is_tty(&std::io::stdin())`.
//     ///
//     /// The result of the first check is cached for subsequent
//     /// checks. Internally uses
//     ///[`stdin_is_tty`](Condition::stdin_is_tty).
//     pub const STDIN_IS_TTY: Condition = Condition(Condition::stdin_is_tty);
//     /// Evaluates to `true` if
//     ///`is_tty(&std::io::stdout()) && is_tty(&std::io::stderr())`.
//     ///
//     /// The result of the first check is cached for subsequent
//     /// checks. Internally uses
//     ///[`stdouterr_are_tty`](Condition::stdouterr_are_tty).
//     pub const STDOUTERR_ARE_TTY: Condition =
//         Condition(Condition::stdouterr_are_tty);
//     /// Evaluates to `true` if
//     ///`is_tty(&std::io::stdout())`.
//     ///
//     /// A call is dispatched each time the condition is checked.
//     /// This is expensive, so prefer to use
//     ///[`STDOUT_IS_TTY`](Condition::STDOUT_IS_TTY)
//     /// instead.
//     ///
//     /// Internally uses
//     ///[`stdout_is_tty_live`](Condition::stdout_is_tty_live).
//     pub const STDOUT_IS_TTY_LIVE: Condition =
//         Condition(Condition::stdout_is_tty_live);
//     /// Evaluates to `true` if
//     ///`is_tty(&std::io::stderr())`.
//     ///
//     /// A call is dispatched each time the condition is checked.
//     /// This is expensive, so prefer to use
//     ///[`STDERR_IS_TTY`](Condition::STDERR_IS_TTY)
//     /// instead.
//     ///
//     /// Internally uses
//     ///[`stderr_is_tty_live`](Condition::stderr_is_tty_live).
//     pub const STDERR_IS_TTY_LIVE: Condition =
//         Condition(Condition::stderr_is_tty_live);
//     /// Evaluates to `true` if
//     ///`is_tty(&std::io::stdin())`.
//     ///
//     /// A call is dispatched each time the condition is checked.
//     /// This is expensive, so prefer to use
//     ///[`STDIN_IS_TTY`](Condition::STDIN_IS_TTY)
//     /// instead.
//     ///
//     /// Internally uses
//     ///[`stdin_is_tty_live`](Condition::stdin_is_tty_live).
//     pub const STDIN_IS_TTY_LIVE: Condition =
//         Condition(Condition::stdin_is_tty_live);
//     /// Evaluates to `true` if
//     ///`is_tty(&std::io::stdout()) && is_tty(&std::io::stderr())`.
//     ///
//     /// A call is dispatched each time the condition is checked.
//     /// This is expensive, so prefer to use
//     ///[`STDOUTERR_ARE_TTY`](Condition::STDOUTERR_ARE_TTY)
//     /// instead.
//     ///
//     /// Internally uses
//     ///[`stdouterr_are_tty_live`](Condition::stdouterr_are_tty_live).
//     pub const STDOUTERR_ARE_TTY_LIVE: Condition =
//         Condition(Condition::stdouterr_are_tty_live);
//     /// Returns `true` if
//     ///`is_tty(&std::io::stdout())`.
//     ///
//     /// The result of the first check is cached for subsequent
//     /// checks. This is the backing function for
//     ///[`STDOUT_IS_TTY`](Condition::STDOUT_IS_TTY).
//     pub fn stdout_is_tty() -> bool {
//         static IS_TTY: CachedBool = CachedBool::new();
//         IS_TTY.get_or_init(Condition::stdout_is_tty_live)
//     }
//     /// Returns `true` if
//     ///`is_tty(&std::io::stderr())`.
//     ///
//     /// The result of the first check is cached for subsequent
//     /// checks. This is the backing function for
//     ///[`STDERR_IS_TTY`](Condition::STDERR_IS_TTY).
//     pub fn stderr_is_tty() -> bool {
//         static IS_TTY: CachedBool = CachedBool::new();
//         IS_TTY.get_or_init(Condition::stderr_is_tty_live)
//     }
//     /// Returns `true` if
//     ///`is_tty(&std::io::stdin())`.
//     ///
//     /// The result of the first check is cached for subsequent
//     /// checks. This is the backing function for
//     ///[`STDIN_IS_TTY`](Condition::STDIN_IS_TTY).
//     pub fn stdin_is_tty() -> bool {
//         static IS_TTY: CachedBool = CachedBool::new();
//         IS_TTY.get_or_init(Condition::stdin_is_tty_live)
//     }
//     /// Returns `true` if
//     ///`is_tty(&std::io::stdout()) && is_tty(&std::io::stderr())`.
//     ///
//     /// The result of the first check is cached for subsequent
//     /// checks. This is the backing function for
//     ///[`STDOUTERR_ARE_TTY`](Condition::STDOUTERR_ARE_TTY).
//     pub fn stdouterr_are_tty() -> bool {
//         static IS_TTY: CachedBool = CachedBool::new();
//         IS_TTY.get_or_init(Condition::stdouterr_are_tty_live)
//     }
//     /// Returns `true` if
//     ///`is_tty(&std::io::stdout())`.
//     ///
//     /// This is the backing function for
//     ///[`STDOUT_IS_TTY_LIVE`](Condition::STDOUT_IS_TTY_LIVE).
//     pub fn stdout_is_tty_live() -> bool {
//         is_tty(&std::io::stdout())
//     }
//     /// Returns `true` if
//     ///`is_tty(&std::io::stderr())`.
//     ///
//     /// This is the backing function for
//     ///[`STDERR_IS_TTY_LIVE`](Condition::STDERR_IS_TTY_LIVE).
//     pub fn stderr_is_tty_live() -> bool {
//         is_tty(&std::io::stderr())
//     }
//     /// Returns `true` if
//     ///`is_tty(&std::io::stdin())`.
//     ///
//     /// This is the backing function for
//     ///[`STDIN_IS_TTY_LIVE`](Condition::STDIN_IS_TTY_LIVE).
//     pub fn stdin_is_tty_live() -> bool {
//         is_tty(&std::io::stdin())
//     }
//     /// Returns `true` if
//     ///`is_tty(&std::io::stdout()) && is_tty(&std::io::stderr())`.
//     ///
//     /// This is the backing function for
//     ///[`STDOUTERR_ARE_TTY_LIVE`](Condition::STDOUTERR_ARE_TTY_LIVE).
//     pub fn stdouterr_are_tty_live() -> bool {
//         is_tty(&std::io::stdout()) && is_tty(&std::io::stderr())
//     }
// }

// 当Option对象本身是None时，会将此default作为map_or()的返回值返回出去。
// map_or内部封装了一个match匹配，可以在链式调用中，方便地提取Option中的值：
// 如果本身是Some(t)，那么函数调用结果f(t)将会被作为最终的返回值。
// 如果本身是None，直接返回传给map_or的第一个参数作为最终的返回值。
#[cfg(feature = "detect-env")]
pub fn env_set_or(name: &str, default: bool) -> bool {
    std::env::var_os(name).map_or(default, |v| v != "0")
}

conditions! { feature = "detect-env"
    env_set_or("CLICOLOR_FORCE", false) || env_set_or("CLICOLOR", true),
        CLICOLOR: clicolor,
        CLICOLOR_LIVE: clicolor_live,

    !env_set_or("NO_COLOR", false),
        YES_COLOR: no_color,
        YES_COLOR_LIVE: no_color_live,
}

conditions! { all(feature = "detect-env", feature = "detect-tty")
    Condition::stdouterr_are_tty() && Condition::clicolor() && Condition::no_color(),
        TTY_AND_COLOR: tty_and_color,
        TTY_AND_COLOR_LIVE: tty_and_color_live,
}
