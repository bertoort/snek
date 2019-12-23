mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
extern crate web_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static CELL_SIZE: u32 = 10;
static BOARD_COLOR: &str = "#B8D0EB";
static SNAKE_COLOR: &str = "#6F2DBD";
static APPLE_COLOR: &str = "#FF6666";

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
    context: web_sys::CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Board {
    pub fn new(element: &str, width: u32, height: u32) -> Board {
        let mut snake = Vec::new();
        for i in 0..4 {
            snake.push(height + i + 1);
        }
        let apple = (width * height) - height - 2;
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas").unwrap();
        let board = document.get_element_by_id(element).unwrap();
        board.append_child(&canvas).unwrap();

        let canvas = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        canvas.set_height((CELL_SIZE + 1) * height + 1);
        canvas.set_width((CELL_SIZE + 1) * width + 1);
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Board {
            width,
            height,
            snake,
            apple,
            context,
            direction: Direction::Right,
        }
    }
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    fn is_snake(&self, idx: u32) -> bool {
        for i in self.snake.clone() {
            if idx == i {
                return true;
            }
        }
        return false;
    }
    pub fn draw(&self) {
        self.context.begin_path();

        for row in 0..self.width {
            for col in 0..self.height {
                let idx = self.get_index(row, col);
                let mut color = BOARD_COLOR;
                if idx == self.apple as usize {
                    color = APPLE_COLOR;
                }
                if self.is_snake(idx as u32) {
                    color = SNAKE_COLOR;
                }
                self.context.set_fill_style(&JsValue::from(color));

                self.context.fill_rect(
                    (col * (CELL_SIZE + 1) + 1) as f64,
                    (row * (CELL_SIZE + 1) + 1) as f64,
                    CELL_SIZE as f64,
                    CELL_SIZE as f64,
                )
            }
        }

        self.context.stroke()
    }
}
