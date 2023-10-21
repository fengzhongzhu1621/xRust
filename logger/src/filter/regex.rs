/// 导入外部库, 使用 self::regex 引用此库
extern crate regex;

use std::fmt;

use self::regex::Regex;

#[derive(Debug)]
pub struct Filter {
    inner: Regex,
}

impl Filter {
    pub fn new(spec: &str) -> Result<Filter, String> {
        match Regex::new(spec) {
            Ok(r) => Ok(Filter { inner: r }),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn is_match(&self, s: &str) -> bool {
        // 支持正则表达式匹配
        self.inner.is_match(s)
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}
