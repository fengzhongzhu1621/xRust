/// 记录当前代码的行数和文件路径信息
pub fn record_code_position() {
    let caller = std::panic::Location::caller();

    println!("caller = {}", caller);
}
