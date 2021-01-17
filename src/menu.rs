use ncurses::*;
fn menu_window(mlines: i32, mcols: i32, lines: i32, cols: i32) -> WINDOW {
    let menu_win: WINDOW;
    let (starty, startx): (i32, i32);
    starty = (mlines - lines) / 2;
    startx = (mcols - cols) / 2;
    menu_win = newwin(lines, cols, starty, startx);
    box_(menu_win, 0, 0); // add default border
    wrefresh(menu_win); // refreshes the new window to draw it
    keypad(menu_win, true); // to use arrow keys
    menu_win
}
fn destroy_window(win: WINDOW) {
    wmove(win, 0, 0);
    wclrtobot(win);
    // wborder(win, 32, 32, 32, 32, 32, 32, 32, 32); // 32 is the ascii code for whitespace. this replaces all the window borders with whitespace
    wrefresh(win); // refresh to remove the borders
    delwin(win); // delete the window
}
pub fn pause_menu_control() -> i8 {
    let mut ch: i32 = 0;
    let choice: i8;
    let (mut lines, mut cols): (i32, i32) = (0, 0);
    let menu_win: WINDOW;
    let menu_items = vec!["Resume", "Restart", "Exit"];
    let menu_desc = vec!["Resume the game", "Restart the game", "Exit Game"];
    let mut menu_highlight: usize = 0;

    getmaxyx(stdscr(), &mut lines, &mut cols);
    menu_win = menu_window(lines, cols, 10, 20);
    loop {
        for (menu_item_num, menu_item) in menu_items.iter().enumerate() {
            if menu_item_num == menu_highlight {
                wattron(menu_win, A_REVERSE());
            }
            mvwaddstr(menu_win, menu_item_num as i32 + 1, 1, &menu_item);
            wattroff(menu_win, A_REVERSE());
            wrefresh(menu_win);
        }
        mvwaddstr(menu_win, 7, 1, "                  ");
        mvwaddnstr(menu_win, 7, 1, &menu_desc[menu_highlight], 18);
        mvwaddstr(menu_win, 8, 1, &format!("{}", ch));
        ch = wgetch(menu_win);
        match ch {
            KEY_UP | 107 => {
                if menu_highlight > 0 {
                    menu_highlight -= 1
                }
            }
            KEY_DOWN | 106 => {
                if menu_highlight < 2 {
                    menu_highlight += 1
                }
            }

            10 => {
                choice = menu_highlight as i8;
                break;
            }
            27 => {
                choice = 2;
                break;
            }
            _ => (),
        }
    }
    destroy_window(menu_win);
    choice
}
pub fn main_menu_control() -> i8 {
    let mut ch: i32 = 0;
    let choice: i8;
    let (mut lines, mut cols): (i32, i32) = (0, 0);
    let menu_win: WINDOW;
    let menu_items = vec!["Start", "High Score", "Exit"];
    let menu_desc = vec!["Start the game", "Check Highscrore", "Exit Game"];
    let mut menu_highlight: usize = 0;

    getmaxyx(stdscr(), &mut lines, &mut cols);
    menu_win = menu_window(lines, cols, 10, 20);
    loop {
        for (menu_item_num, menu_item) in menu_items.iter().enumerate() {
            if menu_item_num == menu_highlight {
                wattron(menu_win, A_REVERSE());
            }
            mvwaddstr(menu_win, menu_item_num as i32 + 1, 1, &menu_item);
            wattroff(menu_win, A_REVERSE());
            wrefresh(menu_win);
        }
        // mvwaddstr(menu_win, 7, 1, "                  ");
        wmove(menu_win, 7, 1);
        wclrtoeol(menu_win);
        box_(menu_win, 0, 0);
        mvwaddnstr(menu_win, 7, 1, &menu_desc[menu_highlight], 18);
        mvwaddstr(menu_win, 8, 1, &format!("{}", ch));
        ch = wgetch(menu_win);
        match ch {
            KEY_UP | 107 => {
                if menu_highlight > 0 {
                    menu_highlight -= 1
                }
            }
            KEY_DOWN | 106 => {
                if menu_highlight < 2 {
                    menu_highlight += 1
                }
            }
            10 => {
                choice = menu_highlight as i8;
                break;
            }
            27 => {
                choice = 2;
                break;
            }
            _ => (),
        }
    }
    destroy_window(menu_win);
    choice
}
