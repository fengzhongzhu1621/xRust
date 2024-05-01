use predicates_tree;
use std::fmt;

pub(crate) struct CaseTree(pub(crate) predicates_tree::CaseTree);

impl fmt::Display for CaseTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <predicates_tree::CaseTree as fmt::Display>::fmt(&self.0, f)
    }
}

// Work around `Debug` not being implemented for `predicates_tree::CaseTree`.
impl fmt::Debug for CaseTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <predicates_tree::CaseTree as fmt::Display>::fmt(&self.0, f)
    }
}
