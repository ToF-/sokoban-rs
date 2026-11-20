use std::collections::HashMap;
use crate::Direction;
use crate::Direction::Down;
use crate::Direction::Left;
use crate::Direction::Right;
use crate::Direction::Up;
use crate::Puzzle;
use crate::State;
use crate::coords::Coords;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct PathFinder {
    blocks: HashSet<Coords>,
}

impl PathFinder {
    pub fn new(puzzle: &Puzzle) -> Self {
        let mut blocks = HashSet::new();
        for (coords, state) in puzzle.pieces() {
            if state == State::Wall || state == State::Crate || state == State::Stored {
                let _ = blocks.insert(coords);
            }
        }
        Self { blocks: blocks }
    }

    fn possible_moves(&self, origin: Coords) -> Vec<Coords> {
        let mut result = Vec::new();
        for direction in [Up, Right, Down, Left] {
            let coords = direction.from(origin);
            if !self.blocks.contains(&coords) {
                result.push(coords)
            }
        }
        result
    }
    pub fn path(&mut self, from: Coords, target: Coords) -> Option<Vec<Direction>> {
        let mut to_visit: VecDeque<(Coords, Option<Coords>)> = VecDeque::new();
        let mut moves: HashMap<Coords,Option<Coords>> = HashMap::new();
        let mut found = false;
        to_visit.push_back((from, None));
        while ! (found || to_visit.is_empty()) {
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Level;
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
    fn path_finder_finds_a_path_in_a_puzzle_if_unencumbered_by_crates() {
        let path_finder = PathFinder::new(&puzzle_1());
        let result = path_finder.path((9, 12), (5, 9));
        assert!(result.is_some());
    }
}
