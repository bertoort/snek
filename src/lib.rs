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
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Canvas {
    game: Game,
    context: web_sys::CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Canvas {
    pub fn new(element: &str, game: Game) -> Canvas {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas").unwrap();
        let board = document.get_element_by_id(element).unwrap();
        board.append_child(&canvas).unwrap();

        let canvas = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        canvas.set_height((CELL_SIZE + 1) * &game.height + 1);
        canvas.set_width((CELL_SIZE + 1) * &game.width + 1);
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        Canvas { game, context }
    }
    pub fn init(&self) {
        self.add_key_bindings();
        self.draw();
    }
    pub fn update(&mut self) {
        self.game.tick();
        self.draw();
    }
    fn draw(&self) {
        self.context.begin_path();

        for row in 0..self.game.width {
            for col in 0..self.game.height {
                let idx = self.game.get_index(row, col);
                let mut color = BOARD_COLOR;
                if idx == self.game.apple as usize {
                    color = APPLE_COLOR;
                }
                if self.game.is_snake(idx as u32) {
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
    fn add_key_bindings(&self) {}
}

#[wasm_bindgen]
pub struct Game {
    width: u32,
    height: u32,
    snake: Vec<u32>,
    apple: u32,
    stop: bool,
    direction: Direction,
}

#[wasm_bindgen]
impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let mut snake = Vec::new();
        let stop = false;
        for i in 0..4 {
            snake.push(height + (width * i) + 10);
        }
        let apple = (width * height) - height - 2;
        Game {
            width,
            height,
            snake,
            apple,
            stop,
            direction: Direction::Left,
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
    fn game_over(&mut self) {
        log(&format!("Game over: {}", self.stop));
        self.stop = true;
    }
    fn next_position(&self) -> Option<u32> {
        let head = self.snake.last().unwrap();
        let hit_top_wall = match self.direction {
            Direction::Up => &self.width > head,
            Direction::Left => head == &(0 as u32),
            _ => false,
        };
        if hit_top_wall {
            return None;
        }
        let next = match self.direction {
            Direction::Right => head + 1,
            Direction::Left => head - 1,
            Direction::Up => head - self.width,
            Direction::Down => head + self.width,
        };
        let overlap = self.is_snake(next);
        let wrap_right = head % self.width == self.width - 1 && next % self.width == 0;
        let wrap_left = head % self.width == 0 && next % self.width == self.width - 1;
        let bottom_wall = next > self.width * self.height;
        if overlap || wrap_right || wrap_left || bottom_wall {
            return None;
        }
        Some(next)
    }
    pub fn tick(&mut self) {
        if self.stop {
            return;
        }
        match self.next_position() {
            Some(position) => {
                self.snake.remove(0);
                self.snake.push(position);
            }
            None => self.game_over(),
        }
    }
}
