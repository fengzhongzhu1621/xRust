use super::product::{CaseProducts, Product};
use super::relection::PredicateReflection;
use std::fmt;
use std::slice;

/// 描述断言为什么失败
/// A descriptive explanation for why a predicate failed.
pub struct Case<'a> {
    predicate: Option<&'a dyn PredicateReflection>, // 断言
    result: bool,                                   // 断言结果
    products: Vec<Product>,                         // 包含多个意外结果
    children: Vec<Case<'a>>,                        // case 嵌套
}

impl<'a> Case<'a> {
    /// Create a new `Case` describing the result of a `Predicate`.
    pub fn new(
        predicate: Option<&'a dyn PredicateReflection>,
        result: bool,
    ) -> Self {
        Self {
            predicate,                    // 断言
            result,                       // 断言结果
            products: Default::default(), // 包含多个意外结果
            children: Default::default(), // case 嵌套
        }
    }

    /// Add an additional by product to a `Case`.
    pub fn add_product(mut self, product: Product) -> Self {
        self.products.push(product);
        self
    }

    /// Add an additional by product to a `Case`.
    pub fn add_child(mut self, child: Case<'a>) -> Self {
        self.children.push(child);
        self
    }

    /// The `Predicate` that produced this case.
    pub fn predicate(&self) -> Option<&dyn PredicateReflection> {
        self.predicate
    }

    /// The result of this case.
    pub fn result(&self) -> bool {
        self.result
    }

    /// Access the by-products from determining this case.
    pub fn products(&self) -> CaseProducts<'_> {
        CaseProducts(self.products.iter())
    }

    /// Access the sub-cases.
    pub fn children(&self) -> CaseChildren<'_> {
        CaseChildren(self.children.iter())
    }
}

/// Iterator over a `Case`s sub-cases.
#[derive(Debug, Clone)]
pub struct CaseChildren<'a>(slice::Iter<'a, Case<'a>>);

impl<'a> Iterator for CaseChildren<'a> {
    type Item = &'a Case<'a>;

    fn next(&mut self) -> Option<&'a Case<'a>> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    fn count(self) -> usize {
        self.0.count()
    }
}

impl<'a> fmt::Debug for Case<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let predicate = if let Some(ref predicate) = self.predicate {
            format!("Some({})", predicate)
        } else {
            "None".to_owned()
        };
        f.debug_struct("Case")
            .field("predicate", &predicate)
            .field("result", &self.result)
            .field("products", &self.products)
            .field("children", &self.children)
            .finish()
    }
}
