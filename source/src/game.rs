extern crate serde;

use serde::Serialize;

use std::f64;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;

use crate::point::Point;
use crate::tetrimino::Tetrimino;

#[derive(Serialize, Clone)]
#[wasm_bindgen]
pub struct Game {
    board: Board,
    current_tetrimino: Tetrimino,
    next_tetrimino: Tetrimino,
    static_blocks: Vec<Point>,
}
#[derive(Serialize, Clone, Copy)]
pub struct Board {
    rows: i32,
    cols: i32,
}
#[wasm_bindgen]
impl Board {}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        Self {
            board: Board { rows: 16, cols: 12 },
            current_tetrimino: Tetrimino::create_random_tetrimino(),
            next_tetrimino: Tetrimino::create_random_tetrimino(), // FIXME: remove this field after fixing E0277, the trait `RefFromWasmAbi` is not implemented for `Point`
            static_blocks: vec![],
        }
    }

    pub fn is_tetrimino_in_bounds(&mut self) -> bool {
        self.next_tetrimino
            .cells
            .iter()
            .all(|Point { x: i, y: j }| {
                0 <= *i && *i < self.board.cols && 0 <= *j && *j < self.board.rows
            })
    }

    pub fn is_colliding(&mut self) -> bool {
        let vec1 = &self.next_tetrimino.cells;
        let vec2 = &self.static_blocks;
        let has_common_point = vec1.iter().any(|&point1| vec2.contains(&point1));
        has_common_point
    }

    pub fn move_right(&mut self) {
        self.current_tetrimino.center.x += 1;
        self.current_tetrimino.cells = self
            .current_tetrimino
            .cells
            .iter_mut()
            .map(|Point { x: i, y: j }| Point { x: *i + 1, y: *j })
            .collect();
        self.draw_board();
    }

    pub fn move_left(&mut self) {
        self.current_tetrimino.center.x -= 1;
        self.current_tetrimino.cells = self
            .current_tetrimino
            .cells
            .iter_mut()
            .map(|Point { x: i, y: j }| Point { x: *i - 1, y: *j })
            .collect();
        self.draw_board();
    }

    pub fn rotate(&mut self) {
        self.next_tetrimino = self.current_tetrimino.rotate();
        if !self.is_colliding() && self.is_tetrimino_in_bounds() {
            self.current_tetrimino = self.current_tetrimino.rotate(); // FIXME: called twice, need to impl copy trait for Point
            self.draw_board();
        }
    }

    pub fn move_down(&mut self) {
        self.next_tetrimino = self.current_tetrimino.move_down();
        if !self.is_colliding() && self.is_tetrimino_in_bounds() {
            self.current_tetrimino = self.current_tetrimino.move_down(); // FIXME: called twice, need to impl copy trait for Point
        } else {
            self.current_tetrimino.cells.iter().for_each(|p| {
                self.static_blocks.push(*p);
            });
            self.current_tetrimino = Tetrimino::create_random_tetrimino();
        }
        self.draw_board();
    }

    pub fn tick(&mut self) {
        self.move_down();
    }

    pub fn draw_board(&mut self) {
        let id = "myCanvas".to_string();
        let document = web_sys::window().unwrap().document().unwrap();

        let canvas = document
            .get_element_by_id(&id)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        canvas.set_width(1000);
        canvas.set_height(1600);

        let context: web_sys::CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.begin_path();

        for row in 0..16 {
            for col in 0..10 {
                let delta: f64 = 100 as f64;
                let box_size: f64 = 100 as f64;
                let tile_cells = &self.current_tetrimino.cells;
                let fixed_cells = &self.static_blocks;
                println!("tile_cells is {:?}", &tile_cells);
                let json_str = serde_json::to_string(&self).unwrap();
                let js_value = JsValue::from_str(&json_str);
                web_sys::console::log_1(&js_value);
                if tile_cells.contains(&Point { x: col, y: row }) {
                    let a = serde_json::to_string(&col).unwrap();
                    let a1 = JsValue::from_str(&a);
                    web_sys::console::log_1(&a1);
                    let b = serde_json::to_string(&col).unwrap();
                    let b1 = JsValue::from_str(&b);
                    web_sys::console::log_1(&b1);
                    context.set_fill_style(&JsValue::from_str("black"))
                } else {
                    context.set_fill_style(&JsValue::from_str("white"))
                }
                if fixed_cells.contains(&Point { x: col, y: row }) {
                    context.set_fill_style(&JsValue::from_str("red"))
                }

                context.fill_rect(col as f64 * delta, row as f64 * delta, box_size, box_size);

                context.stroke_rect(col as f64 * delta, row as f64 * delta, box_size, box_size);
            }
        }
    }
}

#[wasm_bindgen]
pub fn say_hello() -> String {
    "helllooooooo!".to_string()
}

#[wasm_bindgen]
pub fn start_game() -> Game {
    let game: Game = Game::new();
    game
}
