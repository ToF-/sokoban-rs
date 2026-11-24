use crate::Direction;
use crate::model::coords::Coords;
use crate::model::coords::adjacent;
use crate::model::coords::col;
use crate::model::coords::row;
use crate::model::element::Element;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Level {
    squares: [[Element; MAX_COL]; MAX_ROW],
}

const MAX_COL: usize = 24;
const MAX_ROW: usize = 20;

fn calibrate(bytes: &[u8]) -> Coords {
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
                start_col = start_col.min(col);
                start_row = start_row.min(row);
                end_col = end_col.max(col);
                end_row = end_row.max(row);
                col += 1;
            }
            _ => {
                col += 1;
            }
        }
    }
    let height = end_row - start_row + 1;
    let width = end_col - start_col + 1;
    ((MAX_ROW - height) / 2, (MAX_COL - width) / 2)
}

impl Level {
    pub fn from_bytes(bytes: &[u8]) -> Level {
        let mut squares = vec![vec![Element::Floor; MAX_COL]; MAX_ROW];
        let (start_row, start_col) = calibrate(bytes);
        Self::acquire_level_elements(&mut squares, bytes, (start_row, start_col));
        Self::fill_around_wall_elements(&mut squares);
        Level {
            squares: std::array::from_fn(|row| std::array::from_fn(|col| squares[row][col])),
        }
    }

    fn acquire_level_elements(squares: &mut Vec<Vec<Element>>, bytes: &[u8], coords: Coords) {
        let (start_row, start_col) = coords;
        let mut row = start_row;
        let mut col = start_col;
        for byte in bytes {
            if *byte == b'\n' {
                row += 1;
                col = start_col;
            } else {
                if col < MAX_COL {
                    squares[row][col] = Element::from(*byte);
                    col += 1;
                }
            }
        }
    }

    fn fill_around_wall_elements(squares: &mut Vec<Vec<Element>>) {
        let mut to_paint: VecDeque<(Coords, Element)> = VecDeque::new();
        let mut visited: HashSet<Coords> = HashSet::new();
        to_paint.push_back(((0, 0), Element::Grass_1));
        while let Some((position, state)) = to_paint.pop_front() {
            let (row, col) = position;
            squares[row][col] = state;
            for direction in [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ] {
                if let Some(destination) = adjacent(position, direction) {
                    let next_state = if state == Element::Grass_1 {
                        Element::Grass_2
                    } else {
                        Element::Grass_1
                    };
                    let (row, col) = destination;
                    if !visited.contains(&destination) && squares[row][col] == Element::Floor {
                        to_paint.push_back((destination, next_state));
                        let _ = visited.insert(destination);
                    }
                }
            }
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

    pub fn state_at(&self, coords: Coords) -> Element {
        let (row, col) = coords;
        self.squares[row][col]
    }

    pub fn crate_max(&self) -> usize {
        self.squares
            .iter()
            .flatten()
            .filter(|state| **state == Element::Crate || **state == Element::Stored)
            .count()
    }

    pub fn crate_coords(&self) -> Vec<Coords> {
        let mut result = Vec::new();
        for row in 0..self.height() {
            for col in 0..self.width() {
                let state = self.squares[row][col];
                if state == Element::Crate || state == Element::Stored {
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
            "/com/example/MyApp/files/level_00.txt",
            gio::ResourceLookupFlags::NONE,
        )
        .expect("Failed to find resource");
        let expected = "-|-|-|-|-|-|-|-|-|-|-|-|
|-|-|-|-|-|-|-|-|-|-|-|-
-|-|-|-|-|-|-|-|-|-|-|-|
|-|-|-|-|-|-|-|-|-|-|-|-
-|-|-|-|-|-|-|-|-|-|-|-|
|-|-|-|-|-|-|-|-|-|-|-|-
-|-|-|-|-|-|-|-|-|-|-|-|
|-|-|-|'''''''''''|-|-|-
-|-|-|-'#########'-|-|-|
|-|-|-|'#       #'|-|-|-
-|-|-|-'#.. $ $@#'-|-|-|
|-|-|-|'#       #'|-|-|-
-|-|-|-'#########'-|-|-|
|-|-|-|'''''''''''|-|-|-
-|-|-|-|-|-|-|-|-|-|-|-|
|-|-|-|-|-|-|-|-|-|-|-|-
-|-|-|-|-|-|-|-|-|-|-|-|
|-|-|-|-|-|-|-|-|-|-|-|-
-|-|-|-|-|-|-|-|-|-|-|-|
|-|-|-|-|-|-|-|-|-|-|-|-
";

        let level = Level::from_bytes(&bytes);
        assert_eq!(expected, level.pretty_print());
    }
}
