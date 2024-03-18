use clipboard_win::formats::Bitmap;
use clipboard_win::{Clipboard, Getter, Setter};
use core::{mem, ptr};
use image::{
    codecs::png::PngEncoder, GenericImageView, ImageEncoder, ImageFormat,
};
use std::env;
use std::io::Write;
use std::{
    fs,
    io::{BufReader, BufWriter, Cursor},
};
use windows::Win32::Graphics::Gdi::{BITMAPFILEHEADER, BITMAPINFOHEADER};

const INFO_HEADER_LEN: usize = mem::size_of::<BITMAPINFOHEADER>();
const FILE_HEADER_LEN: usize = mem::size_of::<BITMAPFILEHEADER>();

#[test]
fn test_to_png() {
    // 读取bitmpa文件
    let image_path = env::current_dir().unwrap().join("tests/test-image.bmp");
    let test_image_bytes = std::fs::read(image_path).expect("Read test image");

    // 打开剪贴板
    let _clip = Clipboard::new_attempts(10).expect("Open clipboard");
    // 保存到剪贴板模拟截图
    Bitmap.write_clipboard(&test_image_bytes).expect("To set image");

    // 从剪贴板读取截图
    let mut output: Vec<u8> = Vec::new();
    let _ = Bitmap.read_clipboard(&mut output);

    // 获得位图的大小（在栈上申请一块未初始化内存）
    let mut info_header = mem::MaybeUninit::<BITMAPINFOHEADER>::uninit();
    let info_header = unsafe {
        ptr::copy_nonoverlapping(
            output.as_ptr().add(FILE_HEADER_LEN),
            info_header.as_mut_ptr() as _,
            INFO_HEADER_LEN,
        );
        // 把T的所有权返回，编译器会主动对T调用drop
        info_header.assume_init()
    };
    let bitmap_width = info_header.biWidth;
    let bitmap_height = info_header.biHeight;

    assert_eq!(bitmap_width, 750);
    assert_eq!(bitmap_height, 300);

    // 将数组转换为BufReader对象，并读取位图信息
    let buf_reader = BufReader::new(Cursor::new(output));
    let img = image::load(buf_reader, ImageFormat::Bmp).unwrap();

    // 转换为png格式
    let mut png_file_content = Vec::new();
    let buff_writer = BufWriter::new(&mut png_file_content);
    let encoder = PngEncoder::new(buff_writer);
    let _ = encoder.write_image(
        &img.as_bytes().to_vec(),
        img.dimensions().0,
        img.dimensions().1,
        img.color().into(),
    );

    // 保存png文件
    let png_path = env::current_dir().unwrap().join("tests/test-image.png");
    let mut file = fs::File::create(png_path).unwrap();
    file.write_all(png_file_content.as_slice()).unwrap();
}
