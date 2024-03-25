/// 将字符串按指定长度分隔
pub fn chunks(s: String, n: usize) -> Vec<String> {
    let sub_strings = s
        .chars()
        .collect::<Vec<char>>()
        .chunks(n)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>();

    sub_strings
}
