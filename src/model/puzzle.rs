use crate::Level;
use crate::model::coords::Coords;
use crate::model::coords::adjacent;
use crate::model::direction::Direction;
use crate::model::element;
use crate::model::element::Element;
use crate::model::move_;
use crate::model::move_::Move_;
use crate::model::move_::move_;
use crate::model::puzzle::Element::Floor;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Puzzle {
    sokoban_coords: Coords,
    elements: HashMap<Coords, Element>,
    floor: HashSet<Coords>,
    goals: HashSet<Coords>,
    crates: HashSet<Coords>,
    marks: HashMap<char, Coords>,
    width: usize,
    height: usize,
}

impl Puzzle {
    pub fn from_level(level: &Level) -> Self {
        let mut elements = HashMap::new();
        let mut floor = HashSet::new();
        let mut sokoban_coords = (0, 0);
        let mut goals = HashSet::new();
        let mut crates = HashSet::new();
        let mut marks = HashMap::new();
        for col in 0..level.width() {
            for row in 0..level.height() {
                let coords = (row, col);
                let level_element = level.element_at(coords);
                let mut is_floor = true;
                match level_element {
                    Element::Crate => {
                        crates.insert(coords);
                    }
                    Element::Stored => {
                        goals.insert(coords);
                        crates.insert(coords);
                    }
                    Element::Sokoban => {
                        sokoban_coords = coords;
                    }
                    Element::OnGoal => {
                        sokoban_coords = coords;
                    }
                    Element::Goal => {
                        goals.insert(coords);
                    }
                    Element::Floor => {}
                    other => is_floor = false,
                };
                if is_floor {
                    floor.insert(coords);
                }
                elements.insert(coords, level_element.clone());
            }
        }
        Self {
            sokoban_coords,
            elements,
            floor,
            goals,
            crates,
            marks,
            width: level.width(),
            height: level.height(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn elements(&self) -> HashMap<Coords, Element> {
        self.elements.clone()
    }

    pub fn floor(&self) -> HashSet<Coords> {
        self.floor.clone()
    }
    pub fn sokoban_coords(&self) -> Coords {
        self.sokoban_coords
    }

    pub fn element_at(&self, coords: Coords) -> Element {
        if coords == self.sokoban_coords() {
            if self.goals.contains(&coords) {
                Element::OnGoal
            } else {
                Element::Sokoban
            }
        } else if self.crates.contains(&coords) {
            if self.goals.contains(&coords) {
                Element::Stored
            } else {
                Element::Crate
            }
        } else if self.floor.contains(&coords) {
            if self.goals.contains(&coords) {
                Element::Goal
            } else {
                Element::Floor
            }
        } else {
            if let Some(element) = self.elements.get(&coords) {
                *element
            } else {
                Element::Outside
            }
        }
    }

    pub fn set_element_at(&mut self, coords: Coords, new_element: Element) {
        self.elements
            .entry(coords)
            .and_modify(|element| *element = new_element)
            .or_insert(new_element);
    }

    fn destination(&self, coords: Coords, direction: Direction) -> Option<Coords> {
        let destination = adjacent(coords, direction).expect("illegal coords");
        if self.floor.contains(&destination) {
            Some(destination)
        } else {
            None
        }
    }

    pub fn possible_moves(&self, coords: Coords, direction: Direction) -> Option<Vec<Move_>> {
        self.destination(coords, direction).and_then(|destination| {
            if self.crates.contains(&coords) && self.crates.contains(&destination) {
                None
            } else {
                let coords_dir: Coords = destination.clone();
                if self.crates.contains(&destination) {
                    self.possible_moves(destination, direction)
                        .and_then(|possible| {
                            let crate_move: Move_ = possible[0];
                            let sokoban_move: Move_ = (coords, coords_dir);
                            let moves: Vec<Move_> = vec![crate_move, sokoban_move];
                            Some(moves)
                        })
                } else {
                    let move_: Move_ = (coords, coords_dir);
                    let moves: Vec<Move_> = vec![move_];
                    Some(moves)
                }
            }
        })
    }

    fn make_move(&mut self, move_: Move_) {
        let (coords, destination) = move_;
        if let Some(mut pos) = self.crates.take(&coords) {
            self.crates.insert(destination);
        } else if self.sokoban_coords == coords {
            self.sokoban_coords = destination;
        }
    }

    pub fn make_moves(&mut self, moves: Vec<Move_>) {
        for move_ in moves {
            self.make_move(move_)
        }
    }

    pub fn walk(&mut self, direction: Direction) -> Option<Vec<Move_>> {
        let coords = self.sokoban_coords();
        self.possible_moves(coords, direction).and_then(|moves| {
            self.make_moves(moves.clone());
            Some(moves)
        })
    }

    pub fn run(&mut self, directions: &Vec<Direction>) {
        for direction in directions.iter() {
            self.walk(*direction);
        }
    }

    pub fn crate_coords(&self) -> Vec<Coords> {
        Vec::from_iter(self.crates())
    }
    pub fn is_solved(&self) -> bool {
        self.crates()
            .iter()
            .all(|crate_coords| self.goals.contains(crate_coords))
    }

    pub fn max_crate(&self) -> usize {
        self.crates().len()
    }

    pub fn crates(&self) -> HashSet<Coords> {
        self.crates.clone()
    }

    pub fn set_mark(&mut self, letter: char) {
        let coords = self.sokoban_coords();
        let _ = self.marks.insert(letter, coords);
    }

    pub fn find_mark_coords(&self, letter: char) -> Option<Coords> {
        self.marks.get(&letter).copied()
    }

    pub fn find_mark_at_coords(&self, target: Coords) -> Option<char> {
        self.marks
            .iter()
            .find_map(|(&letter, &coords)| if coords == target { Some(letter) } else { None })
    }

    pub fn remove_mark(&mut self, letter: char) {
        let coords = self.sokoban_coords();
        self.marks.remove(&letter);
    }

    pub fn pretty_print(&self) -> String {
        let mut result = String::new();
        let mut width: usize = 0;
        let mut height: usize = 0;
        for (coords, _) in &self.elements {
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
                let coords = (row, col);
                if coords == self.sokoban_coords() {
                    if self.goals.contains(&coords) {
                        result.push(char::from(Element::OnGoal))
                    } else {
                        result.push(char::from(Element::Sokoban))
                    }
                } else if self.crates.contains(&coords) {
                    if self.goals.contains(&coords) {
                        result.push(char::from(Element::Stored))
                    } else {
                        result.push(char::from(Element::Crate))
                    }
                } else if self.goals.contains(&coords) {
                    result.push(char::from(Element::Goal))
                } else if self.floor.contains(&coords) {
                    result.push(char::from(Element::Floor))
                } else {
                    if let Some(piece) = self.elements.get(&(row, col)) {
                        result.push(char::from(*piece))
                    } else {
                        result.push(' ')
                    }
                }
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

    fn puzzle() -> Puzzle {
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
        let puzzle = puzzle();
        let mut goals: Vec<&Coords> = puzzle.goals.iter().collect::<Vec<&Coords>>();
        goals.sort();
        let expected: Vec<&Coords> = [
            &(11, 19),
            &(11, 20),
            &(12, 19),
            &(12, 20),
            &(13, 19),
            &(13, 20),
        ]
        .to_vec();
        assert_eq!(expected, goals);
        let mut crate_coords: Vec<Coords> = puzzle.crate_coords().clone();
        crate_coords.sort();

        let expected: Vec<Coords> = vec![(7, 8), (8, 10), (9, 8), (9, 10), (12, 5), (12, 8)];

        assert_eq!(expected, crate_coords);
        assert_eq!((13, 14), puzzle.sokoban_coords());
        assert_eq!(false, puzzle.is_solved());
    }

    #[test]
    fn sokoban_cannot_move_into_walls() {
        let puzzle = puzzle();
        let coords = puzzle.sokoban_coords();
        let coords_up = adjacent(coords, Direction::Up).expect("illegal coords");

        assert_eq!(None, puzzle.possible_moves(coords, Direction::Right));
        assert_eq!(None, puzzle.possible_moves(coords, Direction::Down));
        assert_eq!(None, puzzle.possible_moves(coords, Direction::Left));
        assert_eq!(
            Some(vec![move_(coords, coords_up)]),
            puzzle.possible_moves(coords, Direction::Up)
        );
    }
    #[test]
    fn sokoban_can_push_a_crate() {
        let mut puzzle = puzzle();
        puzzle.set_element_at((12, 9), Element::Sokoban);
        puzzle.set_element_at((13, 14), Element::Floor);
        let coords = (12, 9);
        assert_eq!(
            Some(vec![((12, 8), (12, 7)), ((12, 9), (12, 8))]),
            puzzle.possible_moves(coords, Direction::Left)
        );
    }

    #[test]
    fn sokoban_makes_a_walking_move() {
        let mut puzzle = puzzle();
        puzzle.walk(Direction::Up);
        assert_eq!((12, 14), puzzle.sokoban_coords());
    }
    #[test]
    fn sokoban_cannot_push_two_crates() {
        let mut puzzle = puzzle();
        assert_eq!((13, 14), puzzle.sokoban_coords());
        assert!(puzzle.crates().contains(&(12, 8)));
        assert!(!puzzle.crates().contains(&(12, 7)));
        puzzle.walk(Direction::Up);
        println!("{}", puzzle.pretty_print());
        puzzle.walk(Direction::Left);
        puzzle.walk(Direction::Left);
        puzzle.walk(Direction::Left);
        puzzle.walk(Direction::Left);
        puzzle.walk(Direction::Left);
        puzzle.walk(Direction::Left);
        puzzle.walk(Direction::Left);
        assert_eq!((12, 7), puzzle.sokoban_coords());
        assert!(puzzle.crates().contains(&(12, 6)));
        assert!(puzzle.crates().contains(&(12, 5)));
        let coords = puzzle.sokoban_coords();
        assert_eq!(None, puzzle.possible_moves(coords, Direction::Left));
        puzzle.walk(Direction::Left);
        // no change effected by last walking
        assert_eq!((12, 7), puzzle.sokoban_coords());
        assert!(puzzle.crates().contains(&(12, 6)));
        assert!(puzzle.crates().contains(&(12, 5)));
    }
    #[test]
    fn walking_returns_the_moves_that_were_made() {
        let mut puzzle = puzzle();
        assert_eq!(None, puzzle.walk(Direction::Down));
        assert_eq!(Some(vec![((13, 14), (12, 14))]), puzzle.walk(Direction::Up));
    }
    #[test]
    fn puzzle_is_solved_when_all_crates_are_on_goal() {
        let mut puzzle = puzzle();
        assert!(!puzzle.is_solved());
        puzzle.run(&solution());
        println!("{}", puzzle.pretty_print());
        assert!(puzzle.is_solved());
    }
    #[test]
    fn puzzle_can_receive_a_letter_mark() {
        let mut puzzle = puzzle();
        puzzle.set_mark('a');
        assert_eq!(Some((13, 14)), puzzle.find_mark_coords('a'));
        puzzle.walk(Direction::Up);
        puzzle.walk(Direction::Left);
        puzzle.walk(Direction::Left);
        puzzle.walk(Direction::Left);
        puzzle.set_mark('b');
        assert_eq!(Some((12, 11)), puzzle.find_mark_coords('b'));
    }
    pub fn solution() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Up,
            Direction::Up,
            Direction::Up,
            Direction::Left,
            Direction::Up,
            Direction::Left,
            Direction::Left,
            Direction::Down,
            Direction::Left,
            Direction::Left,
            Direction::Down,
            Direction::Down,
            Direction::Down,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Up,
            Direction::Left,
            Direction::Left,
            Direction::Down,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Up,
            Direction::Up,
            Direction::Up,
            Direction::Left,
            Direction::Up,
            Direction::Left,
            Direction::Left,
            Direction::Down,
            Direction::Down,
            Direction::Down,
            Direction::Up,
            Direction::Up,
            Direction::Left,
            Direction::Left,
            Direction::Down,
            Direction::Down,
            Direction::Down,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Down,
            Direction::Right,
            Direction::Right,
            Direction::Left,
            Direction::Left,
            Direction::Up,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Up,
            Direction::Up,
            Direction::Up,
            Direction::Left,
            Direction::Left,
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Down,
            Direction::Down,
            Direction::Up,
            Direction::Up,
            Direction::Left,
            Direction::Left,
            Direction::Down,
            Direction::Down,
            Direction::Down,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Down,
            Direction::Right,
            Direction::Left,
            Direction::Up,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Up,
            Direction::Up,
            Direction::Up,
            Direction::Up,
            Direction::Right,
            Direction::Up,
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Down,
            Direction::Down,
            Direction::Down,
            Direction::Down,
            Direction::Up,
            Direction::Up,
            Direction::Left,
            Direction::Left,
            Direction::Down,
            Direction::Down,
            Direction::Down,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Down,
            Direction::Right,
            Direction::Up,
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Right,
            Direction::Left,
            Direction::Down,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Up,
            Direction::Up,
            Direction::Up,
            Direction::Up,
            Direction::Up,
            Direction::Up,
            Direction::Right,
            Direction::Right,
            Direction::Down,
            Direction::Down,
            Direction::Left,
            Direction::Left,
            Direction::Down,
            Direction::Down,
            Direction::Down,
            Direction::Down,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Up,
            Direction::Up,
            Direction::Up,
            Direction::Left,
            Direction::Left,
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Down,
            Direction::Down,
            Direction::Up,
            Direction::Up,
            Direction::Left,
            Direction::Left,
            Direction::Down,
            Direction::Down,
            Direction::Down,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Down,
            Direction::Right,
            Direction::Up,
            Direction::Left,
            Direction::Up,
            Direction::Right,
        ]
    }
}
