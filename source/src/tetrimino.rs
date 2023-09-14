extern crate rand;
use rand::Rng;
use serde::{Serialize};
use crate::point::Point;
use crate::tetrimino_factory::TetriminoFactory;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TetriminoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}
#[derive(Debug, Serialize, Clone)]
pub struct Tetrimino {
    pub shape: TetriminoType,
    pub center: Point,
    pub cells: Vec<Point>
}
impl Tetrimino {
    pub fn create_random_tetrimino() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=6) {
            0 => TetriminoFactory::create(TetriminoType::I),
            1 => TetriminoFactory::create(TetriminoType::O),
            2 => TetriminoFactory::create(TetriminoType::T),
            3 => TetriminoFactory::create(TetriminoType::S),
            4 => TetriminoFactory::create(TetriminoType::Z),
            5 => TetriminoFactory::create(TetriminoType::J),
            6 => TetriminoFactory::create(TetriminoType::L),
            _ => panic!("An unexpected error occurred"),
        }
    }

    pub fn rotate(&mut self) -> Self {
        let Point {x: a, y: b} = self.center;
        Self {
            shape: self.shape,
            cells: self
                .cells
                .iter_mut()
                .map(|Point{x: i, y:j}| Point{x: -*j + b + a, y: *i - a + b})
                .collect(),
            center: self.center,
        }
    } 

    pub fn move_down(&mut self) -> Self {
        self.center.y += 1;
        Self {
            shape: self.shape,
            cells: self
                .cells
                .iter_mut()
                .map(|Point{x: i, y:j}| Point{x: *i, y: *j + 1})
                .collect(),
            center: self.center,
        }
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructors() {
        let mut i_shape = Tetrimino::create_random_tetrimino();
        println!("---------I shape is {:?} ", i_shape);
        let ro = i_shape.rotate();
        println!("---------I shape rotated is {:?} ", ro);
        // assert_eq!(i_shape.center, Point{x:81, y:0});
    }
}
