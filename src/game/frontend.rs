// use ncurses::*;
use crate::game::backend::{Board, Cell, Direction, Snake};
use ncurses::{
    box_, delwin, keypad, mvwaddstr, newwin, stdscr, waddstr, wborder, wclrtobot, wmove, wrefresh,
    WINDOW,
};
pub fn game_window(mlines: u32, mcols: u32, vmargin: u32, hmargin: u32) -> WINDOW {
    let game_win: WINDOW;
    let (lines, cols): (u32, u32);
    let (starty, startx): (u32, u32);
    lines = mlines - vmargin * 2;
    cols = mcols - hmargin * 2;
    starty = vmargin;
    startx = hmargin;
    game_win = newwin(lines as i32, cols as i32, starty as i32, startx as i32);
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
    let (mut prev, mut current, _next): (&Cell, &Cell, &Cell);
    let mut snake_iter = snake.iter();
    wmove(game_win, 0, 0);
    // wclrtobot(game_win);
    box_(game_win, 0, 0);

    // I want to draw the snake as ascii box charachters
    // So I'll need to know the last and next cell of the snake to draw the current snake_cell
    // For some reason the snake goes invisible after the first food
    prev = snake_iter.next().unwrap(); // currently this should be head. On initial run this should be the only snake_cell
    mvwaddstr(
        game_win,
        prev.posy() as i32,
        prev.posx() as i32,
        &format!("{}", std::char::from_u32(0x0298).unwrap_or('O')),
    );
    match snake.remove() {
        Some(tail) => {
            mvwaddstr(game_win, tail.posy() as i32, tail.posx() as i32, " ");
        }
        None => (),
    }
    let _current = snake_iter.next();
    current = match _current {
        Some(cell) => cell,
        None => return,
    };
    for next in snake_iter {
        // O(n) the whole snake is redrawn every single tick
        let (snake_l, snake_c): (u32, u32) = current.posyx();
        // mvwaddstr(game_win, snake_l, snake_c, "o");
        let snake_char: u32 = match (
            prev.is_adjacent(current).unwrap(),
            next.is_adjacent(current).unwrap(),
        ) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => 0x2551, //"║"
            (Direction::Up, Direction::Left) | (Direction::Left, Direction::Up) => 0x255d, //"╝"
            (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => 0x255a, //"╚"
            (Direction::Down, Direction::Left) | (Direction::Left, Direction::Down) => 0x2557, // "╗"
            (Direction::Down, Direction::Right) | (Direction::Right, Direction::Down) => 0x2554, //"╔"
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => 0x2550, //"═"
            _ => 0x20,
        };
        mvwaddstr(
            game_win,
            snake_l as i32,
            snake_c as i32,
            &format!("{}", std::char::from_u32(snake_char).unwrap_or('o')),
        );
        prev = current;
        current = next;
    }

    mvwaddstr(
        game_win,
        current.posy() as i32,
        current.posx() as i32,
        &format!(
            "{}",
            std::char::from_u32(match current.is_adjacent(prev).unwrap() {
                Direction::Up | Direction::Down => 0x2551,
                Direction::Left | Direction::Right => 0x2550,
            })
            .unwrap_or('o')
        ),
    );
    wrefresh(game_win);
}

pub fn draw_board(board: &Board, game_win: WINDOW) {
    let (food_l, food_c): (u32, u32) = board.food_posyx();
    mvwaddstr(
        game_win,
        food_l as i32,
        food_c as i32,
        &format!("{}", std::char::from_u32(0x0298).unwrap_or('F')),
    );
}

pub fn _log(_snake: &Snake, _board: &Board) {
    let (shl, shc): (u32, u32) = _snake.posyx();
    let (stl, stc): (u32, u32) = _snake.remove().unwrap().posyx();
    mvwaddstr(stdscr(), 0, 0, &format!("snake:head: {} {} ", shl, shc));
    mvwaddstr(stdscr(), 1, 0, &format!("snake:tail: {} {} ", stl, stc));
    wmove(stdscr(), 2, 0);
    for snake_cell in _snake.iter() {
        waddstr(
            stdscr(),
            &format!(
                "<cell y:{} x:{}>",
                snake_cell.posyx().0,
                snake_cell.posyx().1
            ),
        );
    }
    wrefresh(stdscr());
}
