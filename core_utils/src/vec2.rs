/// 二维数组
pub struct Vec2<T> {
    len: [usize; 2], // n * m 大小的二维数组
    data: Vec<T>,
}

impl<T> Vec2<T> {
    #[inline]
    pub fn new(value: T, len: [usize; 2]) -> Self
    where
        T: Clone,
    {
        // value 是二维数组中元素的初始值
        Vec2 { len, data: vec![value; len[0] * len[1]] }
    }
}

impl<T> Vec2<T> {
    /// 根据二维索引获得值的引用
    #[inline]
    pub fn get(&self, index: [usize; 2]) -> &T {
        debug_assert!(index[0] < self.len[0]);
        debug_assert!(index[1] < self.len[1]);
        &self.data[index[0] * self.len[1] + index[1]]
    }

    /// 根据二维索引设置值
    #[inline]
    pub fn set(&mut self, index: [usize; 2], value: T) {
        debug_assert!(index[0] < self.len[0]);
        debug_assert!(index[1] < self.len[1]);
        self.data[index[0] * self.len[1] + index[1]] = value;
    }
}
