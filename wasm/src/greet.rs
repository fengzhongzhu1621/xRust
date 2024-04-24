use wasm_bindgen::prelude::*;

// wasm_bindgen 负责将Rust数据类型转换为JavaScript可以理解的类型。
// [wasm_bindgen]表明了下面的代码可以在 js 和 rust 中访问。

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn console_log(name: &str) {
    // 调用JavaScript的console.log函数
    log(&format!("Hello, {}!", name));
}
