use bmp_rust::bmp;
use core_utils::aes::*;
use core_utils::random::*;
use std::{env, fs};

#[test]
fn test_aes128_base64() {
    let key = get_random_key16();
    let iv = generate_iv();

    // 加密
    let plain_text = "1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ！".to_string();
    let cipher_text = aes128_base64_encrypt(&key, &iv, plain_text.as_bytes());
    // 加密
    let output =
        aes128_base64_decrypt(&key, &iv, cipher_text.as_bytes()).unwrap();
    let plain_text_2 = String::from_utf8(output).unwrap();

    assert_eq!(plain_text, plain_text_2);
}

#[test]
fn test_aes128_encrypt_bmp() {
    let key = get_random_key16();
    let iv = generate_iv();

    // 加密
    let path = env::current_dir().unwrap().join("tests/test-image.bmp");
    let bmp_buffer = fs::read(path).unwrap();
    let encrypt_bmp_buffer = aes128_encrypt_bmp(&key, &iv, bmp_buffer);
    let encrypt_path =
        env::current_dir().unwrap().join("tests/encrypted-test-image.bmp");
    fs::write(encrypt_path.as_path(), encrypt_bmp_buffer).unwrap();

    // 解密
    let bmp_buffer = fs::read(encrypt_path).unwrap();
    let decrypt_bmp_buffer = aes128_decrypt_bmp(&key, &iv, bmp_buffer);
    let encrypt_path =
        env::current_dir().unwrap().join("tests/decrypted-test-image.bmp");
    fs::write(encrypt_path, decrypt_bmp_buffer).unwrap();

    // 解密后的位图和原始问题简单比较
    let bmp_from_file_origin = bmp::BMP::new_from_file("tests/test-image.bmp");
    let bmp_from_file_decrypt =
        bmp::BMP::new_from_file("tests/decrypted-test-image.bmp");
    let diff_result =
        bmp::BMP::diff(&bmp_from_file_origin, &bmp_from_file_decrypt).unwrap();
    assert_eq!(diff_result.is_same_size(), true);
    assert_eq!(diff_result.diff.len(), 0);
}
