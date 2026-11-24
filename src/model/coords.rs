use crate::model::direction::Direction;

pub type Coords = (usize, usize);

pub const MAX_ROW: usize = 20;
pub const MAX_COL: usize = 24;

pub const MAX_COORDS: (usize, usize) = (MAX_ROW, MAX_COL);
pub fn row(coords: Coords) -> usize {
    coords.0
}

pub fn col(coords: Coords) -> usize {
    coords.1
}

pub fn adjacent(coords: Coords, direction: Direction) -> Option<Coords> {
    let (row, col) = coords;
    match direction {
        Direction::Right if col + 1 >= MAX_COL => None,
        Direction::Right => Some((row, col + 1)),
        Direction::Down if row + 1 >= MAX_ROW => None,
        Direction::Down => Some((row + 1, col)),
        Direction::Left if col == 0 => None,
        Direction::Left => Some((row, col - 1)),
        Direction::Up if row == 0 => None,
        Direction::Up => Some((row - 1, col)),
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coords_max() {
        assert_eq!((20, 24), MAX_COORDS);
    }

    #[test]
    fn adjacent_coords() {
        assert_eq!(Some((0, 1)), adjacent((0, 0), Direction::Right));
        assert_eq!(None, adjacent((0, MAX_COL - 1), Direction::Right));
        assert_eq!(Some((2, 0)), adjacent((1, 0), Direction::Down));
        assert_eq!(None, adjacent((MAX_ROW - 1, 0), Direction::Down));
        assert_eq!(Some((0, 1)), adjacent((0, 2), Direction::Left));
        assert_eq!(None, adjacent((0, 0), Direction::Left));
        assert_eq!(Some((0, 0)), adjacent((1, 0), Direction::Up));
        assert_eq!(None, adjacent((0, 0), Direction::Up));
    }
}
