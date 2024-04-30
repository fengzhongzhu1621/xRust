use std::iter::{empty, once};

/// Creates a shrinker with zero elements.
pub fn empty_shrinker<A: 'static>() -> Box<dyn Iterator<Item = A>> {
    Box::new(empty())
}

/// Creates a shrinker with a single element.
pub fn single_shrinker<A: 'static>(value: A) -> Box<dyn Iterator<Item = A>> {
    Box::new(once(value))
}
