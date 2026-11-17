use std::collections::HashSet;
use crate::Level;
use crate::State;
use crate::direction::Direction;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Puzzle {
    pieces: HashMap<(usize, usize), State>,
    sokoban_position: (usize, usize),
    goals: HashSet<(usize, usize)>,
}

impl Puzzle {
    pub fn from_level(level: &Level) -> Self {
        let mut pieces = HashMap::new();
        let mut sokoban_position = (0, 0);
        let mut goals = HashSet::new();
        for col in 0..level.width() {
            for row in 0..level.height() {
                let state = level.state_at(row, col);
                if state == State::Goal {
                    goals.insert((row, col));
                    pieces.insert((row, col), State::Floor);
                } else if state == State::Sokoban {
                    pieces.insert((row, col), State::Sokoban);
                    sokoban_position = (row, col);
                } else if state != State::Outside {
                    pieces.insert((row, col), state.clone());
                }
            }
        }
        Self {
            pieces,
            sokoban_position,
            goals,
        }
    }

    pub fn sokoban_position(&self) -> (usize, usize) {
        self.sokoban_position
    }

    pub fn state_at(&self, coords: (usize, usize)) -> State {
        self.pieces.get(&coords).expect("illegal coords").clone()
    }

    pub fn is_solved(&self) -> bool {
        for (coords, piece) in &self.pieces {
            if *piece == State::Crate
                && ! self.goals.contains(&coords) {
                    return false
            }
        }
        true
    }

    pub fn can_move(&self, direction: Direction) -> bool {
        let location = direction.from(self.sokoban_position());
        if let Some(piece) = self.pieces.get(&location) {
            if *piece == State::Floor || *piece == State::Goal {
                true
            } else if *piece == State::Crate || *piece == State::Stored {
                let target = direction.from(location);
                if let Some(piece) = self.pieces.get(&target) {
                    *piece == State::Floor || *piece == State::Goal
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn move_piece_at(&mut self, origin: (usize, usize), direction: Direction) {
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
        if let Some(piece) = self.pieces.get_mut(&destination) {
            *piece = target_state
        }
    }

    pub fn r#move(&mut self, direction: Direction) -> Option<(usize, usize)> {
        let origin = self.sokoban_position();
        let destination = direction.from(origin);
        let target_opt = if let Some(piece) = self.pieces.get(&destination) {
            if *piece == State::Crate || *piece == State::Stored {
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
            dbg!(puzzle.sokoban_position());
            assert!(puzzle.can_move(Direction::Left));
            puzzle.r#move(Direction::Left);
        }
        assert_eq!((8, 3), puzzle.sokoban_position());
        assert_eq!(State::Sokoban, puzzle.state_at((8, 3)));
        assert_eq!(State::Crate, puzzle.state_at((8, 2)));
        assert!(!puzzle.can_move(Direction::Left));
    }
}
