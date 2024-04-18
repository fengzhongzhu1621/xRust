//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use wasm::utils::set_once;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn can_set_once() {
    for _ in 0..10 {
        set_once();
    }
}
