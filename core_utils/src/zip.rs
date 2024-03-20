use anyhow::Result;
use std::io::Write;
use std::{fs, path::Path};
use zip::unstable::write::FileOptionsExt;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

/// 使用zip格式压缩文件
pub fn zip_file(key: Vec<u8>, src_path: &Path, dst_path: &Path) -> Result<()> {
    // 创建一个空的zip文件
    let file = fs::File::create(dst_path).unwrap();
    // 设置属性支持加密，使用默认压缩等级
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(CompressionMethod::DEFLATE)
        .with_deprecated_encryption(&key);

    // 添加文件到zip包
    let src_file_name = src_path.file_name().unwrap();
    zip.start_file(src_file_name.to_os_string().to_str().unwrap(), options)
        .unwrap();
    let src_file_content = fs::read(src_path).unwrap();
    zip.write_all(&src_file_content).unwrap();
    zip.finish().unwrap();

    Ok(())
}
