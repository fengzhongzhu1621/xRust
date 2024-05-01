use super::{EqCodePredicate, InCodePredicate};
use predicates_core;

pub trait IntoCodePredicate<P>
where
    P: predicates_core::Predicate<i32>,
{
    /// The type of the predicate being returned.
    type Predicate;

    /// Convert to a predicate for testing a program's exit code.
    fn into_code(self) -> P;
}

/// 默认实现
impl<P> IntoCodePredicate<P> for P
where
    P: predicates_core::Predicate<i32>,
{
    type Predicate = P;

    fn into_code(self) -> Self::Predicate {
        self
    }
}

/// 类型转换 i32 -> EqCodePredicate
impl IntoCodePredicate<EqCodePredicate> for i32 {
    // 实现了 impl predicates_core::Predicate<i32> for EqCodePredicate {
    type Predicate = EqCodePredicate;

    fn into_code(self) -> Self::Predicate {
        Self::Predicate::new(self)
    }
}

/// 类型转换 Vec<i32> -> InCodePredicate
impl IntoCodePredicate<InCodePredicate> for Vec<i32> {
    // 实现了 impl predicates_core::Predicate<i32> for InCodePredicate
    type Predicate = InCodePredicate;

    fn into_code(self) -> Self::Predicate {
        Self::Predicate::new(self)
    }
}

/// 类型转换 &'static [i32] -> InCodePredicate
impl IntoCodePredicate<InCodePredicate> for &'static [i32] {
    // 实现了 impl predicates_core::Predicate<i32> for InCodePredicate
    type Predicate = InCodePredicate;

    fn into_code(self) -> Self::Predicate {
        Self::Predicate::new(self.iter().cloned())
    }
}
