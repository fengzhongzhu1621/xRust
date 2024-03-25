#![cfg(windows)]
use anyhow::Result;
use windows::{core::*, Win32::System::Registry::*};

/// 查询windows注册表的值
fn query_reg_value() -> Result<String> {
    unsafe {
        // 打开注册表
        let mut key = HKEY::default();
        RegOpenKeyExA(
            HKEY_LOCAL_MACHINE,
            s!(r"SOFTWARE\GitForWindows"),
            0,
            KEY_QUERY_VALUE,
            &mut key,
        )?;

        // 获得 value 的字节数
        let mut len = 0;
        RegQueryValueExA(
            key,
            s!("InstallPath"),
            None,
            None,
            None,
            Some(&mut len),
        )?;

        // 获取 value 的值
        let mut buffer = vec![0u8; (len) as usize];
        RegQueryValueExA(
            key,
            s!("InstallPath"),
            None,
            None,
            Some(buffer.as_mut_ptr() as _),
            Some(&mut len),
        )?;

        // 转换为字符串
        let value = String::from_utf8_lossy(&buffer);

        // 去掉结尾的空字符
        let value = value.trim_end_matches("\0");

        Ok(value.to_string())
    }
}

#[test]
fn test_query_reg_value() {
    let actual = query_reg_value().unwrap();
    let expect = r"C:\Program Files\Git".to_string();
    assert_eq!(actual, expect);
}
