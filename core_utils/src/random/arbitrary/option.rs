use crate::iterator::{empty_shrinker, single_shrinker};
use crate::random::{Arbitrary, Gen};

impl<A: Arbitrary> Arbitrary for Option<A> {
    fn arbitrary(g: &mut Gen) -> Option<A> {
        // None or Some(true) or Some(false)
        if g.gen() {
            None
        } else {
            Some(Arbitrary::arbitrary(g)) // bool 类型
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Option<A>>> {
        // Some(true) > None > Some(false) > None
        match *self {
            None => empty_shrinker(),
            Some(ref x) => {
                let chain = single_shrinker(None).chain(x.shrink().map(Some));
                Box::new(chain)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::random::arby;

    #[test]
    fn test_option() {
        let x = arby::<Option<bool>>(5);
        println!("{:#?}", x);
        let x = arby::<Option<bool>>(5);
        println!("{:#?}", x);
        let x = arby::<Option<bool>>(5);
        println!("{:#?}", x);
    }
}
