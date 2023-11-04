#[cold]
#[inline(never)]
// 返回默认值
pub fn unlikely_empty_size_result<T: Default>() -> T {
    Default::default()
}
