extern crate rand;
use core::iter::Iterator;
use rand::Rng;
use std::collections::LinkedList;
use std::ops::Sub;
use std::thread::sleep;
use std::time::Duration;
#[derive(Debug)]
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
impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Self::Up, &Self::Up) => true,
            (&Self::Down, &Self::Down) => true,
            (&Self::Left, &Self::Left) => true,
            (&Self::Right, &Self::Right) => true,
            _ => false,
        }
    }
}

impl Direction {
    fn opposite(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
#[derive(Debug)]
// impl std::fmt::Debug for Direction {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // write!(f,)
//         write!(
//             f,
//             "{}",
//             match *self {
//                 Self::Up => (),
//                 _ => (),
//             }
//         );
//     }
// }

pub enum CellType {
    Food,
    Snake,
    Empty,
}
pub struct Cell {
    line: u32,
    col: u32,
    ctype: CellType,
}
impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        if (self.line == other.line) && (self.col == other.col) {
            return true;
        } else {
            return false;
        }
    }
}
impl Sub for Cell {
    type Output = (i32, i32);
    fn sub(self, rhs: Cell) -> (i32, i32) {
        return (
            self.line as i32 - rhs.line as i32,
            self.col as i32 - rhs.col as i32,
        );
    }
}
impl Cell {
    pub fn new(l: u32, c: u32, t: CellType) -> Cell {
        Cell {
            line: l,
            col: c,
            ctype: t,
        }
    }
    pub fn random(lines: u32, cols: u32) -> Cell {
        let mut rng = rand::thread_rng();
        Cell::new(
            rng.gen_range(1..lines - 1) as u32,
            rng.gen_range(1..cols - 1) as u32,
            CellType::Food,
        )
    }
    pub fn posyx(&self) -> (u32, u32) {
        return (self.line, self.col);
    }
    pub fn posy(&self) -> u32 {
        return self.line;
    }
    pub fn posx(&self) -> u32 {
        return self.col;
    }
    pub fn chtype(&mut self, ctype: CellType) {
        self.ctype = ctype;
    }
    pub fn is_adjacent(&self, other: &Cell) -> Option<Direction> {
        match *self - *other {
            (0, 1) => Some(Direction::Right),
            (1, 0) => Some(Direction::Down),
            (-1, 0) => Some(Direction::Up),
            (0, -1) => Some(Direction::Left),
            _ => None,
        }
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
    Snake,
}

enum GameState {
    Failed(FailState),
    Ready,
    // Playing,
}

pub struct Board {
    maxlines: u32,
    maxcols: u32,
    gamestate: GameState,
    food: Cell,
}

// impl Board<'_> {
impl Board {
    pub fn new(maxlines: u32, maxcols: u32) -> Board {
        Board {
            maxlines,
            maxcols,
            gamestate: GameState::Ready,
            food: Cell::random(maxlines, maxcols),
        }
    }
    pub fn check_collision(&mut self, snake: &Snake) -> bool {
        let (snake_line, snake_col): (u32, u32) = snake.posyx();
        if (snake_line >= self.maxlines - 1)
            || (snake_col >= self.maxcols - 1)
            || (snake_line <= 0)
            || (snake_col <= 0)
        {
            self.gamestate = GameState::Failed(FailState::Wall);
            return true;
        }
        let mut snake_iter = snake.iter();
        snake_iter.next();
        for snake_cell in snake_iter {
            // O(n) ; don't know how to reduce this complexity
            if snake.posyx() == snake_cell.posyx() {
                self.gamestate = GameState::Failed(FailState::Snake);
                return true;
            }
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
    pub fn food_posyx(&self) -> (u32, u32) {
        return self.food.posyx();
    }
    // pub fn food_posy(&self) -> u32 {
    //     return self.food.line;
    // }
    pub fn eat_food(&mut self, snake: &mut Snake) {
        snake.grow();
        // self.food.chtype(CellType::Empty);
        self.spawn_food(snake);
    }
    pub fn spawn_food(&mut self, snake: &Snake) {
        let mut food: Cell = Cell::random(self.maxlines, self.maxcols);
        let mut spawned_food = false;
        while spawned_food != true {
            // check for colliosions with the snake body until a free spot is found and spawn the food there
            spawned_food = true;
            let mut snake_iter = snake.iter();
            for snake_cell in snake_iter.next() {
                if *snake_cell == food {
                    // O(n) ; I think this is nessacary/ I don't know how to reduce the order
                    // if food collides with the snake body then set food to a new random position and set spawned food to false
                    // so that the snake_iter is started again from the front of the snake
                    food = Cell::random(self.maxlines, self.maxcols);
                    spawned_food = false;
                    break;
                }
            }
        }
        self.food = food;
    }
}

pub struct Snake {
    head: Cell,
    body: LinkedList<Cell>,
    direction: Direction,
    grow: bool,
    speed: u32,
    last_tail: Option<Cell>,
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
            speed: 15,
            last_tail: Some(head),
        }
    }
    pub fn posyx(&self) -> (u32, u32) {
        return (self.head.line, self.head.col);
    }
    pub fn smove(&mut self, _direction: Direction) {
        // smove because move is already a keyword
        let direction: Direction;
        if self.direction == _direction.opposite() {
            direction = self.direction
        } else {
            direction = _direction
        }
        if self.grow == false {
            self.last_tail = Some(self.body.pop_back().unwrap());

            // self.grow = false;
        }
        let (dl, dc): (i32, i32) = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };
        self.direction = direction;
        self.head = Cell::new(
            ((self.head.line as i32) + dl) as u32,
            ((self.head.col as i32) + dc) as u32,
            CellType::Snake,
        );
        self.body.push_front(self.head);
        self.grow = false;
    }
    pub fn tick(&mut self) {
        // sleep(time);
        // let time: std::time::Duration =
        //     std::time::Duration::from_millis((1000 / self.speed) as u64);
        // sleep(time);
        sleep(Duration::from_millis((1000 / self.speed) as u64));
        self.smove(self.direction);
    }
    pub fn grow(&mut self) {
        self.grow = true;
    }
    pub fn iter(&self) -> impl Iterator<Item = &Cell> {
        return self.body.iter();
    }
    pub fn remove(&self) -> Option<Cell> {
        return self.last_tail;
    }
    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }
}
