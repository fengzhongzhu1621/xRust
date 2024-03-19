use rand::seq::SliceRandom;
use rand::Rng;
use rand_core::{OsRng, RngCore};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
abcdefghijklmnopqrstuvwxyz\
0123456789)(*&^%$#@!~";

pub fn random_string(len: usize) -> String {
    let mut rng = rand::thread_rng();
    let password: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    return password;
}

pub fn random_string_2(size: usize) -> String {
    let mut rng = &mut rand::thread_rng();
    String::from_utf8(
        CHARSET.choose_multiple(&mut rng, size).cloned().collect(),
    )
    .unwrap()
}

/// 生成随机 iv
pub fn generate_iv() -> [u8; 16] {
    let mut rng = OsRng;
    let mut bytes = [0u8; 16];
    rng.fill_bytes(&mut bytes);

    bytes
}

/// 生成随机 iv
pub fn generate_iv_2() -> [u8; 16] {
    let mut rng = OsRng::default();
    let mut bytes = [0u8; 16];
    rng.fill_bytes(&mut bytes);

    bytes
}
