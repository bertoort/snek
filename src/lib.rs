mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub enum Cell {
    Space,
    Snake,
    Apple,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[wasm_bindgen]
pub struct Board {
    width: u32,
    height: u32,
    snake: Vec<u32>,
    apple: u32,
    direction: Direction,
}

#[wasm_bindgen]
impl Board {
    pub fn new(width: u32, height: u32) -> Board {
        let width = width;
        let height = height;
        let mut snake = Vec::new();
        let apple = (width * height) - height - 1;
        Board {
            width,
            height,
            snake,
            apple,
            direction: Direction::Right,
        }
    }
}
