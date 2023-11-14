use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[test]
fn test_sha256() {
    let input = "hello world!";
    let mut sha = Sha256::new();

    sha.input_str(input);

    let actual = sha.result_str();
    let expect =
        "7509e5bda0c762d2bac7f90d758b5b2263fa01ccbc542ab5e3df163be08e6ca9";
    assert_eq!(actual, expect);
}
