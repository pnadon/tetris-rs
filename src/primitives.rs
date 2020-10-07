use ncurses::{
    COLOR_BLUE, COLOR_CYAN, COLOR_GREEN, COLOR_MAGENTA, COLOR_RED, COLOR_WHITE, COLOR_YELLOW,
};
use std::ops;

pub static SCREEN_STR: &str = "                                       
   ┏━━pnadon/tetris-rs━━┓              
   ┃                    ┃              
   ┃                    ┃   ┏━━next━━━┓
   ┃                    ┃   ┃         ┃
   ┃                    ┃   ┃         ┃
   ┃                    ┃   ┃         ┃
   ┃                    ┃   ┗━━━━━━━━━┛
   ┃                    ┃              
   ┃                    ┃   ┏━━score━━┓
   ┃                    ┃   ┃         ┃
   ┃                    ┃   ┃  0      ┃
   ┃                    ┃   ┃         ┃
   ┃                    ┃   ┗━━━━━━━━━┛
   ┃                    ┃              
   ┃                    ┃   ┏━━lines━━┓
   ┃                    ┃   ┃         ┃
   ┃                    ┃   ┃  0      ┃
   ┃                    ┃   ┃         ┃
   ┃                    ┃   ┗━━━━━━━━━┛
   ┗━━━━━━━━━━━━━━━━━━━━┛              
                                       ";
pub const NEXT_DISP_TL: Coord = Coord { row: 4, col: 29 };
pub const SCORE_DISP_TL: Coord = Coord { row: 11, col: 30 };
pub const LINES_DISP_TL: Coord = Coord { row: 17, col: 30 };
pub const DROP_DISP_TL: Coord = Coord { row: 0, col: 4 };
pub const ARENA_TL: Coord = Coord { row: 2, col: 4 };
pub const STAT_DIMS: Coord = Coord { row: 1, col: 7 };
pub const ARENA_DIMS: Coord = Coord { row: 18, col: 20 };
pub const NEXT_DIMS: Coord = Coord { row: 3, col: 9 };

fn in_area(row: usize, col: usize, tl: Coord, dims: Coord) -> bool {
    (row >= tl.row && row < tl.row + dims.row) && (col >= tl.col && col < tl.col + dims.col)
}

pub fn in_next_disp(row: usize, col: usize) -> bool {
    in_area(row, col, NEXT_DISP_TL, NEXT_DIMS)
}

pub fn in_arena(row: usize, col: usize) -> bool {
    in_area(row, col, ARENA_TL, ARENA_DIMS)
}

pub const BLOCK_HORIZ_MULT: usize = 2;

pub fn arena_row_iter() -> ops::Range<usize> {
    ARENA_TL.col..(ARENA_TL.col + ARENA_DIMS.col)
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Display {
    Next,
    Score,
    Lines,
    Arena,
    Drop,
}

pub fn get_tl(disp: Display) -> Coord {
    match disp {
        Display::Next => NEXT_DISP_TL,
        Display::Score => SCORE_DISP_TL,
        Display::Lines => LINES_DISP_TL,
        Display::Arena => ARENA_TL,
        Display::Drop => DROP_DISP_TL,
    }
}

pub fn get_dims(disp: Display) -> Coord {
    match disp {
        Display::Arena | Display::Drop => ARENA_DIMS,
        Display::Next => NEXT_DIMS,
        _ => STAT_DIMS,
    }
}

pub enum Direction {
    Left,
    Right,
    Down,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn rotate(self, area_width: usize) -> Self {
        Coord::new(self.col, area_width - 1 - self.row)
    }
}

impl ops::Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl ops::Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

impl ops::AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ShapeType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

pub fn shape_color(shape_type: ShapeType) -> i16 {
    match shape_type {
        ShapeType::I => COLOR_WHITE,
        ShapeType::J => COLOR_BLUE,
        ShapeType::L => COLOR_CYAN,
        ShapeType::O => COLOR_RED,
        ShapeType::S => COLOR_GREEN,
        ShapeType::T => COLOR_MAGENTA,
        ShapeType::Z => COLOR_YELLOW,
    }
}

pub fn num_to_shape(num: i16) -> ShapeType {
    match num {
        1 => ShapeType::I,
        2 => ShapeType::J,
        3 => ShapeType::L,
        4 => ShapeType::O,
        5 => ShapeType::S,
        6 => ShapeType::T,
        7 => ShapeType::Z,
        _ => panic!(),
    }
}

pub fn shape_to_num(shape_type: ShapeType) -> i16 {
    match shape_type {
        ShapeType::I => 1,
        ShapeType::J => 2,
        ShapeType::L => 3,
        ShapeType::O => 4,
        ShapeType::S => 5,
        ShapeType::T => 6,
        ShapeType::Z => 7,
    }
}

pub fn shape_coords(shape_type: ShapeType) -> [Coord; 4] {
    match shape_type {
        ShapeType::I => [
            Coord::new(1, 0),
            Coord::new(1, 1),
            Coord::new(1, 2),
            Coord::new(1, 3),
        ],
        ShapeType::J => [
            Coord::new(0, 0),
            Coord::new(1, 0),
            Coord::new(1, 1),
            Coord::new(1, 2),
        ],
        ShapeType::L => [
            Coord::new(1, 0),
            Coord::new(1, 1),
            Coord::new(1, 2),
            Coord::new(0, 2),
        ],
        ShapeType::O => [
            Coord::new(0, 1),
            Coord::new(0, 2),
            Coord::new(1, 1),
            Coord::new(1, 2),
        ],
        ShapeType::S => [
            Coord::new(1, 0),
            Coord::new(1, 1),
            Coord::new(0, 1),
            Coord::new(0, 2),
        ],
        ShapeType::T => [
            Coord::new(0, 1),
            Coord::new(1, 0),
            Coord::new(1, 1),
            Coord::new(1, 2),
        ],
        ShapeType::Z => [
            Coord::new(0, 0),
            Coord::new(0, 1),
            Coord::new(1, 1),
            Coord::new(1, 2),
        ],
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Symbol {
    Data(char),
    Space,
    LiveBlock(i16),
    DeadBlock(i16),
    Text(char),
}

pub fn to_symbol(chr: char) -> Symbol {
    match chr {
        chr @ '0'..='9' => Symbol::Data(chr),
        ' ' => Symbol::Space,
        chr => Symbol::Text(chr),
    }
}

pub fn from_symbol(sym: Symbol) -> char {
    match sym {
        Symbol::Data(num) => num,
        Symbol::Space => ' ',
        Symbol::LiveBlock(_) | Symbol::DeadBlock(_) => '█',
        Symbol::Text(chr) => chr,
    }
}
