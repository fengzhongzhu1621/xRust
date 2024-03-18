use core_utils::image::bitmap;
use std::env;

#[test]
fn test_to_png() {
    // 读取bitmpa文件
    let bitmap_path = env::current_dir().unwrap().join("tests/test-image.bmp");

    // 保存png文件
    let png_path = env::current_dir().unwrap().join("tests/test-image.png");
    bitmap::to_png(bitmap_path.as_path(), png_path.as_path()).unwrap();
}
