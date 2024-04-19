/// 可以理解为用枚举实现了一个策略模型，每个枚举值都是一种策略
/// An implementation to match on simple strings.
#[derive(Debug, Clone)]
pub enum Matcher {
    /// Considers the entire string (trimmed) to be the match.
    /// 去掉字符串两端的空白字符
    AllTrimmed,

    // 查找前缀匹配的第一个单词
    /// After finding the `prefix` followed by one or more spaces, returns the following word.
    PrefixedWord { prefix: &'static str },

    // 查找前缀匹配的第一个单词（是版本字符串）
    /// Similar to `PrefixedWord`, but only if the word is a valid version.
    PrefixedVersion { prefix: &'static str },

    //  如果字符串是 key1=value1\nkey=value，则返回Some(value)
    /// Takes a set of lines (separated by `\n`) and searches for the value in a key/value pair
    /// separated by the `=` character. For example `VERSION_ID="8.1"`.
    KeyValue { key: &'static str },
}

impl Matcher {
    /// Find the match on the input `string`.
    pub fn find(&self, string: &str) -> Option<String> {
        match *self {
            Self::AllTrimmed => Some(string.trim().to_string()),
            Self::PrefixedWord { prefix } => {
                // 查找前缀匹配的第一个单词
                find_prefixed_word(string, prefix).map(str::to_owned)
            }
            Self::PrefixedVersion { prefix } => {
                find_prefixed_word(string, prefix)
                    .filter(|&v| is_valid_version(v))
                    .map(str::to_owned)
            }
            Self::KeyValue { key } => {
                find_by_key(string, key).map(str::to_owned)
            }
        }
    }
}

/// 如果字符串是 key=value或 key="value"格式化，则返回Some(value)
fn find_by_key<'a>(string: &'a str, key: &str) -> Option<&'a str> {
    let key = [key, "="].concat();
    // 将字符串按换行符分隔
    for line in string.lines() {
        // 字符串开头匹配 key=
        if line.starts_with(&key) {
            // 去掉空白字符、去掉双引号
            return Some(
                line[key.len()..]
                    .trim_matches(|c: char| c == '"' || c.is_whitespace()),
            );
        }
    }

    None
}

/// 查找前缀匹配的第一个单词
fn find_prefixed_word<'a>(string: &'a str, prefix: &str) -> Option<&'a str> {
    // 查找子串
    if let Some(prefix_start) = string.find(prefix) {
        // Ignore prefix and leading whitespace
        // 去掉前缀和空白字符
        let string = &string[prefix_start + prefix.len()..].trim_start();

        // Find where the word boundary ends
        // 查找字符串中是否包含空白字符
        let word_end =
            string.find(|c: char| c.is_whitespace()).unwrap_or(string.len());
        // 获取第一个单词
        let string = &string[..word_end];

        Some(string)
    } else {
        None
    }
}

/// 判断字符串两边是否有 .
fn is_valid_version(word: &str) -> bool {
    !word.starts_with('.') && !word.ends_with('.')
}
