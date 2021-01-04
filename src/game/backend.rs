extern crate rand;
enum CellType {
    Snake,
    Food,
    Empty,
}
struct Cell {
    line: i32,
    col: i32,
    ctype: CellType,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            line: -1,
            col: -1,
            ctype: CellType::Empty,
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
struct Snake {
    length: i32,
}

struct SnakeCell {
    pub line: i32,
    pub col: i32,
}

impl SnakeCell {
    fn next(&self, direction: Direction) -> SnakeCell {
        let (dl, dc) = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        SnakeCell {
            line: self.line + dl,
            col: self.col + dc,
            // ctype: self.ctype,
        }
    }
}
