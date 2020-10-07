use rand::prelude::random;

use crate::primitives::{
    get_dims, get_tl, num_to_shape, shape_coords, shape_to_num, Coord, Display,
    ShapeType, Symbol, BLOCK_HORIZ_MULT,
};

#[derive(Debug, Clone, Copy)]
pub struct Shape {
    clockwise_rotations: u8,
    shape_type: ShapeType,
    tl_coords: Coord,
    display: Display,
    is_dead: bool,
}

impl Shape {
    pub fn new() -> Self {
        let choice = random::<usize>() % 7 + 1;
        let shape_type = num_to_shape(choice as i16);
        Self {
            clockwise_rotations: 0,
            shape_type,
            tl_coords: get_tl(Display::Next),
            display: Display::Next,
            is_dead: false,
        }
    }

    pub fn coords(&self) -> [Coord; 4] {
        let mut coords = shape_coords(self.shape_type);
        for i in 0..4 {
            for _ in 0..self.clockwise_rotations {
                coords[i] = coords[i].rotate(match self.shape_type {
                    ShapeType::I => 4,
                    _ => 3,
                });
            }
            coords[i] =
                Coord::new(coords[i].row, coords[i].col * BLOCK_HORIZ_MULT) + self.tl_coords;
        }
        coords
    }

    pub fn rotate_right(&mut self) {
        if self.shape_type == ShapeType::O {
            return;
        }
        self.clockwise_rotations = (self.clockwise_rotations + 1) % 4;
    }

    pub fn move_right(&mut self) {
        self.tl_coords.col += 2;
    }

    pub fn move_left(&mut self) {
        self.tl_coords.col -= 2;
    }

    pub fn move_down(&mut self) {
        self.tl_coords.row += 1;
    }

    pub fn tl_coords(&self) -> Coord {
        self.tl_coords
    }

    pub fn shape_width(&self) -> usize {
        self.coords().iter().map(|coord| coord.col).max().unwrap()
            - self.coords().iter().map(|coord| coord.col).min().unwrap()
            + BLOCK_HORIZ_MULT
    }

    pub fn bounding_box(&self) -> [Coord; 2] {
        [
            Coord::new(
                self.coords().iter().map(|coord| coord.row).min().unwrap(),
                self.coords().iter().map(|coord| coord.col).min().unwrap(),
            ),
            Coord::new(
                self.coords().iter().map(|coord| coord.row).max().unwrap() + 1,
                self.coords().iter().map(|coord| coord.col).max().unwrap() + BLOCK_HORIZ_MULT,
            ),
        ]
    }

    pub fn color_num(&self) -> i16 {
        shape_to_num(self.shape_type)
    }

    pub fn symbol(&self) -> Symbol {
        match self.is_dead {
            true => Symbol::DeadBlock(self.color_num()),
            false => Symbol::LiveBlock(self.color_num()),
        }
    }

    pub fn change_display(&mut self, disp: Display, adjust_coords: bool) {
        if adjust_coords {
            self.tl_coords = (self.tl_coords + get_tl(disp)) - get_tl(self.display);
        }
        self.display = disp;
    }

    pub fn center(&mut self) {
        self.tl_coords.col =
            get_tl(self.display).col + get_dims(self.display).col / 2 - self.shape_width() / 2;
        self.tl_coords.col -= self.tl_coords.col % 2;
    }

    pub fn descent_coords(&self, descent: usize) -> [Coord; 4] {
        let mut coords = self.coords();
        for i in 0..coords.len() {
            coords[i] = Coord::new(coords[i].row + descent, coords[i].col)
        }
        coords
    }

    pub fn display(&self) -> Display {
        self.display
    }

    pub fn kill(&mut self) {
        self.is_dead = true;
    }

    pub fn is_dead(&self) -> bool {
        self.is_dead
    }

    pub fn shape_type(&self) -> ShapeType {
        self.shape_type
    }
}
