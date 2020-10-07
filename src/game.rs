use ncurses::{attrset, getch, mvprintw, nodelay, stdscr, wrefresh, COLOR_PAIR};
use ncurses::{KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP};
use std::{thread, time};
const SPACE_CHAR: i32 = ' ' as i32;
const E_CHAR: i32 = 'e' as i32;
const R_CHAR: i32 = 'r' as i32;
const N_CHAR: i32 = 'n' as i32;
const Y_CHAR: i32 = 'y' as i32;
const D_CHAR: i32 = 'd' as i32;

use crate::primitives::{
    get_dims, get_tl, Coord, Direction, Display, Symbol,
};
use crate::screen::Screen;
use crate::shape::Shape;

const SLEEP_DURATION: u64 = 10; // milliseconds

pub struct Game {
    is_easy: bool,
    framerate: u32,
    rem_drop_height: usize,
    screen: Screen,
    curr_shape: Shape,
    next_shape: Shape,
    level: u32,
    start_level: u32,
    score: u32,
    lines: u32,
}

impl Game {
    pub fn new(screen: Screen, start_level: u32, is_easy: bool) -> Self {
        Self {
            is_easy,
            framerate: 24 - start_level,
            rem_drop_height: 0,
            screen,
            curr_shape: Shape::new(),
            next_shape: Shape::new(),
            level: start_level,
            start_level,
            score: 0,
            lines: 0,
        }
    }

    pub fn run(&mut self) -> bool {
        let mut new_shape = true;
        let mut count = 0;
        let mut stand_still = 0;

        loop {
            if new_shape {
                self.gen_shape();
                new_shape = false;
            }
            if count % self.framerate == 0 {
                if self.rem_drop_height > 0 {
                    if !self.drop_shape() {
                        break;
                    }
                } else if self.curr_shape.display() == Display::Drop {
                    self.curr_shape.change_display(Display::Arena, false);
                }
                if !self.move_shape(Direction::Down) {
                    stand_still += 1;
                    if !self.is_easy && stand_still > 1 || stand_still > 2 {
                        self.curr_shape.kill();
                    }
                } else {
                    stand_still = 0;
                }
            }

            thread::sleep(time::Duration::from_millis(SLEEP_DURATION));
            match getch() {
                KEY_UP => self.rotate(),
                KEY_DOWN => {
                    self.move_shape(Direction::Down);
                }
                KEY_LEFT => {
                    self.move_shape(Direction::Left);
                }
                KEY_RIGHT => {
                    self.move_shape(Direction::Right);
                }
                SPACE_CHAR => self.ground(),
                E_CHAR => self.is_easy ^= true,
                R_CHAR => break,
                _ => (),
            }

            self.screen.draw();
            if self.is_easy {
                self.draw(self.ground_dist());
            }
            self.draw(0);
            self.screen.top();

            if self.curr_shape.is_dead() {
                self.screen.set_shape(self.curr_shape);
                self.points();
                thread::sleep(time::Duration::from_millis(SLEEP_DURATION));
                new_shape = true;
            }

            count += 1;
        }

        self.game_over()
    }

    fn rotate(&mut self) {
        let mut test = self.curr_shape;
        test.rotate_right();
        let occupies_empty_space = test.coords().iter().all(|coord| {
            self.screen.is_space(coord.row, coord.col)
                && self.screen.is_space(coord.row, coord.col + 1)
        });
        if occupies_empty_space {
            
        } else if self.space_available(test, Direction::Left) {
            self.move_shape(Direction::Left);
        } else if self.space_available(test, Direction::Right) {
            self.move_shape(Direction::Right);
        } else {
            return;
        }
        self.curr_shape.rotate_right();
    }

    fn move_shape(&mut self, dir: Direction) -> bool {
        match dir {
            Direction::Left => {
                if self.space_available(self.curr_shape, dir) {
                    self.curr_shape.move_left();
                    true
                } else {
                    false
                }
            }
            Direction::Right => {
                if self.space_available(self.curr_shape, dir) {
                    self.curr_shape.move_right();
                    true
                } else {
                    false
                }
            }
            Direction::Down => {
                if self.space_available(self.curr_shape, dir) {
                    self.curr_shape.move_down();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn space_available(&self, shape: Shape, dir: Direction) -> bool {
        shape.coords().iter().all(|coord| match dir {
            Direction::Left => {
                self.screen.is_space(coord.row, coord.col - 2)
                    && self.screen.is_space(coord.row, coord.col - 1)
            }
            Direction::Right => {
                self.screen.is_space(coord.row, coord.col + 1)
                    && self.screen.is_space(coord.row, coord.col + 2)
            }
            Direction::Down => {
                self.screen.is_space(coord.row + 1, coord.col)
                    && self.screen.is_space(coord.row + 1, coord.col + 1)
            }
        })
    }

    fn ground(&mut self) {
        if self.curr_shape.display() == Display::Drop {
            return;
        }
        while self.move_shape(Direction::Down) {}
        self.curr_shape.kill();
    }

    fn ground_dist(&self) -> usize {
        let mut down = 0;
        let mut test = self.curr_shape;
        loop {
            if !self.space_available(test, Direction::Down) {
                return down;
            }
            test.move_down();
            down += 1;
        }
    }

    fn drop_shape(&mut self) -> bool {
        // assert!(self.curr_shape.display() == Display::Arena);
        let would_occupy_empty_space = self.curr_shape.coords().iter().any(|coord| {
            match (
                self.screen.get_cell(coord.row + 1, coord.col),
                self.screen.get_cell(coord.row + 1, coord.col + 1),
            ) {
                (Symbol::DeadBlock(_), _) | (_, Symbol::DeadBlock(_)) => true,
                _ => false,
            }
        });
        if would_occupy_empty_space {
            return false;
        }
        self.curr_shape.move_down();
        self.rem_drop_height -= 1;
        true
    }

    fn draw(&self, down: usize) {
        attrset(COLOR_PAIR(self.curr_shape.color_num()));
        for coord in self.curr_shape.descent_coords(down).iter() {
            mvprintw(coord.row as i32, coord.col as i32, "██");
        }
        attrset(COLOR_PAIR(1));
    }

    fn gen_shape(&mut self) {
        self.curr_shape = self.next_shape;
        while self.next_shape.shape_type() == self.curr_shape.shape_type() {
            self.next_shape = Shape::new();
        }

        self.rem_drop_height = 2;

        self.curr_shape.change_display(Display::Drop, true);
        self.curr_shape.center();
        self.screen.add_next(&self.next_shape);
        self.drop_shape();
    }

    fn points_earned(&self, rows_filled: usize) -> u32 {
        (match rows_filled {
            0 => 0,
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => 1200,
        }) * self.level
    }

    fn points(&mut self) -> bool {
        let start = get_tl(Display::Arena);
        let end = start + get_dims(Display::Arena);
        let full_lines = (start.row..end.row)
            .filter(|row| {
                (start.col..end.col).all(|col| match self.screen.get_cell(*row, col) {
                    Symbol::DeadBlock(_) | Symbol::LiveBlock(_) => true,
                    _ => false,
                })
            })
            .collect::<Vec<usize>>();

        self.lines += full_lines.len() as u32;
        self.score += self.points_earned(full_lines.len());

        self.screen.update_stat_display(self.score, Display::Score);
        self.screen.update_stat_display(self.lines, Display::Lines);

        self.screen.shift_lines(&full_lines);

        if (self.level == self.start_level && self.lines > self.start_level * 10 + 10)
            || (self.lines >= self.level * 10)
        {
            self.advance_level();
            return true;
        }

        false
    }

    fn advance_level(&mut self) {
        self.level += 1;
        if self.framerate > 2 {
            self.framerate -= 1;
        }
    }

    fn game_over(&mut self) -> bool {
        nodelay(stdscr(), false);
        wrefresh(stdscr());

        for row in 8..12 {
            for col in 4..24 {
                mvprintw(row, col, " ");
            }
        }

        mvprintw(11, 3, "┣");
        mvprintw(8, 3, "┣");

        for row in 4..25 {
            mvprintw(8, row, "━");
            mvprintw(11, row, "━");
        }

        mvprintw(11, 24, "┫");
        mvprintw(8, 24, "┫");

        self.screen.top();

        mvprintw(9, 9, "Game over!");
        mvprintw(10, 6, "Try again? (y/n/d)");

        loop {
            match getch() {
                N_CHAR => return false,
                Y_CHAR => return true,
                D_CHAR => {
                    let bbox = self.curr_shape.bounding_box();
                    mvprintw(
                        0,
                        5,
                        &format!(
                            "{},{} -> [[{},{}][{},{}]]",
                            self.curr_shape.tl_coords().row,
                            self.curr_shape.tl_coords().col,
                            bbox[0].row,
                            bbox[0].col,
                            bbox[1].row,
                            bbox[1].col
                        ),
                    );
                    mvprintw(22, 0, &format!("{:?}", self.screen.contents()));
                    mvprintw(21, 0, &format!("{:?}", self.curr_shape.coords()));
                    loop {
                        let mut coords = Coord::new(0, 0);
                        loop {
                            match getch() {
                                num @ 48..=57 => coords.row = coords.row * 10 + (num as usize - 48),
                                D_CHAR => break,
                                _ => (),
                            }
                            match getch() {
                                num @ 48..=57 => coords.col = coords.col * 10 + (num as usize - 48),
                                D_CHAR => break,
                                R_CHAR => {
                                    mvprintw(
                                        18,
                                        30,
                                        &format!("{:?}", self.screen.contents()[coords.row]),
                                    );
                                }
                                _ => (),
                            }
                            mvprintw(0, 0, &format!("{} {}", coords.row, coords.col));
                        }
                        mvprintw(
                            20,
                            0,
                            &format!("{:?}", self.screen.contents()[coords.row][coords.col]),
                        );
                    }
                }
                _ => (),
            }
        }
    }
}
