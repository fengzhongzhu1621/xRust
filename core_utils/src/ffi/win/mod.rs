#![cfg(windows)]
pub mod clipboard;
mod dc;
mod lock;
mod mem;
pub mod sys;
mod utils;

pub use dc::*;
pub use file::*;
pub use lock::*;
pub use mem::*;
pub use sys::*;
pub use utils::*;

pub use error_code::ErrorCode;
///Alias to result used by this crate
pub type SysResult<T> = Result<T, ErrorCode>;

use super::types::DWORD;

// 用于执行转换的代码页。 此参数可以设置为操作系统中已安装或可用的任何代码页的值。
// https://learn.microsoft.com/zh-cn/windows/win32/intl/code-page-identifiers
pub const CP_UTF8: DWORD = 65001;

// 位图不压缩
pub const BI_RGB: DWORD = 0;
pub const CBM_INIT: DWORD = 0x04;
pub const DIB_RGB_COLORS: DWORD = 0;
pub const ERROR_INCORRECT_SIZE: DWORD = 1462;
