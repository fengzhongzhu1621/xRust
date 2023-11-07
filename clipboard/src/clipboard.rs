use core_utils::ffi::win::*;

pub struct Clipboard {
    _dummy: ()
}

impl Clipboard {
    #[inline(always)]
    ///Attempts to open clipboard, returning clipboard instance on success.
    pub fn new() -> SysResult<Self> {
        open().map(|_| Self { _dummy: () })
    }


}
