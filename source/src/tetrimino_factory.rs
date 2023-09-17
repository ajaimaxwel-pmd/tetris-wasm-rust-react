use crate::{
    point::Point,
    tetrimino::{Tetrimino, TetriminoType},
};
#[derive(Debug)]
pub struct TetriminoFactory;

impl TetriminoFactory {
    pub fn create(shape: TetriminoType) -> Tetrimino {
        match shape {
            TetriminoType::I => Tetrimino {
                shape,
                center: Point { x: 1, y: 0 },
                cells: vec![
                    Point { x: 0, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: 2, y: 0 },
                    Point { x: 3, y: 0 },
                ],
                color: "#c7eae4".to_string(), // TODO: to be called from an enum or hashmap or...
            },
            TetriminoType::O => Tetrimino {
                shape,
                center: Point { x: 0, y: 0 },
                cells: vec![
                    Point { x: 0, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: 0, y: 1 },
                    Point { x: 1, y: 1 },
                ],
                color: "#a7e8bd".to_string(),
            },
            TetriminoType::T => Tetrimino {
                shape,
                center: Point { x: 1, y: 0 },
                cells: vec![
                    Point { x: 0, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: 2, y: 0 },
                    Point { x: 1, y: 1 },
                ],
                color: "#fcbcb8".to_string(),
            },
            TetriminoType::J => Tetrimino {
                shape,
                center: Point { x: 1, y: 1 },
                cells: vec![
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 1 },
                    Point { x: 0, y: 2 },
                    Point { x: -1, y: 2 },
                ],
                color: "#efa7a7".to_string(),
            },
            TetriminoType::L => Tetrimino {
                shape,
                center: Point { x: 0, y: 1 },
                cells: vec![
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 1 },
                    Point { x: 0, y: 2 },
                    Point { x: 1, y: 2 },
                ],
                color: "#ffd972".to_string(),
            },
            TetriminoType::S => Tetrimino {
                shape,
                center: Point { x: 0, y: 0 },
                cells: vec![
                    Point { x: 0, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: 0, y: 1 },
                    Point { x: -1, y: 1 },
                ],
                color: "#dac4f7".to_string(),
            },
            TetriminoType::Z => Tetrimino {
                shape,
                center: Point { x: 0, y: 0 },
                cells: vec![
                    Point { x: 0, y: 0 },
                    Point { x: -1, y: 0 },
                    Point { x: 0, y: 1 },
                    Point { x: 1, y: 1 },
                ],
                color: "#acecf7".to_string(),
            },
            // ... other shapes can be added here
            _ => panic!("Invalid shape"), // Or return a Result if you prefer
        }
    }
}
