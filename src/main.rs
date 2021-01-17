extern crate ncurses;
mod game;
mod highscore;
mod menu;
// use game::{Cell, Snake};
// use ncurses::*;
use ncurses::{curs_set, endwin, initscr, keypad, noecho, raw, refresh, stdscr, CURSOR_VISIBILITY};

fn main() {
    // let (lines, cols): (i32, i32) = (0, 0);
    initscr();
    raw();
    keypad(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    noecho();
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
