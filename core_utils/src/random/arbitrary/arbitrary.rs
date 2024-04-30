use crate::iterator::{empty_shrinker, single_shrinker};
use crate::random::Gen;

pub trait Arbitrary: Clone + 'static {
    /// 构造函数
    /// Return an arbitrary value.
    fn arbitrary(g: &mut Gen) -> Self;

    /// Return an iterator of values that are smaller than itself.
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        empty_shrinker()
    }
}

// 创建一个 Arbitrary 对象
pub fn arby<A: Arbitrary>(size: usize) -> A {
    Arbitrary::arbitrary(&mut Gen::new(size))
}

impl Arbitrary for () {
    fn arbitrary(_: &mut Gen) -> () {
        ()
    }
}

impl Arbitrary for bool {
    fn arbitrary(g: &mut Gen) -> bool {
        g.gen()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = bool>> {
        // true > false
        if *self {
            // 如果是 true，则返回一个迭代器，下一个值是 false
            single_shrinker(false)
        } else {
            // 如果是 false，则返回一个空的迭代器
            empty_shrinker()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn arby_unit() {
        assert_eq!(arby::<()>(5), ());
    }

    #[test]
    fn test_bool() {
        let x = arby::<bool>(5);
        let _ = x.shrink();
    }
}
