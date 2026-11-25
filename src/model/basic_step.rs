use crate::Level;
use crate::Puzzle;
use crate::model::coords::Coords;
use crate::model::coords::adjacent;
use crate::model::element::Element;
use std::collections::HashMap;
use std::sync::LazyLock;

pub static STEP_RULES: LazyLock<HashMap<&'static (Element, Element), (Element, Element)>> =
    LazyLock::new(|| {
        HashMap::from([
            (
                &(Element::Sokoban, Element::Floor),
                (Element::Floor, Element::Sokoban),
            ),
            (
                &(Element::OnGoal, Element::Floor),
                (Element::Goal, Element::Sokoban),
            ),
            (
                &(Element::Sokoban, Element::Goal),
                (Element::Floor, Element::OnGoal),
            ),
            (
                &(Element::OnGoal, Element::Goal),
                (Element::Goal, Element::OnGoal),
            ),
            (
                &(Element::Crate, Element::Floor),
                (Element::Floor, Element::Crate),
            ),
            (
                &(Element::Stored, Element::Floor),
                (Element::Goal, Element::Crate),
            ),
            (
                &(Element::Crate, Element::Goal),
                (Element::Floor, Element::Stored),
            ),
            (
                &(Element::Stored, Element::Goal),
                (Element::Goal, Element::Stored),
            ),
        ])
    });

pub fn step_rules(before: &(Element, Element)) -> Option<(Element, Element)> {
    let (from, to) = before;
    if *from == Element::Floor || *from == Element::Goal {
        Some(before.clone())
    } else if *to == Element::Wall || *to == Element::Crate || *to == Element::Stored {
        None
    } else {
        STEP_RULES.get(before).copied()
    }
}

#[derive(Clone, Debug)]
pub struct BasicStep {
    from: Coords,
    to: Coords,
    origin_before: Element,
    origin_after: Element,
    destination_before: Element,
    destination_after: Element,
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
            origin_before: self.origin_after,
            origin_after: self.origin_before,
            destination_before: self.destination_after,
            destination_after: self.destination_before,
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
    fn basic_steps_for_walking() {
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
    fn reverse_of_walking() {
        let puzzle = puzzle();
        let coords = puzzle.sokoban_coords();
        let coords_up = adjacent(coords, Direction::Up).expect("can't access adjacent");
        let step_opt = BasicStep::new(&puzzle, coords, coords_up);
        assert!(step_opt.is_some());
        let step = step_opt.unwrap();
        let reverse = step.reverse();
        assert_eq!((Element::Floor, Element::Sokoban), reverse.before());
        assert_eq!((Element::Sokoban, Element::Floor), reverse.after());
    }
}
