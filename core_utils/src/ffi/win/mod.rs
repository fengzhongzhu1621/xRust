mod dc;
mod clipboard;
mod sys;
mod utils;
mod lock;
mod mem;

pub use sys::*;
pub use dc::*;
pub use clipboard::*;
pub use utils::*;
pub use lock::*;
pub use mem::*;

pub use error_code::ErrorCode;
///Alias to result used by this crate
pub type SysResult<T> = Result<T, ErrorCode>;
