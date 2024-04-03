#[cfg(windows)]
#[cfg(not(feature = "std"))]
#[test]
fn test_error() {
    use core_utils::platform::unix::lib_c::Error;
    let e = Error::from_raw_os_error(99);
    assert_eq!(e.code, 99);
}
