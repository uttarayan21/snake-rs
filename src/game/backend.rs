extern crate rand;
use crate::settings::Config;
use core::iter::Iterator;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::LinkedList;
use std::ops::Sub;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Difficulty {
    Linear(u8),
    Flat,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        // match (self, other) {
        //     (&Self::Up, &Self::Up) => true,
        //     (&Self::Down, &Self::Down) => true,
        //     (&Self::Left, &Self::Left) => true,
        //     (&Self::Right, &Self::Right) => true,
        //     _ => false,
        // }
        matches!(
            (self, other),
            (&Self::Up, &Self::Up)
                | (&Self::Down, &Self::Down)
                | (&Self::Left, &Self::Left)
                | (&Self::Right, &Self::Right)
        )
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
pub enum CellType {
    Food,
    Snake,
    // Empty,
}
#[derive(Debug)]
pub struct Cell {
    line: u32,
    col: u32,
    _ctype: CellType,
}
impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        (self.line == other.line) && (self.col == other.col)
    }
}
impl Sub for Cell {
    type Output = (i32, i32);
    fn sub(self, rhs: Cell) -> (i32, i32) {
        (
            self.line as i32 - rhs.line as i32,
            self.col as i32 - rhs.col as i32,
        )
    }
}
impl Cell {
    pub fn new(l: u32, c: u32, t: CellType) -> Cell {
        Cell {
            line: l,
            col: c,
            _ctype: t,
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
        (self.line, self.col)
    }
    pub fn posy(&self) -> u32 {
        self.line
    }
    pub fn posx(&self) -> u32 {
        self.col
    }
    pub fn _chtype(&mut self, ctype: CellType) {
        self._ctype = ctype;
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

#[derive(Debug)]
enum FailState {
    Wall,
    Snake,
}

#[derive(Debug)]
enum GameState {
    Failed(FailState),
    Ready,
    // Playing,
}

#[derive(Debug)]
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
            || (snake_line == 0)
            || (snake_col == 0)
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
        false
    }
    pub fn check_food(&self, snake: &Snake) -> bool {
        self.food_posyx() == snake.posyx()
    }
    pub fn food_posyx(&self) -> (u32, u32) {
        self.food.posyx()
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
        while !spawned_food {
            // check for colliosions with the snake body until a free spot is found and spawn the food there
            spawned_food = true;
            let snake_iter = snake.iter();
            for snake_cell in snake_iter {
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
    difficulty: Difficulty,
    grow: bool,
    speed: f32,
    last_tail: Option<Cell>,
}
impl Snake {
    pub fn new(head: Cell, config: &Config) -> Snake {
        let mut temp_body: LinkedList<Cell> = LinkedList::new();
        temp_body.push_front(head);
        Snake {
            head,
            body: temp_body,
            difficulty: config.difficulty,
            direction: Direction::Right,
            grow: true,
            speed: config.speed,
            last_tail: Some(head),
        }
    }
    pub fn posyx(&self) -> (u32, u32) {
        (self.head.line, self.head.col)
    }
    pub fn smove(&mut self, _direction: Direction) {
        // smove because move is already a keyword
        let direction: Direction;
        if self.direction == _direction.opposite() {
            direction = self.direction
        } else {
            direction = _direction
        }
        if !self.grow {
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
        sleep(Duration::from_millis(1000 / self.speed as u64));
        self.smove(self.direction);
    }
    pub fn scale_difficulty(&mut self) {
        match self.difficulty {
            Difficulty::Flat => (),
            Difficulty::Linear(scale) => self.speed *= scale as f32 / 256_f32 + 1_f32,
        }
    }
    pub fn grow(&mut self) {
        self.scale_difficulty();
        self.grow = true;
    }
    pub fn iter(&self) -> impl Iterator<Item = &Cell> {
        self.body.iter()
    }
    pub fn remove(&self) -> Option<Cell> {
        self.last_tail
    }
}
