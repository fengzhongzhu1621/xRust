use file_hashing::get_hash_file;
use md5::Md5;
use sha1::{Digest, Sha1};
use std::io::Error;
use std::path::Path;

pub fn md5<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let mut hasher = Md5::new();
    get_hash_file(path, &mut hasher)
}

pub fn sha1<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let mut hasher = Sha1::new();
    get_hash_file(path, &mut hasher)
}
