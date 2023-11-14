use rand::{rngs::OsRng, RngCore};
use rustc_serialize::base64::{ToBase64, STANDARD};
use std::iter::repeat;

#[test]
fn test_to_base64() {
    // 构造随机字符
    let mut key: Vec<u8> = repeat(0u8).take(32).collect();
    OsRng.fill_bytes(&mut key);

    // 转换为 base64, 例如 hTtygAjRYzc/tNsKHSwJCK3XA8BEcV5LTbYBnHtvK5c=
    println!("{}", key.to_base64(STANDARD));
}
