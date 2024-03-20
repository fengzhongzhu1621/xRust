use core_utils::random::get_random_key16;
use core_utils::zip::zip_file;
use std::env;

#[test]
fn test_zip_file() {
    let src_file_path = env::current_dir().unwrap().join("tests/data.txt");
    let dst_file_path = env::current_dir().unwrap().join("tests/data.zip");
    let key = get_random_key16();
    let _ = zip_file(key.to_vec(), &src_file_path, &dst_file_path);
}
