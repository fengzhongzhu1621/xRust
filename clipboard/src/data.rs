use core_utils::ffi::win::SysResult;
use core_utils::ffi::win::{unlikely_empty_size_result, RawMem};

use core::{mem, ptr, cmp};

///Copies raw bytes from clipboard with specified `format`
///
///Returns number of copied bytes on success, otherwise 0.
///
///It is safe to pass uninit memory
pub fn get(format: u32, out: &mut [u8]) -> SysResult<usize> {
    let size = out.len();
    if size == 0 {
        return Ok(unlikely_empty_size_result());
    }
    let out_ptr = out.as_mut_ptr();

    let ptr = RawMem::from_borrowed(get_clipboard_data(format)?);

    let result = unsafe {
        let (data_ptr, _lock) = ptr.lock()?;
        let data_size = cmp::min(GlobalSize(ptr.get()) as usize, size);
        ptr::copy_nonoverlapping(data_ptr.as_ptr() as *const u8, out_ptr, data_size);
        data_size
    };

    Ok(result)
}