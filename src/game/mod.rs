mod backend;
mod frontend;
use crate::menu;
// use ncurses::*;
use ncurses::{getmaxyx, stdscr, wgetch, wrefresh, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP, WINDOW};
use std::thread::sleep;
pub fn start() {
    let (mut mlines, mut mcols): (i32, i32) = (0, 0);
    let game_win: WINDOW;
    let mut ch: i32;
    // let mut choice: i8;
    getmaxyx(stdscr(), &mut mlines, &mut mcols);
    game_win = frontend::game_window(mlines, mcols, 5, 10);
    loop {
        ch = wgetch(game_win);
        match ch {
            KEY_UP => (),
            KEY_DOWN => (),
            KEY_LEFT => (),
            KEY_RIGHT => (),
            112 => {
                match menu::pause_menu_control() {
                    //112 is keycode for 'p'
                    0 => (),    //resume
                    1 => (),    //restart
                    2 => break, //exit
                    _ => (),    //other charachters just in case
                }
                wrefresh(game_win);
            }
            27 => break,
            _ => (),
        }
        sleep(std::time::Duration::new(0, 10));
    }
    frontend::destroy_window(game_win);
}
