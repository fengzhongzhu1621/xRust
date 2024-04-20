use super::child::Child;
use super::parameter::Parameter;
use std::fmt;

/// Introspect the state of a `Predicate`.
/// 一个断言包含一个paramter和多个child，每个child又包含一个paramter和多个child
pub trait PredicateReflection: fmt::Display {
    /// Parameters of the current `Predicate`.
    fn parameters<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Parameter<'a>> + 'a> {
        let params = vec![];
        // 迭代器的值是 Paramter 类型
        Box::new(params.into_iter())
    }

    /// Nested `Predicate`s of the current `Predicate`.
    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = Child<'a>> + 'a> {
        let params = vec![];
        // 通过into_iter()调用得到的迭代器，其中迭代的是params数组中元素本身（占据所有权）。
        // 调用 params.into_iter()之后，params变量的所有权会被转移走，无法再次使用params。
        // 在堆上给迭代器分配空间】
        // 迭代器的值是 Child 类型
        Box::new(params.into_iter())
    }
}
