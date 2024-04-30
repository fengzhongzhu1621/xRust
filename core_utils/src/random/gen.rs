use rand::seq::SliceRandom;
use rand::{self, Rng, SeedableRng};

pub struct Gen {
    rng: rand::rngs::SmallRng, // 伪随机数生成器
    size: usize,
}

impl Gen {
    /// Returns a `Gen` with the given size configuration.
    ///
    /// The `size` parameter controls the size of random values generated.
    /// For example, it specifies the maximum length of a randomly generated
    /// vector, but is and should not be used to control the range of a
    /// randomly generated number. (Unless that number is used to control the
    /// size of a data structure.)
    pub fn new(size: usize) -> Gen {
        Gen { rng: rand::rngs::SmallRng::from_entropy(), size: size }
    }

    /// Returns the size configured with this generator.
    pub fn size(&self) -> usize {
        self.size
    }

    // 在切片中随机选择一个
    /// Choose among the possible alternatives in the slice given. If the slice
    /// is empty, then `None` is returned. Otherwise, a non-`None` value is
    /// guaranteed to be returned.
    pub fn choose<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        slice.choose(&mut self.rng)
    }

    /// 生成一个随机数，类型自行推导
    pub fn gen<T>(&mut self) -> T
    where
        rand::distributions::Standard: rand::distributions::Distribution<T>,
    {
        self.rng.gen()
    }

    /// 从指定范围生成一个随机数
    pub fn gen_range<T, R>(&mut self, range: R) -> T
    where
        T: rand::distributions::uniform::SampleUniform, // 随机结果
        R: rand::distributions::uniform::SampleRange<T>, // 随机范围
    {
        self.rng.gen_range(range)
    }
}
