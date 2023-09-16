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
    pub cells: Vec<Point>,
    pub color: String,
}
impl Tetrimino {
    pub fn create_random_tetrimino() -> Self {
        let mut rng = rand::thread_rng();
        let mut tetrimino: Tetrimino = match rng.gen_range(0..=6) {
            0 => TetriminoFactory::create(TetriminoType::I),
            1 => TetriminoFactory::create(TetriminoType::O),
            2 => TetriminoFactory::create(TetriminoType::T),
            3 => TetriminoFactory::create(TetriminoType::S),
            4 => TetriminoFactory::create(TetriminoType::Z),
            5 => TetriminoFactory::create(TetriminoType::J),
            6 => TetriminoFactory::create(TetriminoType::L),
            _ => panic!("An unexpected error occurred"),
        };
        tetrimino.cells = tetrimino.cells.iter().map(|Point{x: i, y:j}| Point{x: *i + 4, y: *j})
            .collect();
        let Point {x: a, y: b} = tetrimino.center;
        tetrimino.center = Point {x: a + 4, y: b};
        tetrimino
    }

    pub fn rotate(&mut self) -> Self {
        if self.shape == TetriminoType::O {
            // The 'O' shape doesn't rotate, so just return a copy of self.
            return self.clone();
        }
    
        let Point {x: a, y: b} = self.center;
        Self {
            shape: self.shape,
            cells: self
                .cells
                .iter_mut()
                .map(|Point{x: i, y:j}| Point{x: b + a - *j, y: *i - a + b})
                .collect(),
            center: self.center,
            color: self.color.clone(),
        }
    } 

    pub fn move_down(&mut self) -> Self {
        let Point {x: a, y: b} = self.center;
        
        Self {
            shape: self.shape,
            cells: self
                .cells
                .iter_mut()
                .map(|Point{x: i, y:j}| Point{x: *i, y: *j + 1})
                .collect(),
            center: Point {x: a, y: b + 1},
            color: self.color.clone(),
        }
    }

    pub fn move_right(&mut self) -> Self {
        let Point {x: a, y: b} = self.center;
        Self {
            shape: self.shape,
            cells: self
                .cells
                .iter_mut()
                .map(|Point { x: i, y: j }| Point { x: *i + 1, y: *j })
                .collect(),
            center: Point {x: a + 1, y: b},
            color: self.color.clone(),
        }
    }

    pub fn move_left(&mut self) -> Self {
        let Point {x: a, y: b} = self.center;
        Self {
            shape: self.shape,
            cells: self
                .cells
                .iter_mut()
                .map(|Point { x: i, y: j }| Point { x: *i - 1, y: *j })
                .collect(),
            center: Point {x: a - 1, y: b},
            color: self.color.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_shape_tetrimino_i() {
        let mut i_shape = TetriminoFactory::create(TetriminoType::I);
        println!("---------I shape is {:?} ", i_shape);
        let mut rotated = i_shape.rotate();
        println!("---------1 shape rotated is {:?} ", rotated);
        rotated = rotated.rotate();
        println!("---------2 shape rotated is {:?} ", rotated);
        rotated = rotated.rotate();
        println!("---------3 shape rotated is {:?} ", rotated);
        rotated = rotated.rotate();
        println!("---------4 shape rotated is {:?} ", rotated);
        assert_eq!(rotated.center, i_shape.center); 
        assert_eq!(rotated.cells, i_shape.cells); 
    }

    #[test]
    fn test_rotate_shape_tetrimino_o() {
        let mut i_shape = TetriminoFactory::create(TetriminoType::O);
        println!("---------I shape is {:?} ", i_shape);
        let mut rotated = i_shape.rotate();
        println!("---------1 shape rotated is {:?} ", rotated);
        rotated = rotated.rotate();
        println!("---------2 shape rotated is {:?} ", rotated);
        rotated = rotated.rotate();
        println!("---------3 shape rotated is {:?} ", rotated);
        rotated = rotated.rotate();
        println!("---------4 shape rotated is {:?} ", rotated);
        assert_eq!(rotated.center, i_shape.center); 
        assert_eq!(rotated.cells, i_shape.cells); 
    }

    #[test]
    fn test_rotate_shape_tetrimino_t() {
        let mut i_shape = TetriminoFactory::create(TetriminoType::T);
        println!("---------I shape is {:?} ", i_shape);
        let mut rotated = i_shape.rotate();
        println!("---------1 shape rotated is {:?} ", rotated);
        rotated = rotated.rotate();
        println!("---------2 shape rotated is {:?} ", rotated);
        rotated = rotated.rotate();
        println!("---------3 shape rotated is {:?} ", rotated);
        rotated = rotated.rotate();
        println!("---------4 shape rotated is {:?} ", rotated);
        assert_eq!(rotated.center, i_shape.center); 
        assert_eq!(rotated.cells, i_shape.cells); 
    }

    #[test]
    fn test_rotate_shape_tetrimino_j() {
        let mut i_shape = TetriminoFactory::create(TetriminoType::J);
        println!("---------I shape is {:?} ", i_shape);
        let mut rotated = i_shape.rotate();
        println!("---------1 shape rotated is {:?} ", rotated);
        rotated = rotated.rotate();
        println!("---------2 shape rotated is {:?} ", rotated);
        rotated = rotated.rotate();
        println!("---------3 shape rotated is {:?} ", rotated);
        rotated = rotated.rotate();
        println!("---------4 shape rotated is {:?} ", rotated);
        assert_eq!(rotated.center, i_shape.center); 
        assert_eq!(rotated.cells, i_shape.cells); 
    }

    #[test]
    fn test_rotate_shape_tetrimino_l() {
        let mut i_shape = TetriminoFactory::create(TetriminoType::L);
        println!("---------I shape is {:?} ", i_shape);
        let mut rotated = i_shape.rotate();
        println!("---------1 shape rotated is {:?} ", rotated);
        rotated = rotated.rotate();
        println!("---------2 shape rotated is {:?} ", rotated);
        rotated = rotated.rotate();
        println!("---------3 shape rotated is {:?} ", rotated);
        rotated = rotated.rotate();
        println!("---------4 shape rotated is {:?} ", rotated);
        assert_eq!(rotated.center, i_shape.center); 
        assert_eq!(rotated.cells, i_shape.cells); 
    }

    #[test]
    fn test_rotate_shape_tetrimino_s() {
        let mut i_shape = TetriminoFactory::create(TetriminoType::S);
        println!("---------I shape is {:?} ", i_shape);
        let mut rotated = i_shape.rotate();
        println!("---------1 shape rotated is {:?} ", i_shape);
        rotated = rotated.rotate();
        println!("---------2 shape rotated is {:?} ", i_shape);
        rotated = rotated.rotate();
        println!("---------3 shape rotated is {:?} ", i_shape);
        rotated = rotated.rotate();
        println!("---------4 shape rotated is {:?} ", i_shape);
        assert_eq!(rotated.center, i_shape.center); 
        assert_eq!(rotated.cells, i_shape.cells); 
    }

    #[test]
    fn test_rotate_shape_tetrimino_z() {
        let mut z_shape = TetriminoFactory::create(TetriminoType::Z);
        let mut rotated = z_shape.rotate();
        rotated = rotated.rotate();
        rotated = rotated.rotate();
        rotated = rotated.rotate();
        assert_eq!(rotated.center, z_shape.center); 
        assert_eq!(rotated.cells, z_shape.cells); 
    }
}
