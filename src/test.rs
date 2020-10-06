use ncurses::*;
use std::{time, thread};
pub static COLOR: [i16; 7] = [
    COLOR_YELLOW,
    COLOR_CYAN,
    COLOR_BLUE,
    COLOR_WHITE,
    COLOR_RED,
    COLOR_GREEN,
    COLOR_MAGENTA,
];
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

pub fn test() {
    wmove(stdscr(), 0, 0);
    nodelay(stdscr(), false);
    wrefresh(stdscr());
    for i in 8..12  {
        for j in 3..23 {
            mvprintw(i,j," ");
        }
    }

    mvprintw(11, 2, "┣");
    mvprintw(8, 2, "┣");

    for i in 3..23{
        mvprintw(8, i, "━");
        mvprintw(11, i, "━");
    }

    mvprintw(11, 23, "┫");
    mvprintw(8, 23, "┫");

    mvprintw(9,8,"Game over!");
    mvprintw(10,5,"Try again? (y/n)");

    wmove(stdscr(), 3, 34);
    let color_num = 2;
    init_pair(1, COLOR_RED, -1);
    attrset(COLOR_PAIR( 1));
    let print_char = "█";
    addstr(print_char);
    wmove(stdscr(), 0, 0);
    let ch = getch();
}