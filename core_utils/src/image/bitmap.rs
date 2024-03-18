use image::{
    codecs::png::PngEncoder, GenericImageView, ImageEncoder, ImageFormat,
    ImageResult,
};
use std::{
    fs,
    io::{BufReader, BufWriter},
    path::Path,
};

/// 将bitmap位图转换为png格式
pub fn to_png(bitmap_path: &Path, png_path: &Path) -> ImageResult<()> {
    // 读取位图文件
    let bitmap_fs = fs::File::open(bitmap_path).expect("Read bitmap");
    let buf_reader = BufReader::new(bitmap_fs);
    let img = image::load(buf_reader, ImageFormat::Bmp).unwrap();

    // 创建png空文件
    let png_file = fs::File::create(png_path)?;
    let ref mut buff = BufWriter::new(png_file);
    let encoder = PngEncoder::new(buff);

    // 转换并写png文件
    encoder.write_image(
        &img.as_bytes().to_vec(),
        img.dimensions().0,
        img.dimensions().1,
        img.color().into(),
    )
}
