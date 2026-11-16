#[derive(PartialEq, Debug)]
pub enum State {
    Floor,
    Wall,
    Goal,
    Crate,
    Stored,
    Sokoban,
    OnGoal,
}

impl From<u8> for State {
    fn from(byte: u8) -> Self {
        match byte {
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

}
