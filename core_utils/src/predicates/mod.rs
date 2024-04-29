pub mod core;
pub mod float;
pub mod path;
pub mod str;

mod boolean;
mod boxed;
mod eq;

pub use boolean::{always, never, BooleanPredicate, PredicateBooleanExt};
pub use boxed::{BoxPredicate, PredicateBoxExt};
pub use eq::{eq, EqPredicate};
