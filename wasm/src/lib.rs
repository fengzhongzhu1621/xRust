extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod utils;
use utils::set_panic_hook;

/// 引用javascript函数
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

/// 在 JavaScript 中使用这个 Rust 函数。
/// 这和 extern 正相反：我们并非引入函数，而是要把函数给外部世界使用。
#[wasm_bindgen]
pub fn greet(name: &str) {
    set_panic_hook();
    alert(&format!("Hello, {}!", name));
}
