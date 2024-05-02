use super::types::DWORD;

// 用于执行转换的代码页。 此参数可以设置为操作系统中已安装或可用的任何代码页的值。
// https://learn.microsoft.com/zh-cn/windows/win32/intl/code-page-identifiers
pub const CP_UTF8: DWORD = 65001;

// 位图不压缩
pub const BI_RGB: DWORD = 0;
pub const CBM_INIT: DWORD = 0x04;
pub const DIB_RGB_COLORS: DWORD = 0;
pub const ERROR_INCORRECT_SIZE: DWORD = 1462;

// According to https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-system_info
// there is a variant for AMD64 CPUs, but it's not defined in generated bindings.
pub const PROCESSOR_ARCHITECTURE_ARM64: u16 = 12;

pub const WAIT_OBJECT_0: DWORD = 0x00000000;
pub const WAIT_TIMEOUT: DWORD = 258;
