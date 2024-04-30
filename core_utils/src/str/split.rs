/// 将字符串按指定长度分隔
pub fn chunks(s: String, n: usize) -> Vec<String> {
    let sub_strings = s
        .chars() // 转换为字符迭代器
        .collect::<Vec<char>>() // 转换为 Vec<char>
        .chunks(n) // 按指定长度分割
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>();

    sub_strings
}

#[cfg(test)]
mod tests {
    use super::chunks;

    #[test]
    fn test_chunks() {
        let s = "你好, hello！".to_string();
        let actual = chunks(s, 2);
        let expect: Vec<String> = vec![
            "你好".to_string(),
            ", ".to_string(),
            "he".to_string(),
            "ll".to_string(),
            "o！".to_string(),
        ];
        assert_eq!(actual, expect);
    }
}
