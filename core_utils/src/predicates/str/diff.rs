use crate::predicates::core::{
    Case, Palette, Parameter, Predicate, PredicateReflection, Product,
};
use difflib;
use std::borrow;
use std::fmt;

/// Predicate that diffs two strings.
///
/// This is created by the `predicate::str::diff`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DifferencePredicate {
    orig: borrow::Cow<'static, str>,
}

impl Predicate<str> for DifferencePredicate {
    fn eval(&self, edit: &str) -> bool {
        edit == self.orig
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &str,
    ) -> Option<Case<'a>> {
        let result = variable != self.orig;
        if result == expected {
            None
        } else {
            let palette = Palette::new(true);
            let orig: Vec<_> =
                self.orig.lines().map(|l| format!("{}\n", l)).collect();
            let variable: Vec<_> =
                variable.lines().map(|l| format!("{}\n", l)).collect();
            // 按行比较多行字符串
            let diff = difflib::unified_diff(
                &orig,
                &variable,
                "",
                "",
                &palette.expected("orig").to_string(),
                &palette.var("var").to_string(),
                0,
            );
            let mut diff = colorize_diff(diff, palette);
            diff.insert(0, "\n".to_owned());

            Some(
                Case::new(Some(self), result)
                    .add_product(Product::new("diff", diff.join(""))),
            )
        }
    }
}

impl PredicateReflection for DifferencePredicate {
    fn parameters<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Parameter<'a>> + 'a> {
        let params = vec![Parameter::new("original", &self.orig)];
        Box::new(params.into_iter())
    }
}

impl fmt::Display for DifferencePredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::new(f.alternate());
        write!(
            f,
            "{:#} {:#} {:#}",
            palette.description("diff"),
            palette.expected("original"),
            palette.var("var"),
        )
    }
}

#[cfg(feature = "color")]
fn colorize_diff(mut lines: Vec<String>, palette: Palette) -> Vec<String> {
    for (i, line) in lines.iter_mut().enumerate() {
        match (i, line.as_bytes().first()) {
            (0, _) => {
                if let Some((prefix, body)) = line.split_once(' ') {
                    *line = format!("{:#} {}", palette.expected(prefix), body);
                }
            }
            (1, _) => {
                if let Some((prefix, body)) = line.split_once(' ') {
                    *line = format!("{:#} {}", palette.var(prefix), body);
                }
            }
            (_, Some(b'-')) => {
                let (prefix, body) = line.split_at(1);
                *line = format!("{:#}{}", palette.expected(prefix), body);
            }
            (_, Some(b'+')) => {
                let (prefix, body) = line.split_at(1);
                *line = format!("{:#}{}", palette.var(prefix), body);
            }
            (_, Some(b'@')) => {
                *line = format!("{:#}", palette.description(&line));
            }
            _ => (),
        }
    }
    lines
}

pub fn diff<S>(orig: S) -> DifferencePredicate
where
    S: Into<borrow::Cow<'static, str>>,
{
    DifferencePredicate { orig: orig.into() }
}
