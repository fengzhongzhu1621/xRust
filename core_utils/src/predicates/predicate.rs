use super::case::Case;
use super::reflection::PredicateReflection;

/// Trait for generically evaluating a type against a dynamically created
/// predicate function.
///
/// The exact meaning of `eval` depends on the situation, but will usually
/// mean that the evaluated item is in some sort of pre-defined set.  This is
/// different from `Ord` and `Eq` in that an `item` will almost never be the
/// same type as the implementing `Predicate` type.
pub trait Predicate<Item: ?Sized>: PredicateReflection {
    /// Execute this `Predicate` against `variable`, returning the resulting
    /// boolean.
    /// 执行断言，返回断言的结果
    fn eval(&self, variable: &Item) -> bool;

    /// Find a case that proves this predicate as `expected` when run against `variable`.
    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &Item,
    ) -> Option<Case<'a>> {
        // 返回断言的执行结果
        let actual = self.eval(variable);
        if expected == actual {
            Some(Case::new(None, actual))
        } else {
            None
        }
    }
}
