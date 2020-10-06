#[derive(Copy, Clone, Debug)]
pub struct Coord {
    pub row: i32,
    pub col: i32,
}

impl Coord {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}
