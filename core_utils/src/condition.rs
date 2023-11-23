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
