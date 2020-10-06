use ncurses::{
    constants::{stdscr, LcCategory},
    curs_set, endwin, initscr, keypad, nodelay, noecho, setlocale, start_color, use_default_colors,
    CURSOR_VISIBILITY,
};

use clap::{App, Arg};
mod coord;
mod game;
mod screen;
mod shape;
mod test;

fn main() {
    setlocale(LcCategory::ctype, "");
    initscr();
    noecho();
    keypad(stdscr(), true);
    nodelay(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    use_default_colors();

    let matches = App::new("Tetris_rs")
        .version("1.0")
        .author("Phil Nadon <phil@nadon.io>")
        .about("Tetris game implemented in Rust. Original: https://github.com/k-vernooy/tetris")
        .arg(
            Arg::with_name("start_level")
                .short("s")
                .long("start-level")
                .validator(|level| match level.parse::<u8>() {
                    Ok(res) => {
                        if res <= 25 && res > 0 {
                            Ok(())
                        } else {
                            Err("wrong number for level".to_string())
                        }
                    }
                    Err(e) => Err(e.to_string()),
                })
                .takes_value(true),
        )
        .arg(
            Arg::with_name("difficulty")
                .short("e")
                .long("easy")
                .takes_value(false),
        )
        .get_matches();

    let start_level: u32 = matches
        .value_of("start-level")
        .unwrap_or("8")
        .parse()
        .unwrap();
    let is_easy = matches.is_present("difficulty");

    test::test();
    // let screen: screen::Screen = screen::Screen::new();
    // let mut game_instance = game::Game::new(screen, start_level, is_easy);
    // loop {
    //     if !game_instance.run() {
    //         break;
    //     }
    // }

    endwin();
}
