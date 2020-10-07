use crate::shape::Shape;
use ncurses::{addstr, attrset, mvaddstr, refresh, stdscr, wmove, COLOR_PAIR};
use std::{thread, time};

use crate::primitives::{
    arena_row_iter, from_symbol, get_dims, get_tl, in_arena, in_next_disp, to_symbol, Coord,
    Display, Symbol, SCREEN_STR,
};

#[derive(Debug)]
pub struct Screen {
    contents: Vec<Vec<Symbol>>,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            contents: SCREEN_STR
                .lines()
                .map(|line| line.chars().map(to_symbol).collect::<Vec<Symbol>>())
                .collect::<Vec<Vec<Symbol>>>(),
        }
    }

    pub fn width(&self, idx: usize) -> usize {
        self.contents[idx].len()
    }

    pub fn height(&self) -> usize {
        self.contents.len()
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Symbol {
        self.contents[row][col]
    }

    pub fn set_cell(&mut self, coord: Coord, val: Symbol) {
        self.contents[coord.row][coord.col] = val;
        if let Symbol::LiveBlock(_) | Symbol::DeadBlock(_) = val {
            self.contents[coord.row][coord.col + 1] = val;
        }
    }

    pub fn set_disp_cell(&mut self, coord: Coord, disp: Display, val: Symbol) {
        let tl = get_tl(disp);
        self.set_cell(Coord::new(coord.row + tl.row, coord.col + tl.col), val);
    }

    pub fn is_space(&self, row: usize, col: usize) -> bool {
        self.contents[row][col] == Symbol::Space
    }

    pub fn set_space(&mut self, row: usize, col: usize) {
        self.contents[row][col] = Symbol::Space;
    }

    pub fn top(&mut self) {
        wmove(stdscr(), 0, 0);
        addstr(SCREEN_STR.lines().nth(0).unwrap());
        wmove(stdscr(), 0, 0);
    }

    pub fn draw(&mut self) {
        for row in 0..(self.height()) {
            for col in 0..(self.width(row)) {
                let cur = self.get_cell(row, col);
                // "magic numbers", checks if inside of game window, should be replaced
                if in_arena(row, col) || in_next_disp(row, col) {
                    match cur {
                        Symbol::DeadBlock(num) | Symbol::LiveBlock(num) => {
                            attrset(COLOR_PAIR(num));

                            mvaddstr(row as i32, col as i32, &from_symbol(cur).to_string());
                            attrset(COLOR_PAIR(1));
                        }
                        Symbol::Space => {
                            mvaddstr(row as i32, col as i32, &from_symbol(cur).to_string());
                        }
                        _ => (),
                    }
                } else {
                    mvaddstr(row as i32, col as i32, &from_symbol(cur).to_string());
                }
            }
        }
        wmove(stdscr(), 0, 0);
    }

    pub fn add_next(&mut self, shape: &Shape) {
        self.wipe_display(Display::Next);
        for coord in shape.coords().iter() {
            if in_next_disp(coord.row, coord.col) {
                self.set_cell(*coord, shape.symbol());
            } else {
                panic!(
                    "invalid shape & coords for add_next: {:?}\n{:?}",
                    shape,
                    shape.coords()
                );
            }
        }
    }

    pub fn wipe_display(&mut self, disp: Display) {
        let start = get_tl(disp);
        let end = start + get_dims(disp);

        for row in start.row..end.row {
            for col in start.col..end.col {
                mvaddstr(row as i32, col as i32, " ");
                self.set_space(row, col);
            }
        }
        wmove(stdscr(), 0, 0);
    }

    pub fn update_stat_display(&mut self, stat: u32, disp: Display) {
        self.wipe_display(disp);
        for (idx, chr) in stat.to_string().chars().enumerate() {
            self.set_disp_cell(Coord::new(0, idx), disp, Symbol::Data(chr));
        }
    }

    fn disp_flash(&self, lines: &Vec<usize>) {
        for row in 0..(lines.len()) {
            for col in arena_row_iter() {
                mvaddstr(lines[row] as i32, col as i32, "█");
            }
        }
        wmove(stdscr(), 0, 0);
    }

    pub fn shift_lines(&mut self, lines: &Vec<usize>) {
        for row in 0..lines.len() {
            for col in arena_row_iter() {
                self.set_space(lines[row], col);
            }
        }

        thread::sleep(time::Duration::from_millis(45));
        wmove(stdscr(), 0, 0);
        refresh();

        if lines.len() > 0 {
            self.disp_flash(lines);

            refresh();
            thread::sleep(time::Duration::from_millis(45));
            self.draw();
            refresh();
            thread::sleep(time::Duration::from_millis(25));

            self.disp_flash(lines);
            refresh();
            thread::sleep(time::Duration::from_millis(45));
        }

        // optimize, use circular array w/ pointer
        for line in lines.iter() {
            for row in (0..*line).rev() {
                for col in arena_row_iter() {
                    self.contents[row + 1][col] = self.contents[row][col];
                }
            }
        }
    }

    pub fn contents(&self) -> &Vec<Vec<Symbol>> {
        &self.contents
    }
}
