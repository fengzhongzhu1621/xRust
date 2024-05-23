/// 裸指针，又称原始指针，生命周期不受封装对象控制，不遵循 Rust 的所有权和借用规则。
/// 
/// 包含如下两种
/// * mut T
/// * const T
/// 
/// * 裸指针没有生命周期，不保证指向有效的内存
/// * 裸指针可以为空
/// * 接引用裸指针需要在 unsafe 块或 unsafe 函数中进行
/// 

/// 测试定义
#[test]
fn test_convert() {
    let x = 5;
    let _ = &x as *const i32;
}

/// 测试 offset
#[test]
fn test_offset() {
    let x = 5;
    let y = &x as *const i32;

    unsafe {
        // 打印指针的值
        println!("y = {:p}", y);
        // 对指针进行算数计算，计算 y 指针偏移 4 个字节后的地址
        // 注意：这个地址可能指向非法的或非初始化的内存快
        print!("y.offset(1) = {:p}", y.offset(1));
    }
}

/// 测试解引用
#[test]
fn test_deref() {
    let x = 5;
    let y = &x as *const i32;

    unsafe {
        println!("*y = {}", *y);
        assert_eq!(*y, 5);

        // 演示目的，禁止此种用法，此地址指向了非法的或非初始化的内存快
        println!("*y.offset(1) = {}", *y.offset(1));
    }
}