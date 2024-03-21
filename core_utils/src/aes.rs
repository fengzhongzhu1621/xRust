use base64::{engine::general_purpose, Engine as _};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer, symmetriccipher};

use std::mem;
use windows::Win32::Graphics::Gdi::{BITMAPFILEHEADER, BITMAPINFOHEADER};

// 计算位图的文件头长度和位图信息头长度
const FILE_HEADER_LEN: usize = mem::size_of::<BITMAPFILEHEADER>();
const INFO_HEADER_LEN: usize = mem::size_of::<BITMAPINFOHEADER>();

pub fn aes128_cbc_encrypt(
    data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut encryptor = aes::cbc_encryptor(
        aes::KeySize::KeySize128,
        key,
        iv,
        blockmodes::PkcsPadding,
    );

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result =
            encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;

        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

pub fn aes128_cbc_decrypt(
    encrypted_data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize128,
        key,
        iv,
        blockmodes::PkcsPadding,
    );

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result =
            decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

pub fn aes128_base64_encrypt(
    key: &[u8],
    iv: &[u8],
    plain_text: &[u8],
) -> String {
    let output = aes128_cbc_encrypt(plain_text, &key, &iv).unwrap();
    general_purpose::STANDARD.encode(&output)
}

pub fn aes128_base64_decrypt(
    key: &[u8],
    iv: &[u8],
    cipher_text: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let data = general_purpose::STANDARD.decode(cipher_text).unwrap();
    aes128_cbc_decrypt(&data, &key, &iv)
}

pub fn aes128_encrypt_bmp(
    key: &[u8],
    iv: &[u8],
    bmp_buffer: Vec<u8>,
) -> Vec<u8> {
    // 计算位图头长度
    let header_len = FILE_HEADER_LEN + INFO_HEADER_LEN;

    // 获得位图的像素数据
    let bmp_header = &bmp_buffer[..header_len];
    let bmp_pixel_data = &bmp_buffer[header_len..];

    // 只加密像素部分，保留头部不变
    let encrypt_bmp_pixel_data =
        aes128_cbc_encrypt(bmp_pixel_data, &key, &iv).unwrap();

    // 生成新的位图
    let mut encrypt_bmp_buffer = Vec::new();
    encrypt_bmp_buffer.extend_from_slice(bmp_header);
    encrypt_bmp_buffer.extend_from_slice(&encrypt_bmp_pixel_data);

    encrypt_bmp_buffer
}

pub fn aes128_decrypt_bmp(
    key: &[u8],
    iv: &[u8],
    bmp_buffer: Vec<u8>,
) -> Vec<u8> {
    let header_len = FILE_HEADER_LEN + INFO_HEADER_LEN;
    let bmp_header = &bmp_buffer[..header_len];
    let bmp_pixel_data = &bmp_buffer[header_len..];

    // 解密像素部分
    let decrypt_bmp_pixel_data =
        aes128_cbc_decrypt(bmp_pixel_data, &key, &iv).unwrap();

    // 生成原位图
    let mut decrypt_bmp_buffer = Vec::new();
    decrypt_bmp_buffer.extend_from_slice(bmp_header);
    decrypt_bmp_buffer.extend_from_slice(&decrypt_bmp_pixel_data);

    decrypt_bmp_buffer
}
