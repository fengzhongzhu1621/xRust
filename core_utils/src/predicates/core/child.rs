use super::reflection::PredicateReflection;
use std::fmt;

pub struct Child<'a>(&'a str, &'a dyn PredicateReflection);

impl<'a> Child<'a> {
    /// Create a new `Predicate` child.
    pub fn new(key: &'a str, value: &'a dyn PredicateReflection) -> Self {
        Self(key, value)
    }

    /// Access the `Child`'s name.
    pub fn name(&self) -> &str {
        self.0
    }

    /// Access the `Child` `Predicate`.
    pub fn value(&self) -> &dyn PredicateReflection {
        self.1
    }
}

impl<'a> fmt::Display for Child<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

impl<'a> fmt::Debug for Child<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {})", self.0, self.1)
    }
}
