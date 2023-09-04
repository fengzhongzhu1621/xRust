use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub fn hash_u8(value: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(&value);
    format!("{:x}", hasher.finish())
}
