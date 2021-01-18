// use ncurses::*;
use crate::game::backend::{Board, Snake};
use ncurses::{
    box_, delwin, keypad, mvwaddstr, newwin, stdscr, wborder, wclrtobot, wmove, wrefresh, WINDOW,
};
pub fn game_window(mlines: i32, mcols: i32, vmargin: i32, hmargin: i32) -> WINDOW {
    let game_win: WINDOW;
    let (lines, cols): (i32, i32);
    let (starty, startx): (i32, i32);
    lines = mlines - vmargin * 2;
    cols = mcols - hmargin * 2;
    // starty = (mlines - lines) / 2;
    // startx = (mcols - cols) / 2;
    starty = vmargin;
    startx = hmargin;
    game_win = newwin(lines, cols, starty, startx);
    box_(game_win, 0, 0);
    keypad(game_win, true);
    wrefresh(game_win);
    game_win
}

pub fn destroy_window(win: WINDOW) {
    wmove(win, 0, 0);
    wclrtobot(win);
    wborder(win, 32, 32, 32, 32, 32, 32, 32, 32); // 32 is the ascii code for whitespace. this replaces all the window borders with whitespace
    wrefresh(win); // refresh to remove the borders
    delwin(win); // delete the window
}

pub fn draw_snake(snake: &Snake, game_win: WINDOW) {
    // let mut snake_iter = snake.iter();
    wmove(game_win, 0, 0);
    wclrtobot(game_win);
    box_(game_win, 0, 0);
    for snake_cell in snake.iter() {
        let (snake_l, snake_c): (i32, i32) = snake_cell.posyx();
        mvwaddstr(game_win, snake_l, snake_c, "o");
    }
    wrefresh(game_win);
}

pub fn draw_board(board: &Board, game_win: WINDOW) {
    let (food_l, food_c): (i32, i32) = board.food_posyx();
    mvwaddstr(game_win, food_l, food_c, "F");
}

pub fn _log(snake: &Snake, board: &Board) {
    let (shl, shc): (i32, i32) = snake.posyx();
    let (bfl, bfc): (i32, i32) = board.food_posyx();
    mvwaddstr(stdscr(), 0, 0, &format!("snake:head: {} {} ", shl, shc));
    mvwaddstr(stdscr(), 1, 0, &format!("board:food: {} {} ", bfl, bfc));
    // mvwaddstr(
    //     stdscr(),
    //     2,
    //     0,
    //     &format!(
    //         "board:maxlines {} maxcols {}",
    //         board.maxlines, board.maxcols
    //     ),
    // );

    // for snake_cell in snake.iter() {
    //     let (scl, scc): (i32, i32) = snake_cell.posyx();
    //     waddstr(stdscr(), &format!("cell: {} {} ", scl, scc));
    // }
    // mvwaddstr(
    //     stdscr(),
    //     2,
    //     0,
    //     &format!("snake_size {}", snake.iter().size_hint().0),
    // );
    wrefresh(stdscr());
}
