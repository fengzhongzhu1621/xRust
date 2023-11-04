
pub struct Scope<T: Copy>(pub T, pub fn(T));

impl<T: Copy> Drop for Scope<T> {
    #[inline(always)]
    fn drop(&mut self) {
        (self.1)(self.0)
    }
}


pub struct RawMem(Scope<*mut c_void>);