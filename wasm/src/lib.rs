mod greet;
mod point;
mod utils;

pub use greet::*;
pub use point::*;
pub use utils::*;

use console_error_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}
