use crypto::aes::{ctr, KeySize};
use crypto_examples::aes::*;
use rand::{rngs::OsRng, RngCore};
use rustc_serialize::base64::{FromBase64, ToBase64, STANDARD};
use std::iter::repeat;
use std::str;

#[test]
fn test_aes256_cbc_pkcs_encrypt() {}

#[test]
fn test_aes128_ctr_encrypt() {
    // 构造加密key
    let mut key: Vec<u8> = repeat(0u8).take(16).collect();
    OsRng.fill_bytes(&mut key);
    let mut iv: Vec<u8> = repeat(0u8).take(16).collect();
    OsRng.fill_bytes(&mut iv);

    let mut cipher = ctr(KeySize::KeySize128, &key, &iv);
    let plain_text = "hello world";
    let mut output: Vec<u8> = repeat(0u8).take(plain_text.len()).collect();
    cipher.process(plain_text.as_bytes(), &mut output[..]);

    println!("{}", output.to_base64(STANDARD));
}

#[test]
fn test_aes256_ctr_encrypt() {
    let key: [u8; 32] = [
        201, 69, 70, 53, 159, 229, 62, 221, 9, 130, 106, 200, 179, 209, 102,
        104, 88, 212, 131, 155, 128, 78, 104, 239, 139, 193, 191, 77, 240, 31,
        35, 226,
    ];
    let iv: [u8; 16] = [
        152, 64, 151, 200, 70, 196, 54, 77, 70, 115, 140, 214, 80, 136, 104,
        93,
    ];

    let data = "hello world!";
    let output = aes256_ctr_encrypt(data.as_bytes(), &key, &iv).unwrap();
    let actual = output.to_base64(STANDARD);
    let expect = "fcUtY7xGZRb3DHPQ";
    assert_eq!(actual, expect);

    let data = expect.from_base64().unwrap();
    let output = aes256_ctr_decrypt(&data, &key, &iv).unwrap();
    let actual = String::from_utf8_lossy(&output);
    let expect = "hello world!";
    assert_eq!(actual, expect);

    let actual = str::from_utf8(output.as_slice()).unwrap();
    assert_eq!(actual, expect);
}

#[test]
fn test_aes256_cbc_encrypt() {
    let key: [u8; 32] = [
        201, 69, 70, 53, 159, 229, 62, 221, 9, 130, 106, 200, 179, 209, 102,
        104, 88, 212, 131, 155, 128, 78, 104, 239, 139, 193, 191, 77, 240, 31,
        35, 226,
    ];
    let iv: [u8; 16] = [
        152, 64, 151, 200, 70, 196, 54, 77, 70, 115, 140, 214, 80, 136, 104,
        93,
    ];

    let data = "hello world!";
    let output = aes256_cbc_encrypt(data.as_bytes(), &key, &iv).unwrap();
    let actual = output.to_base64(STANDARD);
    let expect = "Yzjur7EPjZU+Jt2cMUSxiQ==";
    assert_eq!(actual, expect);

    let data = expect.from_base64().unwrap();
    let output = aes256_cbc_decrypt(&data, &key, &iv).unwrap();
    let actual = String::from_utf8_lossy(&output);
    let expect = "hello world!";
    assert_eq!(actual, expect);

    let actual = str::from_utf8(output.as_slice()).unwrap();
    assert_eq!(actual, expect);
}
