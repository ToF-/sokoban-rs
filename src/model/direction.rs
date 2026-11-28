use crate::model::coords::Coords;
use crate::model::coords::col;
use crate::model::coords::row;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn directions() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .clone()
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
}

impl From<Direction> for char {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Left => 'L',
            Direction::Up => 'U',
            Direction::Right => 'R',
            Direction::Down => 'D',
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            _ => panic!("illegal direction char"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_of_directions() {
        assert_eq!(
            [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left
            ],
            Direction::directions()
        )
    }

    #[test]
    fn direction_between_coords() {
        assert_eq!(Direction::Up, Direction::between((1, 1), (0, 1)));
        assert_eq!(Direction::Down, Direction::between((0, 1), (1, 1)));
        assert_eq!(Direction::Left, Direction::between((0, 1), (0, 0)));
        assert_eq!(Direction::Right, Direction::between((1, 0), (1, 1)));
    }
}
