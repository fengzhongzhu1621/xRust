pub mod core;
pub mod float;
pub mod path;
pub mod str;

mod boolean;
mod boxed;
mod eq;
mod function;
mod iter;
mod name;

pub use boolean::{always, never, BooleanPredicate, PredicateBooleanExt};
pub use boxed::{BoxPredicate, PredicateBoxExt};
pub use eq::{eq, EqPredicate};
pub use function::{function, FnPredicate};
pub use iter::{
    in_hash, in_iter, HashableInPredicate, InPredicate, OrdInPredicate,
};
pub use name::{NamePredicate, PredicateNameExt};
