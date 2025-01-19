#[cfg(test)]
mod test {
    // use super::*;
    use regex::{Captures, Regex};

    #[test]
    /// 清理和标准化输入的文本
    fn test_regex_replace_all() {
        let text =
            "He's going to the park. I'd love to join! They'll meet us there.";

        // 匹配诸如 's, 'd, 'll 等模式
        let pattern1 = Regex::new(r"([''])(s|d|ll)").unwrap();
        // 替换缩写：将常见的英语缩写（如 "he's" → "he is"）替换为完整形式。
        let matched = pattern1.replace_all(text, |capture: &Captures| {
            match &capture[2] {
                "s" => " is",
                "d" => " had",
                "ll" => " will",
                _ => "<unk>",
            }
        });
        // 删除所有非字母和非空格的字符。
        let pattern2 = Regex::new(r"[^a-zA-Z\s]").unwrap();
        let cleaned_text = pattern2.replace_all(&matched, "");

        // 字符串转换为小写
        let output = cleaned_text.to_lowercase();

        println!("{}", output);
    }
}
