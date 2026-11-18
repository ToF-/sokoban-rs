use crate::coords::Coords;

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn from(&self, coords: Coords) -> Coords {
        let (row, col) = coords;
        match self {
            Direction::Left => (row, col - 1),
            Direction::Up => (row - 1, col),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
        }
    }
}
