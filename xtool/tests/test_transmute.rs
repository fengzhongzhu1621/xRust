use std::mem::transmute;

// 用于进行类型转换的 unsafe 函数，它能够将一个 A 类型的变量的 底层数据 直接视为另一个 B 类型的数据，以此将 A 类型转换为 B 类型
#[test]
fn test_point_to_func() {
    fn foo() -> i32 {
        0
    }
    let pointer = foo as *const ();
    let function =
        unsafe { std::mem::transmute::<*const (), fn() -> i32>(pointer) };
    assert_eq!(function(), 0);
}

// &[u8] => u32
#[test]
fn test_refu8_to_u32() {
    let raw_bytes = [0x78, 0x56, 0x34, 0x12];

    let num = unsafe { std::mem::transmute::<[u8; 4], u32>(raw_bytes) };
    assert_eq!(num, 0x12345678);

    // use `u32::from_ne_bytes` instead
    let num = u32::from_ne_bytes(raw_bytes);
    assert_eq!(num, 0x12345678);

    // or use `u32::from_le_bytes` or `u32::from_be_bytes` to specify the endianness
    let num = u32::from_le_bytes(raw_bytes);
    assert_eq!(num, 0x12345678);

    let num = u32::from_be_bytes(raw_bytes);
    assert_eq!(num, 0x78563412);
}

#[test]
fn test_to_usize() {
    let ptr = &0;
    let ptr_num_transmute = unsafe { std::mem::transmute::<&i32, usize>(ptr) };
    // assert_eq!(ptr_num_transmute, 4537590148);

    // Use an `as` cast instead
    let ptr_num_cast = ptr as *const i32 as usize;
    // assert_eq!(ptr_num_cast, 0);
}

// *mut T =>  &mut T
#[test]
fn test_ref() {
    let ptr: *mut i32 = &mut 0;
    let ref_transmuted =
        unsafe { std::mem::transmute::<*mut i32, &mut i32>(ptr) };
    assert_eq!(ref_transmuted, &mut 0);

    // Use a reborrow instead
    let ref_casted = unsafe { &mut *ptr };
    assert_eq!(ref_casted, &mut 0);
}

// &mut T => &mut U
#[test]
fn test_mut_ref() {
    let ptr = &mut 0;
    let val_transmuted =
        unsafe { std::mem::transmute::<&mut i32, &mut u32>(ptr) };
    assert_eq!(val_transmuted, &mut 0);

    // Now, put together `as` and reborrowing - note the chaining of `as`
    // `as` is not transitive
    let val_casts = unsafe { &mut *(ptr as *mut i32 as *mut u32) };
    assert_eq!(val_casts, &mut 0);
}

// &str => &[u8]
#[test]
fn test_str_to_refu8() {
    // this is not a good way to do this.
    let slice = unsafe { std::mem::transmute::<&str, &[u8]>("Rust") };
    assert_eq!(slice, &[82, 117, 115, 116]);

    // You could use `str::as_bytes`
    let slice = "Rust".as_bytes();
    assert_eq!(slice, &[82, 117, 115, 116]);

    // Or, just use a byte string, if you have control over the string
    // literal
    assert_eq!(b"Rust", &[82, 117, 115, 116]);
}
