use ncurses::{attrset, getch, mvprintw, nodelay, stdscr, wmove, wrefresh, COLOR_PAIR};
use ncurses::{KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP};
use std::{mem, thread, time};
const SPACE_CHAR: i32 = ' ' as i32;
const E_CHAR: i32 = 'e' as i32;
const R_CHAR: i32 = 'r' as i32;
const N_CHAR: i32 = 'n' as i32;
const Y_CHAR: i32 = 'y' as i32;

use crate::primitives::{
    Coord, Direction, Symbol, get_dims, get_tl, Display, ARENA_TL, ARENA_DIMS,
};
use crate::shape::Shape;
use crate::screen::Screen;

const SLEEP_DURATION: u64 = 10; // milliseconds

pub struct Game {
    tr_coord: Coord,
    is_game_over: bool,
    is_easy: bool,
    cannot_move: bool,
    framerate: u32,
    is_dead: bool,
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
            is_game_over: false,
            tr_coord: Coord::new(0, 0),
            cannot_move: false,
            is_easy,
            framerate: 24 - start_level,
            is_dead: false,
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

        loop {
            new_shape = self.check_shape_state(new_shape, count);

            if self.is_game_over {
                break;
            }

            thread::sleep(time::Duration::from_millis(SLEEP_DURATION));

            match getch() {
                KEY_UP => self.rotate(),
                KEY_DOWN => self.move_shape(Direction::Down),
                KEY_LEFT => self.move_shape(Direction::Left),
                KEY_RIGHT => self.move_shape(Direction::Right),
                SPACE_CHAR => self.ground(),
                E_CHAR => self.is_easy ^= true,
                R_CHAR => return true,
                _ => (),
            }

            self.screen.draw();

            if self.is_easy {
                self.draw(self.ground_dist());
            }

            self.draw(0);
            self.screen.top();
            wmove(stdscr(), 0, 0);

            count += 1;

            if self.is_dead {
                new_shape = true;
                self.add_shape();
                self.points();
            }
        }

        return self.game_over();
    }

    pub fn add_shape(&mut self) {
        for coord in self.curr_shape.coords() {
            self.screen.set_cell(*coord, Display::Arena, self.curr_shape.symbol());
        }
    }

    fn check_shape_state(&mut self, new_shape: bool, count: u32) -> bool {
        let mut new_shape = new_shape;
        if new_shape {
            self.gen_shape();
            new_shape = false;
        } else if (count + 1) % self.framerate == 0 {
            self.check_death();
        } else if count % self.framerate == 0 {
            if self.rem_drop_height > 0 {
                self.drop_shape();
            } else {
                self.fall();

                if self.cannot_move {
                    new_shape = true;
                }
            }
        }
        new_shape
    }

    fn rotate(&mut self) {
        let mut temp = [[false; 5]; 5];

        for row in 0..self.curr_shape.height() {
            for col in 0..self.curr_shape.width() {
                temp[temp.len() - 1 - col][row] = self.curr_shape.coords()[row][col]
            }
        }

        if self.char_coords(&temp, 1).iter().all(|coord| {
            self.screen
                .is_space(coord.row as usize - 1, coord.col as usize + 2)
        }) {
            self.curr_shape.set_coords(temp);
        }
    }

    fn move_shape(&mut self, dir: Direction) {
        let coords = self.char_coords(self.curr_shape.coords(), 1);
        match dir {
            Direction::Left => {
                if coords.iter().all(|coord| {
                    self.screen
                        .is_space(coord.row as usize - 1, coord.col as usize)
                }) {
                    self.tr_coord.col -= 2;
                }
            }
            Direction::Right => {
                if coords.iter().all(|coord| {
                    self.screen
                        .is_space(coord.row as usize - 1, coord.col as usize + 4)
                }) {
                    self.tr_coord.col += 2;
                }
            }
            Direction::Down => {
                if coords.iter().all(|coord| {
                    self.screen
                        .is_space(coord.row as usize + 1, coord.col as usize + 2)
                }) {
                    self.tr_coord.row += 1;
                }
            }
        }
    }

    fn ground(&mut self) {
        loop {
            for coord in self.char_coords(self.curr_shape.coords(), 1).iter() {
                if !self
                    .screen
                    .is_space(coord.row as usize + 1, coord.col as usize + 2)
                {
                    return;
                }
            }
            self.tr_coord.row += 1;
        }
    }

    fn ground_dist(&self) -> usize{
        let mut down = 0;
        loop {
            for coord in self.char_coords(self.curr_shape.coords(), down).iter() {
                if !self
                    .screen
                    .is_space(coord.row as usize + 1, coord.col as usize + 2)
                {
                    return down;
                }
            }
            down += 1;
        }
    }

    fn check_death(&mut self) {
        self.is_dead = self
            .char_coords(&self.curr_shape.coords(), 1)
            .iter()
            .any(|coord| {
                !self
                    .screen
                    .is_space(coord.row as usize, coord.col as usize + 3)
            })
    }

    fn drop_shape(&mut self) {
        match self
            .char_coords(&self.curr_shape.coords(), 1)
            .iter()
            .map(|coord| self.screen.get_cell(coord.row as usize, coord.col as usize))
            .any(|chr| match chr {
                Symbol::Block(_) => true,
                _ => false,
            })
        {
            true => self.is_game_over = true,
            false => {
                self.tr_coord.row += 1;
                self.rem_drop_height -= 1;
            }
        }
    }

    fn fall(&mut self) {
        self.tr_coord.row += 1;
    }

    fn draw(&self, down: usize) {
        let mut curr_pos = Coord::new(
            self.tr_coord.row + shape::DEFAULT_ROW + down,
            self.tr_coord.col + shape::DEFAULT_COL,
        );

        attrset(COLOR_PAIR(self.curr_shape.color_num()));
        for row in 0..self.curr_shape.height() {
            for col in 0..self.curr_shape.width() {
                if self.curr_shape.coords()[row][col] {
                    mvprintw(curr_pos.row as i32, curr_pos.col as i32, "█");
                }
                curr_pos.col += 2;
            }
            curr_pos.row += 1;
            curr_pos.col = self.tr_coord.col + shape::DEFAULT_COL;
        }
        attrset(COLOR_PAIR(1));
    }

    fn generate(&mut self) {
        self.is_dead = false;
        mem::swap(&mut self.next_shape, &mut self.curr_shape);
        self.next_shape = Shape::new();

        self.rem_drop_height = self.curr_shape.shape_height();
        self.tr_coord = Coord::new(shape::DEFAULT_ROW, shape::DEFAULT_COL);
    }

    fn gen_shape(&mut self) {
        self.generate();
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
            .filter(|row| (start.col..end.col).all(|col| match self.screen.get_cell(*row, col) {
                Symbol::Block(_) => true,
                _ => false,
            }))
            .collect::<Vec<usize>>();

        self.lines += full_lines.len() as u32;
        self.score += self.points_earned(full_lines.len());

        self.screen.update_stat_display(self.score, Display::Score);
        self.screen.update_stat_display(self.lines, Display::Lines);

        self.screen.shift_lines(&full_lines);

        if (self.level == self.start_level && self.lines > self.start_level * 10 + 10) || (self.lines >= self.level * 10)
        {
            self.advance_level();
            return true;
        }

        return false;
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
            for col in 3..23 {
                mvprintw(row, col, " ");
            }
        }

        mvprintw(11, 2, "┣");
        mvprintw(8, 2, "┣");

        for row in 3..24 {
            mvprintw(8, row, "━");
            mvprintw(11, row, "━");
        }

        mvprintw(11, 23, "┫");
        mvprintw(8, 23, "┫");

        self.screen.top();

        mvprintw(9, 8, "Game over!");
        mvprintw(10, 5, "Try again? (y/n)");

        loop {
            match getch() {
                N_CHAR => return false,
                Y_CHAR => return true,
                _ => (),
            }
        }
    }

    pub fn char_coords(&self, shape: &[Coord; 4], down: usize) -> Vec<Coord> {
        let curr_pos = Coord::new(
            self.tr_coord.row + shape::DEFAULT_ROW + down,
            self.tr_coord.col + shape::DEFAULT_COL,
        );

        return (0..shape.len())
            .map(|row| {
                (0..shape[row].len())
                    .filter(move |col| shape[row][*col])
                    .map(move |col| {
                        Coord::new(curr_pos.row + row, curr_pos.col + 2 * col)
                    })
            })
            .flatten()
            .collect::<Vec<Coord>>();
    }
}
