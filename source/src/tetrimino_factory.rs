use crate::{
    point::Point,
    tetrimino::{Tetrimino, TetriminoType},
};
#[derive(Debug)]
pub struct TetriminoFactory;
/**
 *
    I : [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)] @ Pos(1, 0);
    O : [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(1, 1)] @ Pos(0, 0);
    T : [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(1, 1)] @ Pos(1, 0);
    J : [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(-1, 2)] @ Pos(0, 1);
    L : [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(1, 2)] @ Pos(0, 1);
    S : [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(-1, 1)] @ Pos(0, 0);
    Z : [Pos(0, 0), Pos(-1, 0), Pos(0, 1), Pos(1, 1)] @ Pos(0, 0);
*/
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
            },
            // ... other shapes can be added here
            _ => panic!("Invalid shape"), // Or return a Result if you prefer
        }
    }
}
