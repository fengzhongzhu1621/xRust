use rand::{rngs::OsRng, RngCore};
use std::iter::repeat;

/// 构造随机字符
#[test]
fn test_fill_bytes() {
    let mut key: Vec<u8> = repeat(0u8).take(32).collect();
    OsRng.fill_bytes(&mut key);
    println!("{:?}", key);
}
