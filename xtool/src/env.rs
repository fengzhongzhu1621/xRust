#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    /// Do not run the test
    Skip,
    /// Ignore test failures
    Ignore,
    /// Fail on mismatch
    Verify,
    /// Overwrite on mismatch
    Overwrite,
}

impl Action {
    /// 将环境变量的值转换为枚举类型
    pub fn with_env_var(var: impl AsRef<std::ffi::OsStr>) -> Option<Self> {
        let var = var.as_ref();
        // 从环境变量根据key取值
        let value = std::env::var_os(var)?;
        // 将环境变量的值转换为枚举类型
        Self::with_env_value(value)
    }

    /// 将字符串的值转换为枚举类型
    pub fn with_env_value(value: impl AsRef<std::ffi::OsStr>) -> Option<Self> {
        let value = value.as_ref();
        match value.to_str()? {
            "skip" => Some(Action::Skip),
            "ignore" => Some(Action::Ignore),
            "verify" => Some(Action::Verify),
            "overwrite" => Some(Action::Overwrite),
            _ => None,
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Self::Verify
    }
}
