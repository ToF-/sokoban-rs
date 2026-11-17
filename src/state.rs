#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(usize)]
pub enum State {
    Crate = 0,
    Floor = 1,
    Goal = 2,
    OnGoal = 3,
    Outside = 4,
    Sokoban = 5,
    Stored = 6,
    Wall = 7,
}

impl From<u8> for State {
    fn from(byte: u8) -> Self {
        match byte {
            39 => State::Outside,
            32 => State::Floor,
            35 => State::Wall,
            46 => State::Goal,
            36 => State::Crate,
            42 => State::Stored,
            64 => State::Sokoban,
            43 => State::OnGoal,
            _ => State::Floor,
        }
    }
}

impl TryFrom<usize> for State {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(State::Crate),
            1 => Ok(State::Floor),
            2 => Ok(State::Goal),
            3 => Ok(State::OnGoal),
            4 => Ok(State::Outside),
            5 => Ok(State::Sokoban),
            6 => Ok(State::Stored),
            7 => Ok(State::Wall),
            _ => Err(()),
        }
    }
}
impl From<State> for char {
    fn from(state: State) -> Self {
        match state {
            State::Outside => '\'',
            State::Floor => ' ',
            State::Wall => '#',
            State::Crate => '$',
            State::Goal => '.',
            State::Stored => '*',
            State::Sokoban => '@',
            State::OnGoal => '+',
        }
    }
}

impl State {
    pub fn add_crate(self) -> State {
        match self {
            State::Floor => State::Crate,
            State::Goal => State::Stored,
            other => other,
        }
    }

    pub fn add_sokoban(self) -> State {
        match self {
            State::Floor => State::Sokoban,
            State::Goal => State::OnGoal,
            other => other,
        }
    }

    pub fn remove_crate(self) -> State {
        match self {
            State::Crate => State::Floor,
            State::Stored => State::Goal,
            other => other,
        }
    }

    pub fn remove_sokoban(self) -> State {
        match self {
            State::Sokoban => State::Floor,
            State::OnGoal => State::Goal,
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_floor_is_added_a_crate() {
        let s = State::Floor;
        assert_eq!(State::Crate, s.add_crate());
    }

    #[test]
    fn a_goal_is_added_a_crate() {
        let s = State::Goal;
        assert_eq!(State::Stored, s.add_crate());
    }

    #[test]
    fn a_floor_is_added_a_sokoban() {
        let s = State::Floor;
        assert_eq!(State::Sokoban, s.add_sokoban());
    }

    #[test]
    fn a_goal_is_added_a_sokoban() {
        let s = State::Goal;
        assert_eq!(State::OnGoal, s.add_sokoban());
    }

    #[test]
    fn other_additions_dont_work() {
        let s = State::Wall;
        assert_eq!(State::Wall, s.add_sokoban());
        let s = State::Crate;
        assert_eq!(State::Crate, s.add_sokoban());
    }
}
