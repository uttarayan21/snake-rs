extern crate rand;
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
    pub line: i32,
    pub col: i32,
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

pub struct Snake {
    head: Cell,
    pub body: LinkedList<Cell>,
    length: i32,
    pub direction: Direction,
}
impl Snake {
    pub fn new(head: Cell) -> Snake {
        let mut temp_body: LinkedList<Cell> = LinkedList::new();
        temp_body.push_front(head);
        Snake {
            head,
            body: temp_body,
            length: 1,
            direction: Direction::Right,
        }
    }

    pub fn smove(&mut self, direction: Direction) {
        // smove because move is already a keyword
        let mut tail: Cell = self.body.pop_back().unwrap();
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
}
