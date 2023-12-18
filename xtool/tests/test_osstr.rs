use std::ffi::OsStr;
use std::ffi::OsString;

#[test]
fn test_osstr_len() {
    let str1 = OsStr::new("1你好2");
    assert_eq!(str1.len(), 8);
}

#[test]
#[cfg(windows)]
fn test_osstr_encode_wide() {
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    let str1 = OsStr::new("1你好2");
    let vec_u16: Vec<u16> = str1.encode_wide().collect();
    assert_eq!(vec_u16, vec![49, 20320, 22909, 50]);
    assert_eq!(vec_u16, "1你好2".encode_utf16().collect::<Vec<u16>>());

    // 计算长度
    let size = "1你".encode_utf16().collect::<Vec<u16>>().len();
    assert_eq!(size, 2);

    assert_eq!(
        OsString::from_wide(&"1你".encode_utf16().collect::<Vec<u16>>()),
        OsStr::new("1你").to_os_string()
    );

    // 转换为 &[u16]
    let bytes_u16 = vec_u16.as_slice();
    println!("{:?}", bytes_u16);

    // 比较
    assert_eq!(vec_u16, "1你好2".encode_utf16().collect::<Vec<u16>>());

    // split_off
    let str = OsStr::new("1你好2");
    let mut vec_u16_mut: Vec<u16> = str.encode_wide().collect();
    let right_vec_u16_mut = vec_u16_mut.split_off(2);
    assert_eq!(vec_u16_mut, "1你".encode_utf16().collect::<Vec<u16>>());
    assert_eq!(right_vec_u16_mut, "好2".encode_utf16().collect::<Vec<u16>>());
    // panic
    // let _ = vec_u16_mut.split_off(10);

    // split_at
    let str = OsStr::new("1你好2");
    let vec_u16: Vec<u16> = str.encode_wide().collect();
    let (left, right) = vec_u16.split_at(2);
    assert_eq!(left, "1你".encode_utf16().collect::<Vec<u16>>().as_slice());
    assert_eq!(right, "好2".encode_utf16().collect::<Vec<u16>>().as_slice());
    // panic
    // let _ = vec_u16_mut.split_at(10);

    // splitn
    let str = OsStr::new("前缀_时间戳_hash_文件名");
    let vec_u16: Vec<u16> = str.encode_wide().collect();
    let value: Vec<&[u16]> =
        vec_u16.splitn(4, |num| *num == '_' as u16).into_iter().collect();
    assert_eq!(
        value,
        vec![
            vec![21069, 32512],
            vec![26102, 38388, 25139],
            vec![104, 97, 115, 104],
            vec![25991, 20214, 21517]
        ]
    );
    assert_eq!(value[3], vec![25991, 20214, 21517]);
    let value: Vec<&[u16]> =
        vec_u16.splitn(4, |num| *num == '|' as u16).into_iter().collect();
    assert_eq!(
        value,
        vec![[
            21069, 32512, 95, 26102, 38388, 25139, 95, 104, 97, 115, 104, 95,
            25991, 20214, 21517
        ]]
    );
}

#[test]
fn test_join() {
    let str1 = OsStr::new("1");
    let str2 = OsStr::new("你好");
    let str3 = OsStr::new("2");

    let joind = [str1, str2, str3].join(OsStr::new(""));
    assert_eq!(joind, OsStr::new("1你好2"));
}
