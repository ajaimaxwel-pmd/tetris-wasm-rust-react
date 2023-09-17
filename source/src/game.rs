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
    game_over: bool,
    points: i64,
    best_score: i64,
}
#[derive(Serialize, Clone, Copy)]
pub struct Board {
    rows: i32,
    cols: i32,
}
#[wasm_bindgen]
impl Board {}

static mut BEST_SCORE: i32 = 0; // TODO

#[wasm_bindgen]
impl Game {

    pub fn new() -> Self {
        Self {
            board: Board { rows: 16, cols: 10 }, // TODO, (row, col) to be set by client
            current_tetrimino: Tetrimino::create_random_tetrimino(),
            next_tetrimino: Tetrimino::create_random_tetrimino(), // FIXME: remove this field after fixing E0277, the trait `RefFromWasmAbi` is not implemented for `Point`
            static_blocks: vec![],
            game_over: false,
            points: 0,
            best_score: 0,
        }
    }

    pub fn is_tetrimino_in_bounds(&mut self) -> bool {
        self.next_tetrimino
            .cells
            .iter()
            .all(|Point { x: i, y: j }| {
                *i >= 0 &&
                *i < self.board.cols &&
                *j >= 0 &&
                *j < self.board.rows
            })
    }

    pub fn is_colliding(&mut self) -> bool {
        let vec1 = &self.next_tetrimino.cells;
        let vec2 = &self.static_blocks;
        let has_common_point = vec1.iter().any(|&point1| vec2.contains(&point1));
        has_common_point
    }

    pub fn move_right(&mut self) {
        self.next_tetrimino = self.current_tetrimino.move_right();
        if !self.is_colliding() && self.is_tetrimino_in_bounds() {
            self.current_tetrimino = self.next_tetrimino.clone(); // FIXME: called twice, need to impl copy trait for Point
            self.draw_board();
        }
    }

    pub fn move_left(&mut self) {
        self.next_tetrimino = self.current_tetrimino.move_left();
        if !self.is_colliding() && self.is_tetrimino_in_bounds() {
            self.current_tetrimino = self.next_tetrimino.clone(); // FIXME: called twice, need to impl copy trait for Point
            self.draw_board();
        }
    }

    pub fn rotate(&mut self) {
        self.next_tetrimino = self.current_tetrimino.rotate();
        if !self.is_colliding() && self.is_tetrimino_in_bounds() {
            self.current_tetrimino = self.next_tetrimino.clone(); // FIXME: called twice, need to impl copy trait for Point
            self.draw_board();
        } else {
            let b = serde_json::to_string("Rotate:: out of bound").unwrap();
            let b1 = JsValue::from_str(&b);
            web_sys::console::log_1(&b1);
        }
    }

    pub fn is_row_complete(&mut self, row: i32) -> bool{
        let mut filled_columns = 0;
        for point in self.static_blocks.iter() {
            if point.y == row {
                filled_columns += 1;
            }
        }
        filled_columns == 10
    }

    fn remove_and_shift_line(&mut self, completed_row: i32) {
        // Remove points in the completed row
        self.static_blocks.retain(|&point| point.y != completed_row);
    
        // Move points that are above the completed row one step down
        for point in self.static_blocks.iter_mut() {
            if point.y < completed_row {
                point.y += 1;
            }
        }
    }

    pub fn check_all_complete_rows(&mut self) {
        for row in 0..15 {
            let is_complete = self.is_row_complete(row);
            let b = serde_json::to_string(&is_complete).unwrap();
            let b1 = JsValue::from_str(&b);
            web_sys::console::log_1(&b1);
            if is_complete {
                self.points += 10;
                self.remove_and_shift_line(row);
            }
        }
    }

    pub fn is_game_over(&mut self) -> bool {
        for point in self.static_blocks.iter() {
            if point.y == 0 {
                return true;
            }
        }
        false
    }

    pub fn move_down(&mut self) {
        self.next_tetrimino = self.current_tetrimino.move_down();
        if !self.is_colliding() && self.is_tetrimino_in_bounds() {
            self.current_tetrimino = self.next_tetrimino.clone(); // FIXME: called twice, need to impl copy trait for Point
        } else {
            self.current_tetrimino.cells.iter().for_each(|p| {
                self.static_blocks.push(*p);
            });
            self.current_tetrimino = Tetrimino::create_random_tetrimino();
        }
        self.check_all_complete_rows();
        if self.is_game_over() {
            self.game_over = true;
            // TODO: stop game
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

        canvas.set_width(400);
        canvas.set_height(600);

        let context: web_sys::CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.begin_path();

        for row in 0..16 {
            for col in 0..10 {
                let delta: f64 = 32 as f64;
                let box_size: f64 = 32 as f64;
                let tile_cells = &self.current_tetrimino.cells;
                let fixed_cells = &self.static_blocks;
                let tile_color = &self.current_tetrimino.color;
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
                    context.set_fill_style(&JsValue::from_str(tile_color))
                } else {
                    context.set_fill_style(&JsValue::from_str("#fbfaff"))
                }
                if fixed_cells.contains(&Point { x: col, y: row }) {
                    context.set_fill_style(&JsValue::from_str("#d8e2dc"))
                }

                context.set_stroke_style(&JsValue::from_str("#ffd9ff"));
                context.set_line_width(0.5);
                context.fill_rect(col as f64 * delta, row as f64 * delta, box_size, box_size);
                context.stroke_rect(col as f64 * delta, row as f64 * delta, box_size, box_size);

                // Draw scrore
                context.clear_rect(400.0, 600.0, 200.0, 50.0);
                context.set_font("30px Arial");
                context.set_fill_style(&JsValue::from_str("black"));
                let score_text = format!("Score: {}", self.points);
                context.fill_text(&score_text, 10.0, 550.0).unwrap();

                if self.game_over {
                    // Draw Game over 
                    context.set_font("50px Arial");
                    context.set_fill_style(&JsValue::from_str("red"));
                    let text = "Game Over";
                    context.fill_text(text, 40.00, 200.00).expect("Failed to render text");
                }
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
