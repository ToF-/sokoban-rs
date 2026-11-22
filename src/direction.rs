use crate::coords::Coords;
use crate::coords::col;
use crate::coords::row;

#[derive(Copy, Clone, Debug, PartialEq)]
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
            Direction::Left => {
                if col > 0 {
                    (row, col - 1)
                } else {
                    (row, 0)
                }
            }
            Direction::Up => {
                if row > 0 {
                    (row - 1, col)
                } else {
                    (0, col)
                }
            }
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
        }
    }

    pub fn between(origin: Coords, destination: Coords) -> Direction {
        if row(origin) < row(destination) {
            Direction::Down
        } else if row(origin) > row(destination) {
            Direction::Up
        } else if col(origin) > col(destination) {
            Direction::Left
        } else {
            Direction::Right
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}
