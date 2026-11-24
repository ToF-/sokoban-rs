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
    Down = 8,
    Left = 9,
    Up = 10,
    Right = 11,
    Mark_A = 12,
    Mark_B = 13,
    Mark_C = 14,
    Grass_1 = 15,
    Grass_2 = 16,
}

pub const STATE_MAX: usize = 17;

impl From<u8> for State {
    fn from(byte: u8) -> Self {
        match byte {
            b'\'' => State::Outside,
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
            0 => Ok(State::Crate),
            1 => Ok(State::Floor),
            2 => Ok(State::Goal),
            3 => Ok(State::OnGoal),
            4 => Ok(State::Outside),
            5 => Ok(State::Sokoban),
            6 => Ok(State::Stored),
            7 => Ok(State::Wall),
            8 => Ok(State::Down),
            9 => Ok(State::Left),
            10 => Ok(State::Up),
            11 => Ok(State::Right),
            12 => Ok(State::Mark_A),
            13 => Ok(State::Mark_B),
            14 => Ok(State::Mark_C),
            15 => Ok(State::Grass_1),
            16 => Ok(State::Grass_2),
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
