// 根据特性引用不同的文件
#[cfg_attr(feature = "humantime", path = "extern_impl.rs")]
mod imp;

// 限制在项目内的 fmt 模块内可见
pub(in crate::fmt) use self::imp::*;
