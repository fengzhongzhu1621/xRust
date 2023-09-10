pub struct Numbers<'data> {
    // 结构体的生命周期 <= data
    data: &'data Vec<i32>,
    even_idx: usize,
}

impl<'data> Numbers<'data> {
    pub fn new(data: &'data Vec<i32>) -> Self {
        Self { data, even_idx: 0 }
    }

    /// 返回偶数
    pub fn next_even<'numbers>(&'numbers mut self) -> Option<&'data i32> {
        while let Some(x) = self.get(self.even_idx) {
            self.even_idx += 1;
            if *x % 2 == 0 {
                // 返回偶数
                return Some(x);
            }
        }
        None
    }

    /// 根据索引返回向量的值，注意Numers的生命周期和返回结果的生命周期不一样
    fn get<'numbers>(&'numbers self, idx: usize) -> Option<&'data i32> {
        if idx < self.data.len() {
            Some(&self.data[idx])
        } else {
            None
        }
    }
}
