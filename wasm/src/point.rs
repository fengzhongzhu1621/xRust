use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point {
    x: u32,
    y: u32,
}

#[wasm_bindgen]
impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Self { x, y }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    /// 移动点的位置
    pub fn move_by(&mut self, dx: u32, dy: u32) {
        self.x += dx;
        self.y += dy;
    }
}
