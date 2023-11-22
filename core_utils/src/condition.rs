use core::sync::atomic::AtomicPtr;

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
        crate::ffi::console::cache_enable()
    }
}

impl Default for Condition {
    fn default() -> Self {
        Condition::DEFAULT
    }
}

#[repr(transparent)]
pub struct AtomicCondition(AtomicPtr<()>);
