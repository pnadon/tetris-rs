use crate::shape::Shape;
use crate::shape::COLOR;
use crate::coord::Coord;
use ncurses::{addstr, attrset, init_pair, mvaddstr, refresh, stdscr, wmove, COLOR_PAIR};
use std::{thread, time};

pub static SCREEN_STR: &str = "  ┏━━k-vernooy/tetris━━┓              
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

const DEFAULT_WIN_ROW: usize = 3;
const DEFAULT_WIN_COL: usize = 34;
const NEXT_DISP_TL: Coord = Coord { row: 3, col: 28};
const SCORE_DISP_TL: Coord = Coord { row: 9, col: 28};
const LINES_DISP_TL: Coord = Coord { row: 15, col: 28};
const STAT_DIMS: Coord = Coord { row: 3, col: 9};
const ARENA_TL: Coord = Coord { row: 1, col: 3};
const ARENA_DIMS: Coord = Coord { row: 18, col: 20};

enum Symbols {
    TLCorner,
    TRCorner,
    BLCorner,
    BRCorner,
    VBar,
    HBar,
    Space,
    Block(usize),
    Text(char),
}

pub struct Screen {
    contents: Vec<Vec<String>>,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            contents: SCREEN_STR
                .lines()
                .map(|line| line.chars().map(|s| (s as char).to_string()).collect::<Vec<String>>())
                .collect::<Vec<Vec<String>>>(),
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

    pub fn get_cell(&self, row: usize, col: usize) -> &str {
        &self.contents[row][col]
    }

    pub fn set_cell(&mut self, row: usize, col: usize, val: &str) {
        self.contents[row][col] = val.to_string();
    }

    pub fn is_space(&self, row: usize, col: usize) -> bool {
        self.contents[row][col] == " "
    }

    pub fn set_space(&mut self, row: usize, col: usize) {
        self.contents[row][col] = " ".to_string();
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
                    match cur.parse::<usize>() {
                        Ok(cur_num) => {
                            let (color_num, print_char) = match cur_num % 2 == 0 {
                                true => (cur_num / 2 - 1, "█"),
                                false => (cur_num / 2, "█"),
                            };
                            init_pair(color_num as i16 + 3, COLOR[color_num] as i16, -1);
                            attrset(COLOR_PAIR(color_num as i16 + 3));

                            addstr(print_char);

                            init_pair(1, COLOR[4], -1);
                            attrset(COLOR_PAIR(1));
                        }
                        Err(_) => {
                            addstr(" ");
                        }
                    }
                } else {
                    addstr(cur);
                }
            }
            addstr("\n");
        }
    }

    pub fn add_next(&mut self, shape: &Shape) {
        for i in 0..3 {
            for j in 0..4 {
                if shape.coords()[i][j] {
                    self.set_cell(
                        DEFAULT_WIN_ROW + i,
                        DEFAULT_WIN_COL + (2 * j),
                        &COLOR[0].to_string(),
                    );
                    self.set_cell(
                        DEFAULT_WIN_ROW + i,
                        DEFAULT_WIN_COL + (2 * j) + 1,
                        &COLOR[1].to_string(),
                    );
                } else {
                    self.set_space(DEFAULT_WIN_ROW + i, DEFAULT_WIN_COL + (2 * j));
                    self.set_space(DEFAULT_WIN_ROW + i, DEFAULT_WIN_COL + (2 * j) + 1);
                }
            }
        }
    }

    pub fn update_int_displays(&mut self, score: u32, row: usize, col: usize) {
        self.set_cell(row, col, &score.to_string());

        for z in 37..42 {
            self.set_space(row, z);
        }

        for i in 1..(self.get_cell(row, col).len()) {
            self.set_cell(row, col + i, "");
        }
    }

    fn disp_flash(&self, lines: &Vec<usize>) {
        for row in 0..(lines.len()) {
            for col in 3..23 {
                mvaddstr(lines[row] as i32, col, "█");
            }
        }
    }

    pub fn shift_lines(&mut self, lines: &Vec<usize>) {
        for row in 0..lines.len() {
            for col in 5..25 {
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
        for row in 0..(lines.len()) {
            for col in (0..(lines[row] - 1)).rev() {
                for x in 5..25 {
                    self.contents[col + 1][x] = self.contents[col][x].clone();
                }
            }
        }
    }
}
