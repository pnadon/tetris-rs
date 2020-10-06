use ncurses::{
    COLOR_BLUE, COLOR_CYAN, COLOR_GREEN, COLOR_MAGENTA, COLOR_RED, COLOR_WHITE, COLOR_YELLOW,
};
use rand::prelude::random;
pub static COLOR: [i16; 7] = [
    COLOR_YELLOW,
    COLOR_CYAN,
    COLOR_BLUE,
    COLOR_WHITE,
    COLOR_RED,
    COLOR_GREEN,
    COLOR_MAGENTA,
];
pub static SHAPE_COORDS: [[[bool; 5]; 5]; 7] = [
    [
        // O
        [false, false, false, false, false],
        [false, true, true, false, false],
        [false, true, true, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ],
    [
        // I
        [false, false, false, false, false],
        [true, true, true, true, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ],
    [
        // L
        [false, true, true, false, false],
        [false, true, false, false, false],
        [false, true, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ],
    [
        // J
        [false, true, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ],
    [
        // zag
        [false, false, true, false, false],
        [false, true, true, false, false],
        [false, true, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ],
    [
        // zig
        [false, true, false, false, false],
        [false, true, true, false, false],
        [false, false, true, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ],
    [
        // T
        [false, false, false, false, false],
        [false, false, true, false, false],
        [false, true, true, true, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ],
];

pub const DEFAULT_ROW: i32 = 2;
pub const DEFAULT_COL: i32 = 0;

pub static CHAR_KEYS: [[usize; 2]; 7] =
    [[1, 2], [3, 4], [5, 6], [7, 8], [9, 10], [11, 12], [13, 14]];

pub struct Shape {
    coords: [[bool; 5]; 5],
    chars: [usize; 2],
    color: i16,
}

impl Shape {
    pub fn new() -> Self {
        let choice = random::<usize>() % SHAPE_COORDS.len();
        Self {
            coords: SHAPE_COORDS[choice],
            chars: CHAR_KEYS[choice],
            color: COLOR[choice],
        }
    }

    pub fn coords(&self) -> &[[bool; 5]; 5] {
        &self.coords
    }

    pub fn set_coords(&mut self, coords: [[bool; 5]; 5]) {
        self.coords = coords;
    }

    pub fn height(&self) -> usize {
        self.coords().len()
    }

    pub fn width(&self) -> usize {
        self.coords()[0].len()
    }

    pub fn shape_height(&self) -> usize {
        (0..self.height())
            .map(|row| (0..self.width()).any(|col| self.coords()[row][col]))
            .fold(0, |acc, row| acc + if row { 1 } else { 0 })
    }

    pub fn color(&self) -> i16 {
        self.color
    }

    pub fn chars(&self) -> [usize; 2] {
        self.chars
    }
}
