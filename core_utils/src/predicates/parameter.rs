use std::fmt;

pub struct Parameter<'a>(&'a str, &'a dyn fmt::Display);

impl<'a> Parameter<'a> {
    /// Create a new `Parameter`.
    /// key: 参数名，字符串类型
    /// value: 参数值，可以打印的对象
    pub fn new(key: &'a str, value: &'a dyn fmt::Display) -> Self {
        Self(key, value)
    }

    /// Access the `Parameter` name.
    pub fn name(&self) -> &str {
        self.0
    }

    /// Access the `Parameter` value.
    pub fn value(&self) -> &dyn fmt::Display {
        self.1
    }
}

impl<'a> fmt::Display for Parameter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

impl<'a> fmt::Debug for Parameter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {})", self.0, self.1)
    }
}
