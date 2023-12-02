use core::fmt;
use core::marker::PhantomData;

// PhantomData 不消耗存储空间，它只是模拟了某种类型的数据，以方便静态分析。
// pub(crate) 只在当前 crate 中可见，可以在同一个 crate 中的任何地方访问
// 因为set中只有一个 u16 的元素，没有应用到 T，所以需要一个 PhantomData 来关联 T 类型
pub struct Set<T>(PhantomData<T>, pub(crate) u16);

// 实现位操作的 trait
pub trait SetMember: Copy + fmt::Debug {
    const MAX_VALUE: u8;

    fn bit_mask(self) -> u16;
    fn from_bit_mask(value: u16) -> Option<Self>;
}

// Set 中的元素必须实现了 SetMember trait
impl<T: SetMember> Set<T> {
    /// 默认值
    pub const EMPTY: Self = Set(PhantomData, 0);

    /// 判断是否包含指定位
    pub fn contains(self, value: T) -> bool {
        (value.bit_mask() & self.1) == value.bit_mask()
    }

    /// 为 Set 实现迭代器
    pub const fn iter(self) -> Iter<T> {
        Iter { index: 0, set: self }
    }
}

impl<T: SetMember> Default for Set<T> {
    fn default() -> Self {
        Set::EMPTY
    }
}

impl<T: SetMember> fmt::Debug for Set<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

pub struct Iter<T> {
    index: u8,
    set: Set<T>,
}

impl<T: SetMember> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index <= T::MAX_VALUE {
            let mask: u16 = 1 << self.index;

            self.index += 1;
            // 根据掩码获取需要匹配的值，返回 set 中匹配的值
            if let Some(v) = T::from_bit_mask(mask) {
                if self.set.contains(v) {
                    return Some(v);
                }
            }
        }

        None
    }
}

// 实现 Clone trait
impl<T> Copy for Set<T> {}

impl<T> Clone for Set<T> {
    fn clone(&self) -> Self {
        // 需要包含 PhantomData 元素
        Self(self.0, self.1)
    }
}

impl<T> PartialEq for Set<T> {
    fn eq(&self, other: &Self) -> bool {
        // 不需要比较 PhantomData
        self.1 == other.1
    }
}

impl<T> Eq for Set<T> {}

impl<T> PartialOrd for Set<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl<T> Ord for Set<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

impl<T> core::hash::Hash for Set<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.1.hash(state);
    }
}
