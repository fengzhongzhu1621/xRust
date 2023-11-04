use core::{mem, ptr, marker, slice, fmt, cmp};

pub struct Buffer<'a> {
    ptr: *mut u8,
    len: usize,
    capacity: usize,
    _lifetime: marker::PhantomData<&'a str>
}
