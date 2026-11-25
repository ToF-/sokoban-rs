use crate::Direction;
use crate::Puzzle;
use crate::model::basic_step::BasicStep;
use crate::model::basic_step::step_rules;
use crate::model::coords::Coords;
use crate::model::coords::adjacent;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PuzzleMove {
    pub basic_steps: Vec<BasicStep>,
}

impl PuzzleMove {
    pub fn new(puzzle: &Puzzle, origin_coords: Coords, direction: Direction) -> Option<Self> {
        adjacent(origin_coords, direction).and_then(|destination_coords| {
            let target_coords = adjacent(destination_coords, direction)
                .expect("location out of reach adjacent to a reachable location");
            let origin = puzzle.element_at(origin_coords);
            let destination = puzzle.element_at(destination_coords);
            let target = puzzle.element_at(target_coords);
            step_rules(&(destination, target)).and_then(|step1| {
                dbg!(&destination_coords, &target_coords, &step1);
                let (destination_, _) = step1;
                step_rules(&(origin, destination_)).and_then(|step2| {
                    dbg!(&origin_coords, &destination_coords, &step2);
                    let basic_step_1 =
                        BasicStep::new(puzzle, destination_coords, target_coords).unwrap();
                    let basic_step_2 =
                        BasicStep::new(puzzle, origin_coords, destination_coords).unwrap();
                    if basic_step_1.is_identical() {
                        Some(Self {
                            basic_steps: vec![basic_step_2],
                        })
                    } else {
                        Some(Self {
                            basic_steps: vec![basic_step_1, basic_step_2],
                        })
                    }
                })
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::direction::Direction;
    use crate::model::element::Element;
    use crate::model::level::Level;
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
    fn move_towards_a_wall_is_none() {
        let puzzle = puzzle();
        let coords = puzzle.sokoban_coords();
        assert!(PuzzleMove::new(&puzzle, coords, Direction::Left).is_none());
    }
    #[test]
    fn walking_up_is_some_move() {
        let puzzle = puzzle();
        let coords = puzzle.sokoban_coords();
        assert_eq!(
            Some(PuzzleMove {
                basic_steps: vec![BasicStep {
                    from: (13, 14),
                    to: (12, 14),
                    origin_before: Element::Sokoban,
                    origin_after: Element::Floor,
                    destination_before: Element::Floor,
                    destination_after: Element::Sokoban
                }]
            }),
            PuzzleMove::new(&puzzle, coords, Direction::Up)
        );
    }
    #[test]
    fn walking_to_a_crate_and_pushing_it() {
        let mut puzzle = puzzle();
        puzzle.puzzle_move(Direction::Up);
        puzzle.puzzle_move(Direction::Left);
        puzzle.puzzle_move(Direction::Left);
        puzzle.puzzle_move(Direction::Left);
        puzzle.puzzle_move(Direction::Left);
        puzzle.puzzle_move(Direction::Left);
        let coords = puzzle.sokoban_coords();
        assert_eq!(
            Some(PuzzleMove {
                basic_steps: vec![BasicStep {
                    from: (1, 14),
                    to: (12, 14),
                    origin_before: Element::Sokoban,
                    origin_after: Element::Floor,
                    destination_before: Element::Floor,
                    destination_after: Element::Sokoban
                }]
            }),
            PuzzleMove::new(&puzzle, coords, Direction::Left)
        );
        assert!(true);
    }
}
