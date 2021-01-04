// use ncurses::*;
use ncurses::{box_, delwin, keypad, newwin, wborder, wrefresh, WINDOW};
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
    // wmove(win, 0, 0);
    // wclrtobot(win);
    wborder(win, 32, 32, 32, 32, 32, 32, 32, 32); // 32 is the ascii code for whitespace. this replaces all the window borders with whitespace
    wrefresh(win); // refresh to remove the borders
    delwin(win); // delete the window
}
