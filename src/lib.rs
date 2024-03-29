mod utils;

use rand::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
extern crate rand;
extern crate web_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static CELL_SIZE: u32 = 10;
static BOARD_COLOR: &str = "#B8D0EB";
static SNAKE_COLOR: &str = "#26C485";
static APPLE_COLOR: &str = "#FF6666";
static mut DIRECTION: &str = "right";

pub enum Cell {
    Space,
    Snake,
    Apple,
}

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // log(&format!("{}", var));
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
    pub fn init(&mut self) {
        self.game.place_apple();
        self.add_key_bindings();
        self.draw();
    }
    pub fn update(&mut self) {
        unsafe {
            self.game.set_direction(DIRECTION);
        }
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
    fn add_key_bindings(&mut self) {
        let document = web_sys::window().unwrap().document().unwrap();
        let change_direction = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let key: &str = &event.key();
            unsafe {
                match key {
                    "w" | "ArrowUp" => DIRECTION = "up",
                    "a" | "ArrowLeft" => DIRECTION = "left",
                    "s" | "ArrowDown" => DIRECTION = "down",
                    "d" | "ArrowRight" => DIRECTION = "right",
                    _ => (),
                };
            }
        }) as Box<dyn FnMut(_)>);
        document
            .add_event_listener_with_callback("keydown", change_direction.as_ref().unchecked_ref())
            .unwrap();
        change_direction.forget();
    }
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
            snake.push(width + i + 1);
        }
        let apple = (width * height) + 1;
        Game {
            width,
            height,
            snake,
            apple,
            stop,
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
    fn game_over(&mut self) {
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
                if self.apple == position {
                    self.place_apple()
                } else {
                    self.snake.remove(0);
                }
                self.snake.push(position);
            }
            None => self.game_over(),
        };
    }
    fn is_valid_direction(&self, dir: &str) -> bool {
        match (dir, self.direction.clone()) {
            ("left", Direction::Right) => return false,
            ("right", Direction::Left) => return false,
            ("down", Direction::Up) => return false,
            ("up", Direction::Down) => return false,
            _ => (),
        };
        return true;
    }
    pub fn set_direction(&mut self, dir: &str) {
        if !self.is_valid_direction(dir) {
            return;
        }
        match dir {
            "left" => self.direction = Direction::Left,
            "right" => self.direction = Direction::Right,
            "down" => self.direction = Direction::Down,
            "up" => self.direction = Direction::Up,
            _ => (),
        };
    }
    fn place_apple(&mut self) {
        let available_spots = self.get_available_spots();
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0, available_spots.len());
        let random_spot = available_spots[random];
        self.apple = random_spot;
    }
    fn get_available_spots(&self) -> Vec<u32> {
        let mut available_spots = Vec::new();
        for i in 0..(self.width * self.height) {
            if !self.is_snake(i) {
                available_spots.push(i)
            }
        }
        available_spots
    }
}
