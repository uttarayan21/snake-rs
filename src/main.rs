extern crate ncurses;
mod game;
mod highscore;
mod menu;
// use game::{Cell, Snake};
// use ncurses::*;
use ncurses::{
    curs_set, endwin, getmaxyx, initscr, keypad, noecho, raw, refresh, setlocale, stdscr,
    LcCategory, CURSOR_VISIBILITY,
};

fn main() {
    // let (lines, cols): (i32, i32) = (0, 0);
    setlocale(LcCategory::all, "");
    initscr();
    raw();
    keypad(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    noecho();
    let (mut mlines, mut mcols): (i32, i32) = (0, 0);
    getmaxyx(stdscr(), &mut mlines, &mut mcols);
    if (mlines < 20) || (mcols < 35) {
        refresh();
        endwin();
        eprintln!("Sorry window size too small");
        std::process::exit(1);
    }
    loop {
        match menu::main_menu_control() {
            0 => game::start(),
            1 => highscore::show(),
            _ => break,
        }
    }
    refresh();
    endwin();
}
