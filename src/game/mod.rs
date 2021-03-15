mod backend;
mod frontend;
use crate::menu;
use crate::settings::Config;
// use ncurses::*;
pub use backend::Difficulty;
use backend::{Board, Cell, Snake};
use ncurses::{
    getmaxyx, nodelay, stdscr, wgetch, wrefresh, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP, WINDOW,
};
pub fn start(config: &Config) {
    let (mut mlines, mut mcols): (i32, i32) = (0, 0);
    let game_win: WINDOW;
    let mut ch: i32;
    let (vmargin, hmargin): (u32, u32) = (5, 10);
    getmaxyx(stdscr(), &mut mlines, &mut mcols);
    game_win = frontend::game_window(mlines as u32, mcols as u32, vmargin, hmargin);
    let mut snake = Snake::new(
        Cell::new(
            mlines as u32 / 2 - vmargin,
            mcols as u32 / 2 - hmargin,
            backend::CellType::Snake,
        ),
        config,
    ); //Initialise snake in the middle of the screen
    let mut board = Board::new(mlines as u32 - vmargin * 2, mcols as u32 - hmargin * 2);
    nodelay(game_win, true);
    loop {
        frontend::draw_snake(&snake, game_win); // always draw snake before board because the snake will clear the game win
        frontend::draw_board(&board, game_win);
        frontend::_log(&snake, &board);
        if board.check_collision(&snake) {
            // Add stuff here to show the score and
            // how You lose screen
            break;
        }
        if board.check_food(&snake) {
            // snake.grow();
            board.eat_food(&mut snake);
        }
        ch = wgetch(game_win);
        match ch {
            KEY_UP | 107 => snake.smove(backend::Direction::Up),
            KEY_DOWN | 106 => snake.smove(backend::Direction::Down),
            KEY_LEFT | 104 => snake.smove(backend::Direction::Left),
            KEY_RIGHT | 108 => snake.smove(backend::Direction::Right),
            112 | 27 => {
                nodelay(game_win, false);
                match menu::pause_menu_control() {
                    //112 is keycode for 'p'
                    0 => (), //resume
                    1 => {
                        snake = Snake::new(
                            Cell::new(
                                mlines as u32 / 2 - vmargin,
                                mcols as u32 / 2 - hmargin,
                                backend::CellType::Snake,
                            ),
                            config,
                        ); //Initialise snake in the middle of the screen

                        board = Board::new(mlines as u32 - vmargin * 2, mcols as u32 - hmargin * 2); //restart
                        frontend::clear_window(game_win);
                    }
                    2 => break, //exit
                    _ => (),    //other charachters just in case
                }
                wrefresh(game_win);
                nodelay(game_win, true);
            }
            // 27 => break,
            _ => snake.tick(),
        }
    }

    frontend::destroy_window(game_win);
}
