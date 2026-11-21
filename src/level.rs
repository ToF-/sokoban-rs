use crate::coords::Coords;
use crate::coords::col;
use crate::coords::row;
use crate::state::State;

#[derive(Clone)]
pub struct Level {
    width: usize,
    height: usize,
    tiles: Vec<State>,
}

const MAX_COL: usize = 22;
const MAX_ROW: usize = 32;

impl Level {
    pub fn from_bytes(bytes: &[u8]) -> Level {
        let mut col: usize = 0;
        let mut row: usize = 0;
        let mut max_col: usize = 0;
        let mut max_row: usize = 0;
        for byte in bytes {
            match byte {
                10 => {
                    row += 1;
                    col = 0;
                }
                _ => {
                    col += 1;
                    if col > max_col {
                        max_col = col
                    };
                }
            }
        }
        max_row = row + 2;
        let size: usize = MAX_ROW * MAX_COL;
        let mut tiles: Vec<State> = vec![State::Outside; MAX_COL * MAX_ROW];
        row = 0;
        col = 0;
        let mut in_comment = false;
        for byte in bytes {
            match byte {
                10 => {
                    row += 1;
                    col = 0;
                    in_comment = false;
                }
                b => {
                    if *b == 59 || *b == 58 {
                        in_comment = true
                    };
                    let state = if !in_comment {
                        State::from(*b)
                    } else {
                        State::Outside
                    };
                    let index = row * MAX_COL + col;
                    tiles[index] = state;
                    col += 1;
                }
            }
        }
        Level {
            width: max_col,
            height: row,
            tiles: tiles,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn state_at(&self, coords: Coords) -> State {
        let index = row(coords) * self.width + col(coords);
        self.tiles[index].clone()
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
    let mut start = 0;
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == 59 {
            levels.push(Level::from_bytes(&bytes[start..index + 2]));
            index += 2;
            start = index + 2;
        } else {
            index += 1;
        }
    }
    if start < bytes.len() {
        levels.push(Level::from_bytes(&bytes[start..]));
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
        assert_eq!(21, level.width());
        assert_eq!(13, level.height());
        assert_eq!(State::Wall, level.state_at((1, 5)));
        assert_eq!(State::Wall, level.state_at((1, 6)));
        assert_eq!(State::Crate, level.state_at((3, 6)));
        assert_eq!(State::Sokoban, level.state_at((9, 12)));
    }
}
