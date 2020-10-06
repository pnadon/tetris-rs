use rand::prelude::random;

use crate::primitives::{Coord, Symbol, ShapeType, num_to_shape, shape_coords, shape_to_num, Display, get_tl};

pub struct Shape {
    clockwise_rotations: u8,
    shape_type: ShapeType,
    rel_coords: Coord,
    display: Display,
}

impl Shape {
    pub fn new() -> Self {
        let choice = random::<usize>() % 7;
        let shape_type = num_to_shape(choice as i16);
        Self {
            clockwise_rotations: 0,
            shape_type,
            rel_coords: Coord::new(0, 0),
            display: Display::Next,
        }
    }

    pub fn coords(&self) -> std::slice::Iter<'_, Coord> {
        let mut coords = shape_coords(self.shape_type);
        for i in 0..4 {
            for rotation in 0..self.clockwise_rotations {
                coords[i] = coords[i].rotate(
                    match self.shape_type {
                        ShapeType::I => 4,
                        _ => 3,
                    }
                );
            }
            coords[i] += self.rel_coords + get_tl(self.display);
        }
        coords.iter()
    }

    pub fn rotate_right(&mut self) {
        if self.shape_type == ShapeType::O {
            return;
        }
        self.clockwise_rotations = (self.clockwise_rotations + 1) % 4;
    }

    pub fn rotate_left(&mut self) {
        if self.shape_type == ShapeType::O {
            return;
        }
        self.clockwise_rotations = (self.clockwise_rotations + 3) % 4;
    }

    pub fn shape_height(&self) -> usize {
        self.coords().map(|coord| coord.row).max().unwrap()
        - self.coords().map(|coord| coord.row).min().unwrap() + 1
    }

    pub fn shape_width(&self) -> usize {
        self.coords().map(|coord| coord.col).max().unwrap()
        - self.coords().map(|coord| coord.col).min().unwrap() + 1
    }

    pub fn color_num(&self) -> i16{
        shape_to_num(self.shape_type)
    }

    pub fn shape_type(&self) -> ShapeType {
        self.shape_type
    }

    pub fn symbol(&self) -> Symbol {
        Symbol::Block(self.color_num())
    }
}
