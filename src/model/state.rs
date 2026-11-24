#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(usize)]
pub enum State {
    Outside = 0,
    Grass_1 = 1,
    Grass_2 = 2,
    Wall = 3,
    Floor = 4,
    Goal = 5,
    Crate = 6,
    Stored = 7,
    Sokoban = 8,
    Mark_A = 9,
    Mark_B = 10,
    Mark_C = 11,
    Left = 12,
    Up = 13,
    Right = 14,
    Down = 15,
    OnGoal = 16,
}

pub const STATE_MAX: usize = 17;

impl From<u8> for State {
    fn from(byte: u8) -> Self {
        match byte {
            b'\'' => State::Outside,
            b'-' => State::Grass_1,
            b'|' => State::Grass_2,
            b' ' => State::Floor,
            b'#' => State::Wall,
            b'.' => State::Goal,
            b'$' => State::Crate,
            b'*' => State::Stored,
            b'@' => State::Sokoban,
            b'+' => State::OnGoal,
            b'A' => State::Mark_A,
            b'B' => State::Mark_B,
            b'C' => State::Mark_C,
            b'-' => State::Grass_1,
            b'|' => State::Grass_2,
            _ => State::Floor,
        }
    }
}

impl TryFrom<usize> for State {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(State::Outside),
            1 => Ok(State::Grass_1),
            2 => Ok(State::Grass_2),
            3 => Ok(State::Wall),
            4 => Ok(State::Floor),
            5 => Ok(State::Goal),
            6 => Ok(State::Crate),
            7 => Ok(State::Stored),
            8 => Ok(State::Sokoban),
            9 => Ok(State::Mark_A),
            10 => Ok(State::Mark_B),
            11 => Ok(State::Mark_C),
            12 => Ok(State::Left),
            13 => Ok(State::Up),
            14 => Ok(State::Right),
            15 => Ok(State::Down),
            16 => Ok(State::OnGoal),
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
            State::Mark_A => 'A',
            State::Mark_B => 'B',
            State::Mark_C => 'C',
            State::Grass_1 => '-',
            State::Grass_2 => '|',
            _ => ' ',
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
            State::Crate => State::Sokoban,
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

    pub fn z_order(self) -> usize {
        match self {
            State::Outside => 0,
            State::Floor => 0,
            State::Wall => 0,
            State::Crate => 2,
            State::Goal => 1,
            State::Stored => 2,
            State::Sokoban => 2,
            State::Down => 2,
            State::Left => 2,
            State::Up => 2,
            State::Right => 2,
            State::OnGoal => 2,
            State::Mark_A => 1,
            State::Mark_B => 1,
            State::Mark_C => 1,
            State::Grass_1 => 0,
            State::Grass_2 => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[ignore]
    #[test]
    fn a_floor_is_added_a_crate() {
        let s = State::Floor;
        assert_eq!(State::Crate, s.add_crate());
    }

    #[ignore]
    #[test]
    fn a_goal_is_added_a_crate() {
        let s = State::Goal;
        assert_eq!(State::Stored, s.add_crate());
    }

    #[ignore]
    #[test]
    fn a_floor_is_added_a_sokoban() {
        let s = State::Floor;
        assert_eq!(State::Sokoban, s.add_sokoban());
    }

    #[ignore]
    #[test]
    fn a_goal_is_added_a_sokoban() {
        let s = State::Goal;
        assert_eq!(State::OnGoal, s.add_sokoban());
    }
    #[ignore]
    #[test]
    fn a_crate_is_added_a_sokoban() {
        let s = State::Crate;
        assert_eq!(State::Sokoban, s.add_sokoban());
    }

    #[ignore]
    #[test]
    fn other_additions_dont_work() {
        let s = State::Wall;
        assert_eq!(State::Wall, s.add_sokoban());
        let s = State::Crate;
        assert_eq!(State::Crate, s.add_crate());
    }
}
