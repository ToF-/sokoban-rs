use crate::Direction;
use crate::Direction::Down;
use crate::Direction::Left;
use crate::Direction::Right;
use crate::Direction::Up;
use crate::Puzzle;
use crate::model::coords::Coords;
use crate::model::direction;
use crate::model::puzzle;
use crate::model::state;
use crate::model::state::State;
use std::collections::HashMap;
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

    fn path_from(target: Coords, moves: &HashMap<Coords, Option<Coords>>) -> Vec<Direction> {
        let mut result = Vec::new();
        let mut coords = target;
        while let Some(previous_opt) = moves.get(&coords) {
            if let Some(previous) = previous_opt {
                let direction = Direction::between(*previous, coords);
                result.insert(0, direction);
                coords = *previous
            } else {
                return result;
            }
        }
        result
    }

    pub fn find_path(&mut self, origin: Coords, target: Coords) -> Option<Vec<Direction>> {
        let mut to_visit: VecDeque<(Coords, Option<Coords>)> = VecDeque::new();
        let mut visited: HashSet<Coords> = HashSet::new();
        let mut moves: HashMap<Coords, Option<Coords>> = HashMap::new();
        let mut found = false;
        to_visit.push_back((origin, None));
        while let Some((position, previous)) = to_visit.pop_front() {
            let _ = moves.insert(position, previous);
            if position == target {
                found = true;
                break;
            } else {
                for direction in [
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                ] {
                    let destination = direction.from(position);
                    if !self.blocks.contains(&destination) && !visited.contains(&destination) {
                        to_visit.push_back((destination, Some(position)));
                    }
                }
                let _ = visited.insert(position);
            };
        }
        if found {
            Some(Self::path_from(target, &moves))
        } else {
            None
        }
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

    #[ignore]
    #[test]
    fn path_finder_finds_a_path_in_a_puzzle_if_unencumbered_by_crates() {
        let mut path_finder = PathFinder::new(&puzzle_1());
        let result = path_finder.find_path((9, 12), (8, 12));
        assert_eq!(Some(vec![Direction::Up]), result);
        let mut path_finder = PathFinder::new(&puzzle_1());
        let result = path_finder.find_path((9, 12), (8, 9));
        assert_eq!(
            Some(vec![
                Direction::Up,
                Direction::Left,
                Direction::Left,
                Direction::Left
            ]),
            result
        );
        let mut path_finder = PathFinder::new(&puzzle_1());
        let result = path_finder
            .find_path((9, 12), (5, 9))
            .expect("didn't find a path");
        assert_eq!(7, result.len());
    }
}
