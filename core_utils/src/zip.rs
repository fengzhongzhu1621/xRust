use anyhow::Result;
use std::io::Write;
use std::{fs, path::Path};
use walkdir::WalkDir;
use zip::unstable::write::FileOptionsExt;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};
use zip_extensions::zip_create_from_directory_with_options;

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

/// 使用zip格式压缩文件文件夹，并返回原文件夹的大小
pub fn zip_directory(
    key: Vec<u8>,
    source_dir: &Path,
    archive_file: &Path,
    max_depth: usize,
) -> Result<u64> {
    let mut total_size: u64 = 0;
    // 计算文件夹的大小
    for metadata in WalkDir::new(source_dir)
        .min_depth(1)
        .max_depth(max_depth)
        .into_iter()
        // 忽略正在运行的进程或无权访问的目录
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        // 只计算文件
        .filter(|metadata| metadata.is_file())
    {
        total_size += metadata.len();
    }

    // 压缩文件夹
    let options = FileOptions::default()
        .compression_method(CompressionMethod::DEFLATE)
        .with_deprecated_encryption(&key);
    zip_create_from_directory_with_options(
        &archive_file.to_path_buf(),
        &source_dir.to_path_buf(),
        options,
    )
    .unwrap();

    Ok(total_size)
}
