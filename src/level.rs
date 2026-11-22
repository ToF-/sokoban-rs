use crate::Direction;
use crate::coords::Coords;
use crate::coords::col;
use crate::coords::row;
use crate::state::State;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Level {
    squares: [[State; MAX_COL]; MAX_ROW],
}

const MAX_COL: usize = 24;
const MAX_ROW: usize = 20;

fn level_limits(bytes: &[u8]) -> (Coords, Coords) {
    let mut start_col: usize = MAX_COL + 1;
    let mut start_row: usize = MAX_ROW + 1;
    let mut end_col: usize = 0;
    let mut end_row: usize = 0;
    let mut col: usize = 0;
    let mut row: usize = 0;
    for byte in bytes {
        match byte {
            10 => {
                row += 1;
                col = 0;
            }
            b'#' => {
                if col < start_col {
                    start_col = col
                };
                if row < start_row {
                    start_row = row
                };
                if col > end_col {
                    end_col = col
                };
                if row > end_row {
                    end_row = row
                };
                col += 1;
            }
            _ => {
                col += 1;
            }
        }
    }
    ((start_row, start_col), (end_row, end_col))
}
impl Level {
    pub fn from_bytes(bytes: &[u8]) -> Level {
        let mut squares = vec![vec![State::Floor; MAX_COL]; MAX_ROW];
        let ((start_row, start_col), (end_row, end_col)) = level_limits(bytes);
        let height = end_row - start_row + 1;
        let width = end_col - start_col + 1;
        let mut row: usize = (MAX_ROW - height) / 2;
        let mut col: usize = (MAX_COL - width) / 2;

        for byte in bytes {
            match byte {
                b'\n' => {
                    row += 1;
                    col = (MAX_COL - width) / 2;
                }

                _ => {
                    if col < MAX_COL {
                        squares[row][col] = State::from(*byte);
                        col += 1;
                    }
                }
            }
        }
        let mut to_paint: VecDeque<(Coords, State)> = VecDeque::new();
        let mut visited: HashSet<Coords> = HashSet::new();
        to_paint.push_back(((0, 0), State::Grass_1));
        while let Some((position, state)) = to_paint.pop_front() {
            let (row, col) = position;
            squares[row][col] = state;
            for direction in [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ] {
                let destination = direction.from(position);
                let next_state = if state == State::Grass_1 {
                    State::Grass_2
                } else {
                    State::Grass_1
                };
                let (row, col) = destination;
                if row < MAX_ROW && col < MAX_COL && !visited.contains(&destination) {
                    if squares[row][col] == State::Floor {
                        to_paint.push_back((destination, next_state));
                        let _ = visited.insert(destination);
                    }
                }
            }
        }
        Level {
            squares: std::array::from_fn(|row| std::array::from_fn(|col| squares[row][col])),
        }
    }

    pub fn width(&self) -> usize {
        MAX_COL
    }

    pub fn height(&self) -> usize {
        MAX_ROW
    }

    pub fn size(&self) -> usize {
        MAX_COL * MAX_ROW
    }

    pub fn state_at(&self, coords: Coords) -> State {
        let (row, col) = coords;
        self.squares[row][col]
    }

    pub fn crate_max(&self) -> usize {
        self.squares
            .iter()
            .flatten()
            .filter(|state| **state == State::Crate || **state == State::Stored)
            .count()
    }

    pub fn crate_coords(&self) -> Vec<Coords> {
        let mut result = Vec::new();
        for row in 0..self.height() {
            for col in 0..self.width() {
                let state = self.squares[row][col];
                if state == State::Crate || state == State::Stored {
                    result.push((row, col))
                }
            }
        }
        result
    }
    pub fn pretty_print(&self) -> String {
        let mut result = String::new();
        for row in 0..self.height() {
            for col in 0..self.width() {
                result.push(char::from(self.state_at((row, col))))
            }
            result.push('\n');
        }
        result
    }
}

pub fn levels_from_bytes(bytes: &[u8]) -> Vec<Level> {
    let mut levels = Vec::new();
    let mut index = 0;
    let mut start_index = 0;
    let mut start = false;
    let mut level_last_line = false;
    let mut count = 0;
    while index < bytes.len() {
        if bytes[index] == b'\n' {
            if level_last_line {
                levels.push(Level::from_bytes(&bytes[start_index..=index]));
                level_last_line = false;
                start_index = index + 1;
            }
        };
        if bytes[index] == b';' {
            level_last_line = true
        };
        index += 1
    }
    if level_last_line {
        println!("Level#{}", count);
        levels.push(Level::from_bytes(&bytes[start_index..]));
        count += 1;
    }
    levels
}

#[cfg(test)]
mod tests {
    use super::*;
    use gtk::gio;

    #[test]
    fn initialize_from_bytes() {
        gio::resources_register_include!("myapp.gresource").expect("Failed to register resources.");
        let bytes = gio::resources_lookup_data(
            "/com/example/MyApp/files/level_01.txt",
            gio::ResourceLookupFlags::NONE,
        )
        .expect("Failed to find resource");
        println!("{:?}", bytes);
        let level = Level::from_bytes(&bytes);
        assert_eq!(24, level.width());
        assert_eq!(20, level.height());
        assert_eq!(State::Wall, level.state_at((12, 5)));
        assert_eq!(State::Wall, level.state_at((1, 6)));
        assert_eq!(State::Crate, level.state_at((3, 6)));
        assert_eq!(State::Sokoban, level.state_at((9, 12)));
        assert_eq!(6, level.crate_max());
    }
}
