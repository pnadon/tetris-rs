use crate::shape::Shape;
use ncurses::{addstr, attrset, mvaddstr, refresh, stdscr, wmove, COLOR_PAIR};
use std::{thread, time};

use crate::primitives::{
    Symbol, SCREEN_STR, to_symbol, Coord, from_symbol, arena_row_iter, NEXT_DISP_TL, ARENA_TL, ARENA_DIMS, STAT_DIMS, SCORE_DISP_TL, LINES_DISP_TL, get_tl, get_dims, Display,
};

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

    fn in_area(row: usize, col: usize, tl: Coord, dims: Coord) -> bool {
        (row >= tl.row as usize 
            && 
        row < tl.row as usize + dims.row as usize
        ) && (col >= tl.col as usize
            &&
            col < tl.col as usize + dims.col as usize
        ) 
    }

    fn in_next_disp(row: usize, col: usize) -> bool {
        Self::in_area(row, col, NEXT_DISP_TL, STAT_DIMS)
    }

    fn in_arena(row: usize, col: usize) -> bool {
        Self::in_area(row, col, ARENA_TL, ARENA_DIMS)
    }

    fn in_score_disp(row: usize, col: usize) -> bool {
        Self::in_area(row, col, SCORE_DISP_TL, STAT_DIMS)
    }

    fn in_lines_disp(row: usize, col: usize) -> bool {
        Self::in_area(row, col, LINES_DISP_TL, STAT_DIMS)
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

    pub fn set_cell(&mut self, coord: Coord, disp: Display, val: Symbol) {
        let tl = get_tl(disp);
        self.contents[coord.row + tl.row][coord.col + tl.col] = val;
        if let Symbol::Block(_) = val {
            self.contents[coord.row + tl.row][coord.col + tl.col + 1] = val;
        }
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
    }

    pub fn draw(&self) {
        for row in 0..(self.height()) {
            for col in 0..(self.width(row)) {
                let cur = self.get_cell(row, col);
                // "magic numbers", checks if inside of game window, should be replaced
                if Self::in_arena(row, col) || Self::in_next_disp(row, col) {
                    match cur {
                        Symbol::Block(num) => {
                            attrset(COLOR_PAIR(num));

                            addstr(&from_symbol(cur).to_string());
                            attrset(COLOR_PAIR(1));
                        },
                        Symbol::Space => {
                            addstr(&from_symbol(cur).to_string());
                        },
                        _ => panic!("Invalid symbol in arena"),
                    }
                } else {
                    addstr(&from_symbol(cur).to_string());
                }
            }
            addstr("\n");
        }
    }

    pub fn add_next(&mut self, shape: &Shape) {
        self.wipe_display(Display::Next);
        for coord in shape.coords() {
            self.set_cell( *coord, Display::Next, shape.symbol());
        }
    }

    pub fn wipe_display(&mut self, disp: Display) {
        let start = get_tl(disp);
        let end = start + get_dims(disp);

        for row in start.row..end.row {
            for col in start.col..end.col {
                self.set_space(row, col);
            }
        }
    }

    pub fn update_stat_display(&mut self, stat: u32, disp: Display) {
        self.wipe_display(disp);
        for (idx, chr) in stat.to_string().chars().enumerate() {
            self.set_cell(Coord::new(0, idx), disp, Symbol::Data(chr));
        }
    }

    fn disp_flash(&self, lines: &Vec<usize>) {
        for row in 0..(lines.len()) {
            for col in arena_row_iter() {
                mvaddstr(lines[row] as i32, col as i32, "█");
            }
        }
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

            wmove(stdscr(), 0, 0);
            refresh();
            thread::sleep(time::Duration::from_millis(45));
            self.draw();
            wmove(stdscr(), 0, 0);
            refresh();
            thread::sleep(time::Duration::from_millis(25));

            self.disp_flash(lines);

            wmove(stdscr(), 0, 0);
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
}
