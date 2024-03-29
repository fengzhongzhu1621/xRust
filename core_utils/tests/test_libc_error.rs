use core_utils::ffi::lib_c::Error;

#[cfg(not(feature = "std"))]
#[test]
fn test_error() {
    let e = Error::from_raw_os_error(99);
    assert_eq!(e.code, 99);
}
