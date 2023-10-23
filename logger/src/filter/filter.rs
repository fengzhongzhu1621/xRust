use log::{Level, LevelFilter, Metadata, Record};
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::mem;

#[cfg(feature = "regex")]
#[path = "regex.rs"]
mod inner;

#[cfg(not(feature = "regex"))]
#[path = "string.rs"]
mod inner;

#[derive(Debug)]
pub struct Directive {
    name: Option<String>, // 日志的 target
    level: LevelFilter,
}

/// A log filter.
///
/// This struct can be used to determine whether or not a log record
/// should be written to the output.
/// Use the [`Builder`] type to parse and construct a `Filter`.
pub struct Filter {
    pub directives: Vec<Directive>, // 不同target 的日志级别
    pub filter: Option<inner::Filter>,
}

impl Filter {
    /// Returns the maximum `LevelFilter` that this filter instance is
    /// configured to output.
    /// 获得等级最高的日志级别
    pub fn filter(&self) -> LevelFilter {
        self.directives
            .iter()
            .map(|d| d.level)
            .max()
            .unwrap_or(LevelFilter::Off)
    }

    /// Checks if this record matches the configured filter.
    /// 判断日志过滤器是否可用
    pub fn matches(&self, record: &Record) -> bool {
        if !self.enabled(record.metadata()) {
            return false;
        }

        // as_ref 将 Option<inner::Filter> -> Option<&inner::Filter>
        if let Some(filter) = self.filter.as_ref() {
            if !filter.is_match(&record.args().to_string()) {
                return false;
            }
        }

        true
    }

    /// Determines if a log message with the specified metadata would be logged.
    pub fn enabled(&self, metadata: &Metadata) -> bool {
        let level = metadata.level();
        let target = metadata.target();

        // 匹配日志的 target 且 level <= directive.level
        enabled(&self.directives, level, target)
    }
}

// Check whether a level and target are enabled by the set of directives.
pub fn enabled(directives: &[Directive], level: Level, target: &str) -> bool {
    // Search for the longest match, the vector is assumed to be pre-sorted.
    let rev_directives = directives.iter().rev();
    for directive in rev_directives {
        match directive.name {
            // 匹配 log 的 target
            // name: 类型是 &String
            // *name: 类型是 String
            // **name: 定价于 *std::ops::Deref::deref(&s)，因为deref()返回 &str，所以**name 返回 str
            // &**name: 类型是 &str
            Some(ref name) if !target.starts_with(&**name) => {}
            // 根据日志级别过滤
            // 例如 directives 配置了 Info级别的日志, 也就是说最高的日志级别是 Info,
            // 如果Metadata 中的日志级别是 Debug,超过了最高级别Info,日志就会被过滤
            Some(..) | None => return level <= directive.level,
        }
    }
    false
}

impl fmt::Debug for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Filter")
            .field("filter", &self.filter)
            .field("directives", &self.directives)
            .finish()
    }
}

/// A builder for a log filter.
///
/// It can be used to parse a set of directives from a string before building
/// a [`Filter`] instance.
pub struct Builder {
    directives: HashMap<Option<String>, LevelFilter>, //
    filter: Option<inner::Filter>,
    built: bool,
}

/// 构造函数
impl Builder {
    /// Initializes the filter builder with defaults.
    pub fn new() -> Builder {
        Builder { directives: HashMap::new(), filter: None, built: false }
    }
}

/// 根据模块名和级别添加 directive
impl Builder {
    /// Adds a directive to the filter.
    ///
    /// The given module (if any) will log at most the specified level provided.
    /// If no module is provided then the filter will apply to all log messages.
    pub fn filter(
        &mut self,
        module: Option<&str>,
        level: LevelFilter,
    ) -> &mut Self {
        self.directives.insert(module.map(|s| s.to_string()), level);
        self
    }

    /// Adds a directive to the filter for a specific module.
    pub fn filter_module(
        &mut self,
        module: &str,
        level: LevelFilter,
    ) -> &mut Self {
        self.filter(Some(module), level)
    }

    /// Adds a directive to the filter for all modules.
    pub fn filter_level(&mut self, level: LevelFilter) -> &mut Self {
        self.filter(None, level)
    }
}

/// 根据环境变量添加 directive
impl Builder {
    /// Parses the directives string.
    pub fn parse(&mut self, filters: &str) -> &mut Self {
        let (directives, filter) = parse_spec(filters);

        self.filter = filter;

        for directive in directives {
            self.directives.insert(directive.name, directive.level);
        }
        self
    }

    /// Initializes the filter builder from an environment.
    pub fn from_env(env: &str) -> Builder {
        let mut builder = Builder::new();

        if let Ok(s) = env::var(env) {
            builder.parse(&s);
        }

        builder
    }
}

// 用于创建 Filter 对象
impl Builder {
    /// Build a log filter,不能重复生成
    pub fn build(&mut self) -> Filter {
        assert!(!self.built, "attempt to re-use consumed builder");
        self.built = true;

        let mut directives = Vec::new();
        if self.directives.is_empty() {
            // Adds the default filter if none exist
            directives
                .push(Directive { name: None, level: LevelFilter::Error });
        } else {
            // Consume map of directives.
            // mem::take 等价于 mem::replace(name, String::new())
            // 用默认值替换，并且返回原值。
            // take操作需要类型实现Default特性。然而，如果这个类型没有实现Default特性，你还是可以用 mem::replace
            let directives_map = mem::take(&mut self.directives);
            directives = directives_map
                .into_iter()
                .map(|(name, level)| Directive { name, level })
                .collect();
            // 对 directives 根据的 name 名称的字符串长度进行排序
            // Sort the directives by length of their name, this allows a
            // little more efficient lookup at runtime.
            directives.sort_by(|a, b| {
                // 获得名称的长度
                let alen = a.name.as_ref().map(|a| a.len()).unwrap_or(0);
                let blen = b.name.as_ref().map(|b| b.len()).unwrap_or(0);
                alen.cmp(&blen)
            });
        }

        Filter {
            directives: mem::take(&mut directives),
            filter: mem::replace(&mut self.filter, None),
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Builder::new()
    }
}

impl fmt::Debug for Builder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.built {
            f.debug_struct("Filter").field("built", &true).finish()
        } else {
            f.debug_struct("Filter")
                .field("filter", &self.filter)
                .field("directives", &self.directives)
                .finish()
        }
    }
}

/// Parse a logging specification string (e.g: "crate1,crate2::mod3,crate3::x=error/foo")
/// and return a vector with log directives.
fn parse_spec(spec: &str) -> (Vec<Directive>, Option<inner::Filter>) {
    let mut dirs = Vec::new();

    let mut parts = spec.split('/');
    let mods = parts.next();
    let filter = parts.next();
    if parts.next().is_some() {
        eprintln!(
            "warning: invalid logging spec '{}', \
             ignoring it (too many '/'s)",
            spec
        );
        return (dirs, None);
    }
    if let Some(m) = mods {
        for s in m.split(',').map(|ss| ss.trim()) {
            if s.is_empty() {
                continue;
            }
            let mut parts = s.split('=');
            let (log_level, name) = match (
                parts.next(),
                parts.next().map(|s| s.trim()),
                parts.next(),
            ) {
                (Some(part0), None, None) => {
                    // if the single argument is a log-level string or number,
                    // treat that as a global fallback
                    // 将字符串转换为 Result<LevelFilter, Err> 类型
                    match part0.parse() {
                        Ok(level_filter) => (level_filter, None),
                        Err(_) => (LevelFilter::max(), Some(part0)),
                    }
                }
                (Some(part0), Some(""), None) => {
                    (LevelFilter::max(), Some(part0))
                }
                (Some(part0), Some(part1), None) => match part1.parse() {
                    Ok(level_filter) => (level_filter, Some(part0)),
                    _ => {
                        eprintln!(
                            "warning: invalid logging spec '{}', \
                                 ignoring it",
                            part1
                        );
                        continue;
                    }
                },
                _ => {
                    eprintln!(
                        "warning: invalid logging spec '{}', \
                             ignoring it",
                        s
                    );
                    continue;
                }
            };
            dirs.push(Directive {
                name: name.map(|s| s.to_string()),
                level: log_level,
            });
        }
    }

    // 将 Option<&str> 转换为 Filter 对象
    let filter = filter.and_then(|filter| match inner::Filter::new(filter) {
        Ok(re) => Some(re),
        Err(e) => {
            eprintln!("warning: invalid regex filter - {}", e);
            None
        }
    });

    (dirs, filter)
}
