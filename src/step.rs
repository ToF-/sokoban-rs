use crate::Direction;
use crate::model::coords::Coords;

#[derive(Clone, PartialEq, Debug)]
pub struct Step {
    from: Coords,
    to: Coords,
    pushing: Option<(usize, Box<Step>)>,
}

impl Step {
    pub fn new(origin: Coords, direction: Direction, pushing: Option<usize>) -> Self {
        let destination = direction.from(origin);
        Self {
            from: origin,
            to: destination,
            pushing: match pushing {
                None => None,
                Some(crate_index) => Some((
                    crate_index,
                    Box::new(Self {
                        from: destination,
                        to: direction.from(destination),
                        pushing: None,
                    }),
                )),
            },
        }
    }

    pub fn from(&self) -> Coords {
        self.from
    }

    pub fn to(&self) -> Coords {
        self.to
    }

    pub fn pushing(&self) -> Option<(usize, Box<Step>)> {
        self.pushing.clone()
    }

    pub fn reverse(&self) -> Self {
        let origin = self.from();
        let destination = self.to();
        Self {
            from: destination,
            to: origin,
            pushing: match self.pushing() {
                None => None,
                Some((crate_index, step)) => Some((crate_index, Box::new(step.reverse()))),
            },
        }
    }
}
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn reversing_a_simple_step() {
        let step = Step::new((3, 8), Direction::Up, None);
        assert_eq!((3, 8), step.from());
        assert_eq!((2, 8), step.to());
        assert_eq!(None, step.pushing());
    }

    #[test]
    fn reversing_a_pushing_step() {
        // stepping to left from 3,8 to 3,7, pushing crate #7 from 3,7 to 3,6
        let step = Step::new((3, 8), Direction::Left, Some(7));
        let reversed = step.reverse();
        assert_eq!((3, 7), reversed.from());
        assert_eq!((3, 8), reversed.to());
        let (crate_index, boxed_crate_step) =
            reversed.pushing().expect("failed to get pushing step");
        assert_eq!(7, crate_index);
        let crate_step = *boxed_crate_step;
        // crate has to return from 3,6 to 3,7
        assert_eq!((3, 6), crate_step.from());
        assert_eq!((3, 7), crate_step.to());
    }
}
