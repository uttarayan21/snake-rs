extern crate rand;
use rand::Rng;
use std::collections::LinkedList;
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Copy for Direction {}
impl Clone for Direction {
    fn clone(&self) -> Self {
        *self
    }
}
pub enum CellType {
    Food,
    Snake,
    Empty,
}
pub struct Cell {
    line: i32,
    col: i32,
    ctype: CellType,
}

impl Cell {
    pub fn new(l: i32, c: i32, t: CellType) -> Cell {
        Cell {
            line: l,
            col: c,
            ctype: t,
        }
    }
    pub fn random(lines: i32, cols: i32) -> Cell {
        let mut rng = rand::thread_rng();
        Cell::new(
            rng.gen_range(0..lines),
            rng.gen_range(0..cols),
            CellType::Food,
        )
    }
    pub fn posyx(&self) -> (i32, i32) {
        return (self.line, self.col);
    }
    pub fn chtype(&mut self, ctype: CellType) {
        self.ctype = ctype;
    }
}

impl Copy for CellType {}
impl Clone for CellType {
    fn clone(&self) -> CellType {
        *self
    }
}
impl Copy for Cell {}
impl Clone for Cell {
    fn clone(&self) -> Cell {
        *self
    }
}

enum FailState {
    Wall,
    // Body,
}

enum GameState {
    Failed(FailState),
    Ready,
    // Playing,
}

pub struct Board {
    maxlines: i32,
    maxcols: i32,
    gamestate: GameState,
    food: Cell,
}

// impl Board<'_> {
impl Board {
    pub fn new(maxlines: i32, maxcols: i32) -> Board {
        Board {
            maxlines,
            maxcols,
            gamestate: GameState::Ready,
            food: Cell::random(maxlines, maxcols),
        }
    }
    pub fn check_collision(&mut self, snake: &Snake) -> bool {
        let (snake_line, snake_col): (i32, i32) = snake.posyx();
        if (snake_line >= self.maxlines)
            || (snake_col >= self.maxcols)
            || (snake_line <= 0)
            || (snake_col <= 0)
        {
            self.gamestate = GameState::Failed(FailState::Wall);
            return true;
        }
        return false;
    }
    pub fn check_food(&self, snake: &Snake) -> bool {
        if self.food_posyx() == snake.posyx() {
            return true;
        } else {
            return false;
        }
    }
    pub fn food_posyx(&self) -> (i32, i32) {
        return self.food.posyx();
    }
    pub fn eat_food(&mut self, snake: &mut Snake) {
        snake.grow();
        self.food.chtype(CellType::Empty);
    }
    pub fn spawn_food() {}
}

pub struct Snake {
    head: Cell,
    pub body: LinkedList<Cell>,
    // length: i32,
    pub direction: Direction,
    grow: bool,
}
impl Snake {
    pub fn new(head: Cell) -> Snake {
        let mut temp_body: LinkedList<Cell> = LinkedList::new();
        temp_body.push_front(head);
        Snake {
            head,
            body: temp_body,
            // length: 1,
            direction: Direction::Right,
            grow: false,
        }
    }
    pub fn posyx(&self) -> (i32, i32) {
        return (self.head.line, self.head.col);
    }
    pub fn smove(&mut self, direction: Direction) {
        // smove because move is already a keyword
        let mut tail: Cell;
        if self.grow {
            tail = self.body.back().unwrap().clone();
        } else {
            tail = self.body.pop_back().unwrap();
        }
        tail.ctype = CellType::Empty;
        let (dl, dc) = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };
        self.direction = direction;
        self.head = Cell::new(self.head.line + dl, self.head.col + dc, CellType::Snake);
        self.body.push_front(self.head);
    }
    pub fn tick(&mut self) {
        self.smove(self.direction);
    }
    pub fn grow(&mut self) {
        self.grow = true;
    }
}
