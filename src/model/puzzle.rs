use crate::model::basic_step::Move;
use crate::Level;
use crate::model::basic_step::BasicStep;
use crate::model::coords::Coords;
use crate::model::coords::adjacent;
use crate::model::direction::Direction;
use crate::model::element;
use crate::model::element::Element;
use crate::model::puzzle::Element::Floor;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Puzzle {
    elements: HashMap<Coords, Element>,
    sokoban_coords: Coords,
    goals: HashSet<Coords>,
    crates: Vec<Coords>,
    marks: HashMap<char, Coords>,
    width: usize,
    height: usize,
}

impl Puzzle {
    pub fn from_level(level: &Level) -> Self {
        let mut elements = HashMap::new();
        let mut sokoban_coords = (0, 0);
        let mut goals = HashSet::new();
        let mut crates = Vec::new();
        let mut marks = HashMap::new();
        for col in 0..level.width() {
            for row in 0..level.height() {
                let coords = (row, col);
                let level_element = level.element_at(coords);
                match level_element {
                    Element::Crate => {
                        crates.push(coords);
                    }
                    Element::Stored => {
                        goals.insert(coords);
                        crates.push(coords);
                    }
                    Element::Sokoban => {
                        sokoban_coords = coords;
                    }
                    Element::OnGoal => {
                        sokoban_coords = coords;
                        goals.insert(coords);
                    }
                    Element::Goal => {
                        goals.insert(coords);
                    }
                    other => {}
                };
                elements.insert(coords, level_element.clone());
            }
        }
        Self {
            elements,
            sokoban_coords,
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
    pub fn sokoban_coords(&self) -> Coords {
        self.sokoban_coords
    }

    pub fn element_at(&self, coords: Coords) -> Element {
        self.elements.get(&coords).expect("illegal coords").clone()
    }

    pub fn update_positions(&mut self, basic_step: &BasicStep) {
        dbg!(&self.crates);
        if basic_step.destination_after == Element::Sokoban
            || basic_step.destination_after == Element::OnGoal
        {
            println!("moving sokoban to {:?}", basic_step.to);
            self.sokoban_coords = basic_step.to
        } else if basic_step.destination_after == Element::Crate
            || basic_step.destination_after == Element::Stored
        {
            let index = self
                .find_crate_index_at(basic_step.from)
                .expect("can't find crate");
            self.crates[index] = basic_step.to;
            println!("moving crate#{} from {:?} to {:?}", index, basic_step.from, basic_step.to);
        }
    }
    pub fn puzzle_step(&mut self, basic_step: &BasicStep) {
        self.elements
            .entry(basic_step.from)
            .and_modify(|element| *element = basic_step.origin_after)
            .or_insert(basic_step.origin_after);
        self.elements
            .entry(basic_step.to)
            .and_modify(|element| *element = basic_step.destination_after)
            .or_insert(basic_step.destination_after);
        self.update_positions(&basic_step);
    }

    pub fn puzzle_move(&mut self, direction: Direction) -> Option<Move> {
        let origin_coords = self.sokoban_coords;
        let destination_coords =
            adjacent(origin_coords, direction).expect("unreachable destination");
        let target_coords = adjacent(destination_coords, direction).expect("unreachable target");
        BasicStep::new(self, destination_coords, target_coords).and_then(|step_1| {
            if !step_1.is_identical() {
                self.puzzle_step(&step_1)
            };
            BasicStep::new(self, origin_coords, destination_coords).and_then(|step_2| {
                self.puzzle_step(&step_2);
                if !step_1.is_identical() {
                    Some(vec![step_1, step_2])
                } else {
                    Some(vec![step_2])
                }
            })
        })
    }

    pub fn puzzle_undo(&mut self, move_: Move) {
        let mut steps = move_.clone();
        steps.reverse();
        for step in steps.iter() {
            let basic_step = step.reverse();
            self.puzzle_step(&basic_step);
        self.update_positions(&basic_step);
        }
    }

    pub fn is_solved(&self) -> bool {
        self.crates
            .iter()
            .all(|crate_coords| self.goals.contains(crate_coords))
    }

    pub fn can_move(&self, direction: Direction) -> bool {
        if let Some(destination) = adjacent(self.sokoban_coords(), direction)
            && let Some(piece) = self.elements.get(&destination)
        {
            if *piece == Element::Floor || *piece == Element::Goal {
                true
            } else if *piece == Element::Crate {
                if let Some(target) = adjacent(destination, direction)
                    && let Some(piece) = self.elements.get(&target)
                {
                    *piece == Element::Floor || *piece == Element::Goal
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

    pub fn crates(&self) -> Vec<Coords> {
        self.crates.clone()
    }

    pub fn find_crate_index_at(&self, coords: Coords) -> Option<usize> {
        self.crates
            .iter()
            .position(|&crate_coords| crate_coords == coords)
    }

    pub fn insert_mark(&mut self, letter: char) {
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
                if let Some(piece) = self.elements.get(&(row, col)) {
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
        let mut crates: Vec<Coords> = puzzle.crates.clone();
        crates.sort();

        let expected: Vec<Coords> = vec![(7, 8), (8, 10), (9, 8), (9, 10), (12, 5), (12, 8)];

        assert_eq!(expected, crates);
        assert_eq!((13, 14), puzzle.sokoban_coords());
        assert_eq!(false, puzzle.is_solved());
    }

    #[test]
    fn sokoban_cannot_move_into_wall() {
        let puzzle = puzzle();
        let coords = puzzle.sokoban_coords();
        let coords_up = adjacent(coords, Direction::Up);
        println!("{}", puzzle.pretty_print());
        dbg!(coords);
        dbg!(coords_up);
        assert_eq!(
            Some(&Element::Floor),
            puzzle.elements().get(&coords_up.unwrap())
        );
        assert_eq!(false, puzzle.can_move(Direction::Left));
        assert_eq!(false, puzzle.can_move(Direction::Down));
        assert_eq!(false, puzzle.can_move(Direction::Right));
        assert_eq!(true, puzzle.can_move(Direction::Up));
    }

    #[test]
    fn max_crates() {
        let puzzle = puzzle();
        assert_eq!(6, puzzle.max_crate())
    }
    #[test]
    fn find_crate_index() {
        let puzzle = puzzle();
        println!("{}", puzzle.pretty_print());
        dbg!(&puzzle.crates);
        assert_eq!(None, puzzle.find_crate_index_at((8, 12)));
        assert_eq!(Some(0), puzzle.find_crate_index_at((12, 5)));
        assert_eq!(Some(1), puzzle.find_crate_index_at((7, 8)));
        assert_eq!(Some(2), puzzle.find_crate_index_at((9, 8)));
        assert_eq!(Some(3), puzzle.find_crate_index_at((12, 8)));
        assert_eq!(Some(4), puzzle.find_crate_index_at((8, 10)));
        assert_eq!(Some(5), puzzle.find_crate_index_at((9, 10)));
    }
    #[test]
    fn crate_coords() {
        let puzzle = puzzle();
        assert_eq!((12, 5), puzzle.crate_coords(0));
    }
    #[test]
    fn making_a_puzzle_move() {
        let mut puzzle = puzzle();
        assert_eq!((13, 14), puzzle.sokoban_coords());
        let result = puzzle.puzzle_move(Direction::Up);
        dbg!(&result);
        assert_eq!((12, 14), puzzle.sokoban_coords());
    }
    #[test]
    fn pushing_a_crate() {
        let mut puzzle = puzzle();
        assert_eq!((13, 14), puzzle.sokoban_coords());
        let moved_crate = puzzle
            .find_crate_index_at((12, 8))
            .expect("can't find crate");
        puzzle.puzzle_move(Direction::Up);
        puzzle.puzzle_move(Direction::Left);
        puzzle.puzzle_move(Direction::Left);
        puzzle.puzzle_move(Direction::Left);
        puzzle.puzzle_move(Direction::Left);
        puzzle.puzzle_move(Direction::Left);
        let result = puzzle.puzzle_move(Direction::Left);
        dbg!(&result);
        assert_eq!((12, 8), puzzle.sokoban_coords());
        assert_eq!(Element::Crate, puzzle.element_at((12, 7)));
        assert_eq!(Some(moved_crate), puzzle.find_crate_index_at((12, 7)));
        let result = puzzle.puzzle_move(Direction::Left);
        // this last move is not possible (blocked by a crate)
        let result = puzzle.puzzle_move(Direction::Left);
        println!("{}", puzzle.pretty_print());
        assert_eq!(None, result);
    }
    #[test]
    fn undoing_pushing_a_crate() {
        let mut puzzle = puzzle();
        assert_eq!((13, 14), puzzle.sokoban_coords());
        let moved_crate = puzzle
            .find_crate_index_at((12, 8))
            .expect("can't find crate");
        puzzle.puzzle_move(Direction::Up);
        puzzle.puzzle_move(Direction::Left);
        puzzle.puzzle_move(Direction::Left);
        puzzle.puzzle_move(Direction::Left);
        puzzle.puzzle_move(Direction::Left);
        puzzle.puzzle_move(Direction::Left);
        let result = puzzle.puzzle_move(Direction::Left);
        dbg!(&result);
        assert_eq!((12, 8), puzzle.sokoban_coords());
        assert_eq!(Element::Crate, puzzle.element_at((12, 7)));
        assert_eq!(Some(moved_crate), puzzle.find_crate_index_at((12, 7)));
        let pushing = puzzle.puzzle_move(Direction::Left)
            .expect("pushing not a valid move");

        assert_eq!((12, 7), puzzle.sokoban_coords());
        assert_eq!(Element::Crate, puzzle.element_at((12, 6)));
        println!("{}", puzzle.pretty_print());
        puzzle.puzzle_undo(pushing);
        println!("{}", puzzle.pretty_print());
        assert_eq!((12, 8), puzzle.sokoban_coords());
        assert_eq!(Element::Crate, puzzle.element_at((12, 7)));
        println!("{}", puzzle.pretty_print());
    }
}
