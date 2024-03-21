use core_utils::aes::*;
use core_utils::random::*;

#[test]
fn test_aes128_base64_encrypt() {
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
