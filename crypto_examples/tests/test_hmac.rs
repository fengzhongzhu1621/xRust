use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use rand::{rngs::OsRng, RngCore};
use rustc_serialize::hex::ToHex;
use std::iter::repeat;

#[test]
fn test_hmac_sha256() {
    let mut key: Vec<u8> = repeat(0u8).take(32).collect();
    OsRng.fill_bytes(&mut key);

    let message = "hello world!";
    let mut hmac = Hmac::new(Sha256::new(), &key);
    hmac.input(message.as_bytes());
    println!("HMAC digest: {}", hmac.result().code().to_hex());
}
