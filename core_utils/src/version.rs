use std::fmt::{self, Display, Formatter};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Operating system version.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Version {
    /// Unknown version.
    Unknown,
    /// Semantic version (major.minor.patch).
    Semantic(u64, u64, u64),
    /// Rolling version. Optionally contains the release date in the string format.
    Rolling(Option<String>),
    /// Custom version format.
    Custom(String),
}

impl Version {
    /// Constructs `VersionType` from the given string.
    pub fn from_string<S: Into<String> + AsRef<str>>(s: S) -> Self {
        // AsRef<str> 表示 s 可以通过 .as_ref()转换为 &str 类型
        if s.as_ref().is_empty() {
            Self::Unknown
        } else if let Some((major, minor, patch)) = parse_version(s.as_ref()) {
            Self::Semantic(major, minor, patch)
        } else {
            // Into<String> 说明 s 可以通过into()转换为 String 类型
            Self::Custom(s.into())
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Version::Unknown
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Self::Unknown => f.write_str("Unknown"),
            Self::Semantic(major, minor, patch) => {
                write!(f, "{major}.{minor}.{patch}")
            }
            Self::Rolling(ref date) => {
                let date = match date {
                    Some(date) => format!(" ({date})"),
                    None => "".to_owned(),
                };
                write!(f, "Rolling Release{date}")
            }
            Self::Custom(ref version) => write!(f, "{version}"),
        }
    }
}

pub fn parse_version(s: &str) -> Option<(u64, u64, u64)> {
    // 将字符串按 . 分隔，并返回一个迭代器
    // fuse 适配器可以接受任意迭代器，并生成一个一旦第一次返回 None，就一定会继续返回 None 的迭代器
    let mut iter = s.trim().split_terminator('.').fuse();

    // 非None则转换为整数
    let major = iter.next().and_then(|s| s.parse().ok())?;
    // 非None则转换为整数，为None则默认为0
    let minor = iter.next().unwrap_or("0").parse().ok()?;
    // 非None则转换为整数，为None则默认为0
    let patch = iter.next().unwrap_or("0").parse().ok()?;

    if iter.next().is_some() {
        return None;
    }

    Some((major, minor, patch))
}
