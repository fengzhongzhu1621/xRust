use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct DebugAdapter<T>
where
    T: fmt::Debug,
{
    pub debug: T,
}

impl<T> DebugAdapter<T>
where
    T: fmt::Debug,
{
    pub fn new(debug: T) -> Self {
        Self { debug }
    }
}

impl<T> fmt::Display for DebugAdapter<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.debug)
    }
}

impl<T> fmt::Debug for DebugAdapter<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug.fmt(f)
    }
}
