#[derive(Debug, PartialEq)]
pub enum CellState {
    Alive,
    Dead,
}

pub struct Cell {
    pub state: CellState,
    pub x: i32,
    pub y: i32,
    pub size: i32,
}

impl Cell {
    pub fn new(state: CellState, x: i32, y: i32, size: i32) -> Self {
        Cell { state, x, y, size }
    }
}
