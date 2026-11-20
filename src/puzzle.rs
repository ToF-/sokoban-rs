use crate::Level;
use crate::State;
use crate::coords::Coords;
use crate::direction::Direction;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Puzzle {
    pieces: HashMap<Coords, State>,
    sokoban_position: Coords,
    goals: HashSet<Coords>,
    crates: Vec<Coords>,
    marks: HashMap<char, Coords>,
}

impl Puzzle {
    pub fn from_level(level: &Level) -> Self {
        let mut pieces = HashMap::new();
        let mut sokoban_position = (0, 0);
        let mut goals = HashSet::new();
        let mut crates = Vec::new();
        let mut marks = HashMap::new();
        for col in 0..level.width() {
            for row in 0..level.height() {
                let state = level.state_at((row, col));
                if state == State::Goal {
                    goals.insert((row, col));
                    pieces.insert((row, col), State::Floor);
                } else if state == State::Stored {
                    goals.insert((row, col));
                    crates.push((row, col));
                    pieces.insert((row, col), State::Crate);
                } else if state == State::Sokoban {
                    pieces.insert((row, col), State::Sokoban);
                    sokoban_position = (row, col);
                } else if state == State::Crate {
                    crates.push((row, col));
                    pieces.insert((row, col), State::Crate);
                } else if state != State::Outside {
                    pieces.insert((row, col), state.clone());
                }
            }
        }
        Self {
            pieces,
            sokoban_position,
            goals,
            crates,
            marks,
        }
    }

    pub fn sokoban_position(&self) -> Coords {
        self.sokoban_position
    }

    pub fn state_at(&self, coords: Coords) -> State {
        self.pieces.get(&coords).expect("illegal coords").clone()
    }

    pub fn is_solved(&self) -> bool {
        self.crates
            .iter()
            .all(|crate_coords| self.goals.contains(crate_coords))
    }

    pub fn can_move(&self, direction: Direction) -> bool {
        let destination = direction.from(self.sokoban_position());
        if let Some(piece) = self.pieces.get(&destination) {
            if *piece == State::Floor || *piece == State::Goal {
                true
            } else if *piece == State::Crate {
                let target = direction.from(destination);
                if let Some(piece) = self.pieces.get(&target) {
                    *piece == State::Floor || *piece == State::Goal
                } else {
                    true
                }
            } else {
                false
            }
        } else {
            true
        }
    }

    pub fn max_crate(&self) -> usize {
        self.crates.len()
    }

    pub fn crate_coords(&self, crate_number: usize) -> Coords {
        self.crates[crate_number]
    }

    pub fn find_crate_index_at(&self, coords: Coords) -> Option<usize> {
        self.crates
            .iter()
            .position(|&crate_coords| crate_coords == coords)
    }

    pub fn move_piece_at(&mut self, origin: Coords, direction: Direction) {
        let origin_state = self.state_at(origin);
        let destination = direction.from(origin);
        let destination_state = self.state_at(destination);
        let target_state = if origin_state == State::Sokoban {
            destination_state.add_sokoban()
        } else if origin_state == State::Crate {
            destination_state.add_crate()
        } else {
            destination_state
        };
        let clean_state = if origin_state == State::Sokoban {
            destination_state.remove_sokoban()
        } else if origin_state == State::Crate {
            destination_state.remove_crate()
        } else {
            destination_state
        };
        if let Some(piece) = self.pieces.get_mut(&destination) {
            *piece = target_state
        };
        if let Some(piece) = self.pieces.get_mut(&origin) {
            *piece = clean_state
        };
        if origin_state == State::Crate {
            let crate_index = self
                .find_crate_index_at(origin)
                .expect("can't find crate index");
            self.crates[crate_index] = destination;
        }
    }

    pub fn r#move(&mut self, direction: Direction) -> Option<Coords> {
        let origin = self.sokoban_position();
        let destination = direction.from(origin);
        let target_opt = if let Some(piece) = self.pieces.get(&destination) {
            if *piece == State::Crate {
                Some(direction.from(destination))
            } else {
                None
            }
        } else {
            None
        };
        if let Some(target) = target_opt {
            self.move_piece_at(destination, direction)
        };
        self.move_piece_at(origin, direction);
        self.sokoban_position = destination;
        target_opt
    }

    pub fn insert_mark(&mut self, letter: char) {
        let coords = self.sokoban_position();
        let _ = self.marks.insert(letter, coords);
    }

    pub fn find_mark_coords(&self, letter: char) -> Option<Coords> {
        self.marks.get(&letter).copied()
    }

    pub fn find_mark_at_coords(&self, target: Coords) -> Option<char> {
        self.marks.iter()
            .find_map(|(&letter, &coords)| 
                if coords == target { Some(letter) } else { None })
    }

    pub fn remove_mark(&mut self, letter: char) {
        let coords = self.sokoban_position();
        self.marks.remove(&letter);
    }

    pub fn pretty_print(&self) -> String {
        let mut result = String::new();
        let mut width: usize = 0;
        let mut height: usize = 0;
        for (coords, _) in &self.pieces {
            let (row, col) = coords;
            if *row >= height {
                height = row + 1;
            }
            if *col >= width {
                width = col + 1;
            }
        }
        for row in 0..height {
            for col in 0..width {
                if let Some(piece) = self.pieces.get(&(row, col)) {
                    result.push(char::from(*piece))
                } else {
                    result.push(' ')
                };
            }
            result.push('\n');
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gtk::gio;

    fn puzzle_1() -> Puzzle {
        gio::resources_register_include!("myapp.gresource").expect("Failed to register resources.");
        let bytes = gio::resources_lookup_data(
            "/com/example/MyApp/files/level_01.txt",
            gio::ResourceLookupFlags::NONE,
        )
        .expect("Failed to find resource");
        let level = Level::from_bytes(&bytes);
        Puzzle::from_level(&level)
    }

    #[test]
    fn initialize_from_level() {
        let puzzle = puzzle_1();
        assert_eq!((9, 12), puzzle.sokoban_position());
        assert_eq!(false, puzzle.is_solved());
    }

    #[test]
    fn sokoban_cannot_move_into_wall() {
        let puzzle = puzzle_1();
        assert_eq!(false, puzzle.can_move(Direction::Left));
        assert_eq!(false, puzzle.can_move(Direction::Down));
        assert_eq!(false, puzzle.can_move(Direction::Right));
        assert_eq!(true, puzzle.can_move(Direction::Up));
    }

    #[test]
    fn sokoban_can_move_on_floor() {
        let mut puzzle = puzzle_1();
        puzzle.r#move(Direction::Up);
        assert_eq!((8, 12), puzzle.sokoban_position());
        puzzle.r#move(Direction::Left);
        puzzle.r#move(Direction::Left);
        assert_eq!((8, 10), puzzle.sokoban_position());
    }
    #[test]
    fn sokoban_can_push_a_crate() {
        let mut puzzle = puzzle_1();
        puzzle.r#move(Direction::Up);
        assert_eq!((8, 12), puzzle.sokoban_position());
        for _ in 0..9 {
            puzzle.r#move(Direction::Left);
        }
        println!("{}", puzzle.pretty_print());
        assert_eq!((8, 3), puzzle.sokoban_position());
        assert_eq!(State::Sokoban, puzzle.state_at((8, 3)));
        assert_eq!(State::Crate, puzzle.state_at((8, 2)));
        assert!(!puzzle.can_move(Direction::Left));
    }
    #[test]
    fn max_crates() {
        let puzzle = puzzle_1();
        assert_eq!(6, puzzle.max_crate())
    }
    #[test]
    fn find_crate_index() {
        let puzzle = puzzle_1();
        assert_eq!(None, puzzle.find_crate_index_at((8, 12)));
        assert_eq!(Some(0), puzzle.find_crate_index_at((8, 3)));
        assert_eq!(Some(1), puzzle.find_crate_index_at((3, 6)));
        assert_eq!(Some(2), puzzle.find_crate_index_at((5, 6)));
        assert_eq!(Some(3), puzzle.find_crate_index_at((8, 6)));
        assert_eq!(Some(4), puzzle.find_crate_index_at((4, 8)));
        assert_eq!(Some(5), puzzle.find_crate_index_at((5, 8)));
    }
    #[test]
    fn crate_coords() {
        let puzzle = puzzle_1();
        assert_eq!((8, 3), puzzle.crate_coords(0));
    }
}
