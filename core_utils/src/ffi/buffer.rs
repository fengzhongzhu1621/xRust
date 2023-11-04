use core::{mem, ptr, marker, slice, fmt, cmp};

pub struct Buffer<'a> {
    ptr: *mut u8,
    len: usize, // 实际存放数据的大小
    capacity: usize,    // 容量
    // 不消耗存储空间，它只是模拟了某种类型的数据，以方便静态分析。
    _lifetime: marker::PhantomData<&'a str>
}

impl<'a> Buffer<'a> {
    #[inline(always)]
    // 获得可用的大小
    pub fn remaining(&self) -> usize {
        // 减法，在整数下溢出时返回 0，而不是让程序崩溃
        self.capacity.saturating_sub(self.len)
    }

    #[inline]
    fn push_data(&mut self, text: &[u8]) -> usize {
        // 每次写的数据长度
        let mut write_len = cmp::min(self.remaining(), text.len());

        #[inline(always)]
        fn is_char_boundary(text: &[u8], idx: usize) -> bool {
            if idx == 0 {
                return true;
            }

            match text.get(idx) {
                None => idx == text.len(),
                Some(&byte) => (byte as i8) >= -0x40
            }
        }

        #[inline(never)]
        #[cold]
        fn shift_by_char_boundary(text: &[u8], mut size: usize) -> usize {
            while !is_char_boundary(text, size) {
                size -= 1;
            }
            size
        }

        if !is_char_boundary(text, write_len) {
            //0 is always char boundary so 0 - 1 is impossible
            write_len = shift_by_char_boundary(text, write_len - 1);
        }

        // ptr::copy_nonoverlapping(src, dest, count)
        // 假设两段内存不会有重合部分，因此速度会略快一点
        // 在语义上等同于 C 的 memcpy ，但交换了参数顺序。
        unsafe {
            ptr::copy_nonoverlapping(text.as_ptr(), self.ptr.add(self.len), write_len);
        }
        self.len += write_len;

        write_len
    }

    #[inline(always)]
    pub fn push_str(&mut self, input: &str) -> usize {
        self.push_data(input.as_bytes())
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &'a [u8] {
        unsafe {
            // 假设传入的指针指向有效的内存，且被指向的内存具有正确的数据类型
            slice::from_raw_parts(self.ptr, self.len)
        }
    }

    #[inline(always)]
    pub fn as_str(&self) -> Option<&'a str> {
        core::str::from_utf8(self.as_slice()).ok()
    }

    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr
    }

    pub unsafe fn set_len(&mut self, len: usize) {
        debug_assert!(len <= self.capacity);
        self.len = len;
    }

}


/// 将字节切片转换为 Buffer 对象
impl<'a> From<&'a mut [u8]> for Buffer<'a> {
    #[inline(always)]
    fn from(this: &'a mut [u8]) -> Self {
        Self {
            ptr: this.as_mut_ptr(), // 将字节切片转换为指针
            len: 0,
            capacity: this.len(),
            _lifetime: marker::PhantomData,
        }
    }
}

/// MaybeUninit 提供了在 GlobalAlloc Trait 之外的一种获取内存的方法， 
/// 实际上可类比为泛型 new () 的一种实现方式，不过返回的不是指针，而是变量。
/// MaybeUninit 获取的内存位于栈空间。
/// 处理还没有完全初始化的内存。
impl<'a> From<&'a mut [mem::MaybeUninit<u8>]> for Buffer<'a> {
    #[inline(always)]
    fn from(this: &'a mut [mem::MaybeUninit<u8>]) -> Self {
        Self {
            ptr: this.as_mut_ptr() as *mut u8,
            len: 0,
            capacity: this.len(),
            _lifetime: marker::PhantomData,
        }
    }
}

impl<'a> fmt::Write for Buffer<'a> {
    #[inline(always)]
    fn write_str(&mut self, input: &str) -> fmt::Result {
        let write_len = self.push_str(input);
        if write_len == input.len() {
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }
}

