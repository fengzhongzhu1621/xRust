use crate::debug::DebugAdapter;
use crate::path::read_file;
use crate::predicates::core::{
    default_find_case, Case, Palette, Parameter, Predicate,
    PredicateReflection,
};
use std::fmt;
use std::io;
use std::path;

/// Predicate that compares file matches
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryFilePredicate {
    path: path::PathBuf,
    content: DebugAdapter<Vec<u8>>,
}

impl BinaryFilePredicate {
    fn eval(&self, path: &path::Path) -> io::Result<bool> {
        let content = read_file(path)?;
        Ok(self.content.debug == content)
    }

    /// Creates a new `Predicate` that ensures complete equality
    pub fn utf8(self) -> Option<StrFilePredicate> {
        let path = self.path;
        // Vec<u8> -> String
        let content = String::from_utf8(self.content.debug).ok()?;
        Some(StrFilePredicate { path, content })
    }
}

impl Predicate<path::Path> for BinaryFilePredicate {
    fn eval(&self, path: &path::Path) -> bool {
        self.eval(path).unwrap_or(false)
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &path::Path,
    ) -> Option<Case<'a>> {
        default_find_case(self, expected, variable)
    }
}

impl Predicate<[u8]> for BinaryFilePredicate {
    fn eval(&self, actual: &[u8]) -> bool {
        self.content.debug == actual
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &[u8],
    ) -> Option<Case<'a>> {
        default_find_case(self, expected, variable)
    }
}

impl PredicateReflection for BinaryFilePredicate {
    fn parameters<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Parameter<'a>> + 'a> {
        let params = vec![Parameter::new("content", &self.content)];
        Box::new(params.into_iter())
    }
}

impl fmt::Display for BinaryFilePredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::new(f.alternate());
        write!(
            f,
            "{} {} {}",
            palette.var("var"),
            palette.description("is"),
            palette.expected(self.path.display())
        )
    }
}

/// Creates a new `Predicate` that ensures complete equality
pub fn eq_file<P: Into<path::PathBuf>>(path: P) -> BinaryFilePredicate {
    let path = path.into();
    let content = DebugAdapter::new(read_file(&path).unwrap());
    BinaryFilePredicate { path, content }
}

/// Predicate that compares string content of files
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrFilePredicate {
    path: path::PathBuf,
    content: String,
}

impl StrFilePredicate {
    fn eval(&self, path: &path::Path) -> Option<bool> {
        let content = read_file(path).ok()?;
        let content = String::from_utf8(content).ok()?;
        Some(self.content == content)
    }
}

impl Predicate<path::Path> for StrFilePredicate {
    fn eval(&self, path: &path::Path) -> bool {
        self.eval(path).unwrap_or(false)
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &path::Path,
    ) -> Option<Case<'a>> {
        default_find_case(self, expected, variable)
    }
}

impl Predicate<str> for StrFilePredicate {
    fn eval(&self, actual: &str) -> bool {
        self.content == actual
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &str,
    ) -> Option<Case<'a>> {
        default_find_case(self, expected, variable)
    }
}

impl PredicateReflection for StrFilePredicate {
    fn parameters<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Parameter<'a>> + 'a> {
        let params = vec![Parameter::new("content", &self.content)];
        Box::new(params.into_iter())
    }
}

impl fmt::Display for StrFilePredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::new(f.alternate());
        write!(
            f,
            "{} {} {}",
            palette.var("var"),
            palette.description("is"),
            palette.expected(self.path.display())
        )
    }
}
