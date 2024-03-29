use core_utils::var::Var;
use std::borrow::Cow;

/// The default name for the environment variable to read filters from.
pub const DEFAULT_FILTER_ENV: &str = "RUST_LOG";

/// The default name for the environment variable to read style preferences from.
pub const DEFAULT_WRITE_STYLE_ENV: &str = "RUST_LOG_STYLE";

/// Set of environment variables to configure from.
#[derive(Debug)]
pub struct Env<'a> {
    filter: Var<'a>, // 日志的过滤级别，从环境变了获取
    write_style: Var<'a>,   // 日志类型，从环境变了获取
}

impl<'a> Env<'a> {
    /// Get a default set of environment variables.
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify an environment variable to read the filter from.
    pub fn filter<E>(mut self, filter_env: E) -> Self
    where
        E: Into<Cow<'a, str>>,
    {
        self.filter = Var::new(filter_env);

        self
    }

    /// Specify an environment variable to read the filter from.
    ///
    /// If the variable is not set, the default value will be used.
    pub fn filter_or<E, V>(mut self, filter_env: E, default: V) -> Self
    where
        E: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.filter = Var::new_with_default(filter_env, default);

        self
    }

    /// Use the default environment variable to read the filter from.
    ///
    /// If the variable is not set, the default value will be used.
    pub fn default_filter_or<V>(mut self, default: V) -> Self
    where
        V: Into<Cow<'a, str>>,
    {
        self.filter = Var::new_with_default(DEFAULT_FILTER_ENV, default);

        self
    }

    /// 从环境变量获取 name 的值
    pub fn get_filter(&self) -> Option<String> {
        self.filter.get()
    }

    /// Specify an environment variable to read the style from.
    pub fn write_style<E>(mut self, write_style_env: E) -> Self
    where
        E: Into<Cow<'a, str>>,
    {
        self.write_style = Var::new(write_style_env);

        self
    }

    /// Specify an environment variable to read the style from.
    ///
    /// If the variable is not set, the default value will be used.
    pub fn write_style_or<E, V>(
        mut self,
        write_style_env: E,
        default: V,
    ) -> Self
    where
        E: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.write_style = Var::new_with_default(write_style_env, default);

        self
    }

    /// Use the default environment variable to read the style from.
    ///
    /// If the variable is not set, the default value will be used.
    pub fn default_write_style_or<V>(mut self, default: V) -> Self
    where
        V: Into<Cow<'a, str>>,
    {
        self.write_style =
            Var::new_with_default(DEFAULT_WRITE_STYLE_ENV, default);

        self
    }

    pub fn get_write_style(&self) -> Option<String> {
        self.write_style.get()
    }
}

/// 设置默认环境变量
impl<'a> Default for Env<'a> {
    fn default() -> Self {
        Env {
            filter: Var::new(DEFAULT_FILTER_ENV),
            write_style: Var::new(DEFAULT_WRITE_STYLE_ENV),
        }
    }
}

impl<'a, T> From<T> for Env<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(filter_env: T) -> Self {
        Env::default().filter(filter_env.into())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn env_get_filter_reads_from_var_if_set() {
        env::set_var("env_get_filter_reads_from_var_if_set", "from var");

        let env = Env::new().filter_or("env_get_filter_reads_from_var_if_set", "from default");

        assert_eq!(Some("from var".to_owned()), env.get_filter());
    }

    #[test]
    fn env_get_filter_reads_from_default_if_var_not_set() {
        env::remove_var("env_get_filter_reads_from_default_if_var_not_set");

        let env = Env::new().filter_or(
            "env_get_filter_reads_from_default_if_var_not_set",
            "from default",
        );

        assert_eq!(Some("from default".to_owned()), env.get_filter());
    }

    #[test]
    fn env_get_write_style_reads_from_var_if_set() {
        env::set_var("env_get_write_style_reads_from_var_if_set", "from var");

        let env =
            Env::new().write_style_or("env_get_write_style_reads_from_var_if_set", "from default");

        assert_eq!(Some("from var".to_owned()), env.get_write_style());
    }

    #[test]
    fn env_get_write_style_reads_from_default_if_var_not_set() {
        env::remove_var("env_get_write_style_reads_from_default_if_var_not_set");

        let env = Env::new().write_style_or(
            "env_get_write_style_reads_from_default_if_var_not_set",
            "from default",
        );

        assert_eq!(Some("from default".to_owned()), env.get_write_style());
    }
}
