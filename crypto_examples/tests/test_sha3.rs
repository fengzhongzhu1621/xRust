use crypto::digest::Digest;
use crypto::sha3::Sha3;
use rustc_hex::{FromHex, ToHex};

#[test]
fn test_sha3_256() {
    let input = "hello world!";
    let mut hasher = Sha3::sha3_256();

    hasher.input_str(input);

    let hex = hasher.result_str();
    let expect =
        "9c24b06143c07224c897bac972e6e92b46cf18063f1a469ebe2f7a0966306105";
    assert_eq!(hex, expect);

    let hello_str: Vec<u8> = hex.from_hex().unwrap();
    let res = hello_str.as_slice();
    let expected: &[u8] = &[
        156, 36, 176, 97, 67, 192, 114, 36, 200, 151, 186, 201, 114, 230, 233,
        43, 70, 207, 24, 6, 63, 26, 70, 158, 190, 47, 122, 9, 102, 48, 97, 5,
    ];

    assert_eq!(res, expected);
}
