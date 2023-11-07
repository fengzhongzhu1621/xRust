/*
This internal module contains the style and terminal writing implementation.

Its public API is available when the `termcolor` crate is available.
The terminal printing is shimmed when the `termcolor` crate is not available.
*/
// 根据特性开关选择不同的文件加载
#[cfg_attr(feature = "color", path = "extern_impl.rs")]
#[cfg_attr(not(feature = "color"), path = "shim_impl.rs")]
mod imp;

pub use self::imp::*;

#[cfg(feature = "color")]
pub type SubtleStyle = StyledValue<'static, &'static str>;
#[cfg(not(feature = "color"))]
pub type SubtleStyle = &'static str;
