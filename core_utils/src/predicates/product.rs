use std::borrow;
use std::fmt;
use std::slice;

/// A by-product of a predicate evaluation.
///
/// ```rust
/// use predicates_core;
///
/// let product = predicates_core::reflection::Product::new("key", "value");
/// println!("{}", product);
/// let product = predicates_core::reflection::Product::new(format!("key-{}", 5), 30);
/// println!("{}", product);
/// ```
pub struct Product(borrow::Cow<'static, str>, Box<dyn fmt::Display>);

impl Product {
    /// Create a new `Product`.
    pub fn new<S, D>(key: S, value: D) -> Self
    where
        S: Into<borrow::Cow<'static, str>>,
        D: fmt::Display + 'static,
    {
        // key 是str的写时克隆指针
        // value 是可打印对象
        Self(key.into(), Box::new(value))
    }

    /// Access the `Product` name.
    pub fn name(&self) -> &str {
        self.0.as_ref() // as_ref() 转换为 &str 类型
    }

    /// Access the `Product` value.
    pub fn value(&self) -> &dyn fmt::Display {
        &self.1
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

impl fmt::Debug for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {})", self.0, self.1)
    }
}

/// Iterator over a `Case`s by-products.
#[derive(Debug, Clone)]
pub struct CaseProducts<'a>(pub(crate) slice::Iter<'a, Product>);

impl<'a> Iterator for CaseProducts<'a> {
    type Item = &'a Product;

    fn next(&mut self) -> Option<&'a Product> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // 返回类型是一个元组，该元组表示迭代器剩余长度的边界信息。
        self.0.size_hint()
    }

    fn count(self) -> usize {
        self.0.count()
    }
}
