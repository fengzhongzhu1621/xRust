pub mod types;
pub mod utils;
pub mod mem;
pub mod sys;
pub mod buffer;

pub use error_code::ErrorCode;
///Alias to result used by this crate
pub type SysResult<T> = Result<T, ErrorCode>;
