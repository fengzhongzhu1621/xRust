use crate::debug::DebugAdapter;
use crate::predicates::core::{
    default_find_case, Case, Palette, Predicate, PredicateReflection, Product,
};
use std::fmt;

/// 定义比较运算符
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EqOps {
    Equal,
    NotEqual,
}

impl fmt::Display for EqOps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match *self {
            EqOps::Equal => "==",
            EqOps::NotEqual => "!=",
        };
        write!(f, "{}", op)
    }
}

/// Predicate that returns `true` if `variable` matches the pre-defined `Eq`
/// value, otherwise returns `false`.
///
/// This is created by the `predicate::{eq, ne}` functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EqPredicate<T> {
    constant: T,
    op: EqOps, // 运算符
}

impl<P, T> Predicate<P> for EqPredicate<T>
where
    T: std::borrow::Borrow<P> + fmt::Debug,
    P: fmt::Debug + PartialEq + ?Sized,
{
    fn eval(&self, variable: &P) -> bool {
        match self.op {
            EqOps::Equal => variable.eq(self.constant.borrow()),
            EqOps::NotEqual => variable.ne(self.constant.borrow()),
        }
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &P,
    ) -> Option<Case<'a>> {
        default_find_case(self, expected, variable).map(|case| {
            case.add_product(Product::new(
                "var",
                DebugAdapter::new(variable).to_string(),
            ))
        })
    }
}

impl<T> PredicateReflection for EqPredicate<T> where T: fmt::Debug {}

impl<T> fmt::Display for EqPredicate<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::new(f.alternate());
        write!(
            f,
            "{} {} {}",
            palette.var("var"),
            palette.description(self.op),
            palette.expected(DebugAdapter::new(&self.constant)),
        )
    }
}

/// Creates a new predicate that will return `true` when the given `variable` is
/// equal to a pre-defined value.
pub fn eq<T>(constant: T) -> EqPredicate<T>
where
    T: fmt::Debug + PartialEq,
{
    EqPredicate { constant, op: EqOps::Equal }
}
