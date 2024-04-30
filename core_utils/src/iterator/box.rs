use std::iter::{empty, once};

/// Creates a shrinker with zero elements.
pub fn empty_shrinker<A: 'static>() -> Box<dyn Iterator<Item = A>> {
    Box::new(empty())
}

/// Creates a shrinker with a single element.
pub fn single_shrinker<A: 'static>(value: A) -> Box<dyn Iterator<Item = A>> {
    Box::new(once(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_shrinker() {
        let mut iter = empty_shrinker::<&str>();
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_single_shrinker() {
        let mut iter = single_shrinker("test");
        assert_eq!(iter.next(), Some("test"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let mut iter =
            single_shrinker(None).chain(single_shrinker(1).map(Some));
        assert_eq!(iter.next(), Some(None));
        assert_eq!(iter.next(), Some(Some(1)));
        assert_eq!(iter.next(), None);
    }
}
