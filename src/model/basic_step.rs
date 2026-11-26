use crate::Level;
use crate::Puzzle;
use crate::model::coords::Coords;
use crate::model::coords::adjacent;
use crate::model::element::ELEMENT_MAX;
use crate::model::element::Element;
use std::collections::HashMap;
use std::sync::LazyLock;

pub static STEP_RULES: LazyLock<HashMap<(Element, Element), (Element, Element)>> =
    LazyLock::new(|| {
        let mut result: HashMap<(Element, Element), (Element, Element)> = HashMap::new();
        let mut immobile: HashMap<(Element, Element), (Element, Element)> = HashMap::new();
        for space_element in [Element::Floor, Element::Goal] {
            for value in 0..ELEMENT_MAX {
                if let Ok(adjacent_element) = Element::try_from(value) {
                    let pair = (space_element, adjacent_element);
                    let _ = immobile.insert(pair.clone(), pair.clone());
                }
            }
        }
        result = HashMap::from([
            (
                (Element::Sokoban, Element::Floor),
                (Element::Floor, Element::Sokoban),
            ),
            (
                (Element::OnGoal, Element::Floor),
                (Element::Goal, Element::Sokoban),
            ),
            (
                (Element::Sokoban, Element::Goal),
                (Element::Floor, Element::OnGoal),
            ),
            (
                (Element::OnGoal, Element::Goal),
                (Element::Goal, Element::OnGoal),
            ),
            (
                (Element::Crate, Element::Floor),
                (Element::Floor, Element::Crate),
            ),
            (
                (Element::Stored, Element::Floor),
                (Element::Goal, Element::Crate),
            ),
            (
                (Element::Crate, Element::Goal),
                (Element::Floor, Element::Stored),
            ),
            (
                (Element::Stored, Element::Goal),
                (Element::Goal, Element::Stored),
            ),
        ]);
        result.extend(&immobile);
        result
    });

pub fn step_rules(before: &(Element, Element)) -> Option<(Element, Element)> {
    STEP_RULES.get(before).copied()
}

pub type Move = Vec<BasicStep>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasicStep {
    pub from: Coords,
    pub to: Coords,
    pub origin_before: Element,
    pub origin_after: Element,
    pub destination_before: Element,
    pub destination_after: Element,
}

impl BasicStep {
    pub fn new(puzzle: &Puzzle, from: Coords, to: Coords) -> Option<Self> {
        let origin_before = puzzle.element_at(from);
        let destination_before = puzzle.element_at(to);
        if let Some((origin_after, destination_after)) =
            step_rules(&(origin_before, destination_before))
        {
            Some(Self {
                from,
                to,
                origin_before,
                origin_after: origin_after,
                destination_before,
                destination_after: destination_after,
            })
        } else {
            None
        }
    }

    pub fn is_identical(&self) -> bool {
        self.origin_after == self.origin_before && self.destination_after == self.destination_before
    }

    pub fn before(&self) -> (Element, Element) {
        (self.origin_before, self.destination_before)
    }

    pub fn after(&self) -> (Element, Element) {
        (self.origin_after, self.destination_after)
    }

    pub fn reverse(&self) -> Self {
        Self {
            from: self.to,
            to: self.from,
            origin_before: self.origin_before,
            origin_after: self.origin_after,
            destination_before: self.destination_before,
            destination_after: self.destination_after,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::direction::Direction;
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
    fn basic_steps_for_a_wall_is_none() {
        let puzzle = puzzle();
        let coords = puzzle.sokoban_coords();
        let coords_left = adjacent(coords, Direction::Left).expect("can't access adjacent");
        assert!(BasicStep::new(&puzzle, coords, coords_left).is_none());
    }
    #[test]
    fn basic_steps_for_a_floor_or_a_goal_keep_things_in_place() {
        let puzzle = puzzle();
        let coords = puzzle.sokoban_coords();
        let coords_up = adjacent(coords, Direction::Up).expect("can't access adjacent");
        let expected = Some(BasicStep {
            from: (12, 14),
            to: (13, 14),
            origin_before: Element::Floor,
            origin_after: Element::Floor,
            destination_before: Element::Sokoban,
            destination_after: Element::Sokoban,
        });
        assert_eq!(expected, BasicStep::new(&puzzle, coords_up, coords));

        let goal = (12, 20);
        assert_eq!(Element::Goal, puzzle.element_at(goal));
        let goal_right = adjacent(goal, Direction::Right).expect("can't access adjacent");
        let expected = Some(BasicStep {
            from: (12, 20),
            to: (12, 21),
            origin_before: Element::Goal,
            origin_after: Element::Goal,
            destination_before: Element::Wall,
            destination_after: Element::Wall,
        });
        assert_eq!(expected, BasicStep::new(&puzzle, goal, goal_right));
    }
    #[test]
    fn basic_steps_for_walking_on_the_floor() {
        let puzzle = puzzle();
        let coords = puzzle.sokoban_coords();
        let coords_up = adjacent(coords, Direction::Up).expect("can't access adjacent");
        let step_opt = BasicStep::new(&puzzle, coords, coords_up);
        assert!(step_opt.is_some());
        let step = step_opt.unwrap();
        assert_eq!((Element::Sokoban, Element::Floor), step.before());
        assert_eq!((Element::Floor, Element::Sokoban), step.after());
    }
    #[test]
    fn basic_steps_for_walking_on_a_goal() {
        let mut puzzle = puzzle();
        puzzle.set_element_at((12, 18), Element::Sokoban);
        puzzle.set_element_at((13, 14), Element::Floor);
        let coords = puzzle.sokoban_coords();
        let coords_right = adjacent(coords, Direction::Right).expect("can't access adjacent");
        let step_opt = BasicStep::new(&puzzle, coords, coords_right);
        assert!(step_opt.is_some());
        let step = step_opt.unwrap();
        assert_eq!((Element::Sokoban, Element::Goal), step.before());
        assert_eq!((Element::Floor, Element::OnGoal), step.after());

        puzzle.set_element_at((12, 19), Element::OnGoal);
        puzzle.set_element_at((12, 18), Element::Floor);
        let coords = puzzle.sokoban_coords();
        let coords_right = adjacent(coords, Direction::Right).expect("can't access adjacent");
        let step_opt = BasicStep::new(&puzzle, coords, coords_right);
        assert!(step_opt.is_some());
        let step = step_opt.unwrap();
        assert_eq!((Element::OnGoal, Element::Goal), step.before());
        assert_eq!((Element::Goal, Element::OnGoal), step.after());
    }
    #[test]
    fn reverse_walking_from_floor() {
        let mut puzzle = puzzle();
        let coords = puzzle.sokoban_coords();
        let coords_up = adjacent(coords, Direction::Up).expect("can't access adjacent");
        let _ = BasicStep::new(&puzzle, coords, coords_up).and_then(|step| {
            println!("{:?}", step);
            let pets = step.reverse();
            println!("{:?}", pets);
            assert_eq!((Element::Sokoban, Element::Floor), pets.before());
            assert_eq!((Element::Floor, Element::Sokoban), pets.after());
            Some(pets)
        });

        puzzle.set_element_at((12, 18), Element::Sokoban);
        puzzle.set_element_at((13, 14), Element::Floor);
        let move_opt = puzzle.puzzle_move(Direction::Right);
        let expected = Some(vec![BasicStep {
            from: (12, 18),
            to: (12, 19),
            origin_before: Element::Sokoban,
            origin_after: Element::Floor,
            destination_before: Element::Goal,
            destination_after: Element::OnGoal,
        }]);
        assert_eq!(expected, move_opt);
    }
}
