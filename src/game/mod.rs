mod backend;
mod frontend;
use crate::menu;
// use ncurses::*;
use backend::{Cell, Snake};
use ncurses::{
    getmaxyx, nodelay, stdscr, wgetch, wrefresh, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP, WINDOW,
};
use std::thread::sleep;
pub fn start() {
    let (mut mlines, mut mcols): (i32, i32) = (0, 0);
    let game_win: WINDOW;
    let mut ch: i32;
    // let mut choice: i8;
    getmaxyx(stdscr(), &mut mlines, &mut mcols);
    game_win = frontend::game_window(mlines, mcols, 5, 10);
    let mut snake = Snake::new(Cell::new(mlines / 2, mcols / 2, backend::CellType::Snake));
    nodelay(game_win, true);
    loop {
        frontend::draw_snake(&mut snake, game_win);
        ch = wgetch(game_win);
        match ch {
            KEY_UP => snake.smove(backend::Direction::Up),
            KEY_DOWN => snake.smove(backend::Direction::Down),
            KEY_LEFT => snake.smove(backend::Direction::Left),
            KEY_RIGHT => snake.smove(backend::Direction::Right),
            112 => {
                nodelay(game_win, false);
                match menu::pause_menu_control() {
                    //112 is keycode for 'p'
                    0 => (),    //resume
                    1 => (),    //restart
                    2 => break, //exit
                    _ => (),    //other charachters just in case
                }
                wrefresh(game_win);
                nodelay(game_win, true);
            }
            27 => break,
            _ => (),
        }
        sleep(std::time::Duration::from_millis(300));
        snake.smove(snake.direction);
    }

    frontend::destroy_window(game_win);
}
