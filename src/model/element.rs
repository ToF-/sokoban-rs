#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(usize)]
pub enum Element {
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

impl From<u8> for Element {
    fn from(byte: u8) -> Self {
        match byte {
            b'\'' => Element::Outside,
            b'-' => Element::Grass_1,
            b'|' => Element::Grass_2,
            b' ' => Element::Floor,
            b'#' => Element::Wall,
            b'.' => Element::Goal,
            b'$' => Element::Crate,
            b'*' => Element::Stored,
            b'@' => Element::Sokoban,
            b'+' => Element::OnGoal,
            b'A' => Element::Mark_A,
            b'B' => Element::Mark_B,
            b'C' => Element::Mark_C,
            b'-' => Element::Grass_1,
            b'|' => Element::Grass_2,
            _ => Element::Floor,
        }
    }
}

impl TryFrom<usize> for Element {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Element::Outside),
            1 => Ok(Element::Grass_1),
            2 => Ok(Element::Grass_2),
            3 => Ok(Element::Wall),
            4 => Ok(Element::Floor),
            5 => Ok(Element::Goal),
            6 => Ok(Element::Crate),
            7 => Ok(Element::Stored),
            8 => Ok(Element::Sokoban),
            9 => Ok(Element::Mark_A),
            10 => Ok(Element::Mark_B),
            11 => Ok(Element::Mark_C),
            12 => Ok(Element::Left),
            13 => Ok(Element::Up),
            14 => Ok(Element::Right),
            15 => Ok(Element::Down),
            16 => Ok(Element::OnGoal),
            _ => Err(()),
        }
    }
}
impl From<Element> for char {
    fn from(state: Element) -> Self {
        match state {
            Element::Outside => '\'',
            Element::Floor => ' ',
            Element::Wall => '#',
            Element::Crate => '$',
            Element::Goal => '.',
            Element::Stored => '*',
            Element::Sokoban => '@',
            Element::OnGoal => '+',
            Element::Mark_A => 'A',
            Element::Mark_B => 'B',
            Element::Mark_C => 'C',
            Element::Grass_1 => '-',
            Element::Grass_2 => '|',
            _ => ' ',
        }
    }
}

impl Element {
    pub fn cover(self, other: Element) -> Element {
        match self {
            Element::Floor => match other {
                Element::Crate => Element::Crate,
                Element::Sokoban => Element::Sokoban,
                _ => self,
            },
            Element::Goal => match other {
                Element::Crate => Element::Stored,
                Element::Sokoban => Element::OnGoal,
                _ => self,
            },
            _ => self,
        }
    }

    pub fn uncover(self) -> Element {
        match self {
            Element::Crate => Element::Floor,
            Element::Sokoban => Element::Floor,
            Element::Stored => Element::Goal,
            Element::OnGoal => Element::Goal,
            _ => self,
        }
    }

    pub fn z_order(self) -> usize {
        match self {
            Element::Outside => 0,
            Element::Floor => 0,
            Element::Wall => 0,
            Element::Crate => 2,
            Element::Goal => 1,
            Element::Stored => 2,
            Element::Sokoban => 2,
            Element::Down => 2,
            Element::Left => 2,
            Element::Up => 2,
            Element::Right => 2,
            Element::OnGoal => 2,
            Element::Mark_A => 1,
            Element::Mark_B => 1,
            Element::Mark_C => 1,
            Element::Grass_1 => 0,
            Element::Grass_2 => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn covering_an_element_with_a_crate_or_sokoban() {
        assert_eq!(Element::Crate, Element::Floor.cover(Element::Crate));
        assert_eq!(Element::Stored, Element::Goal.cover(Element::Crate));
        assert_eq!(Element::Sokoban, Element::Floor.cover(Element::Sokoban));
        assert_eq!(Element::OnGoal, Element::Goal.cover(Element::Sokoban));
    }
    #[test]
    fn covering_an_element_with_anything_else_is_inoperant() {
        assert_eq!(Element::Wall, Element::Wall.cover(Element::Crate));
        assert_eq!(Element::Floor, Element::Floor.cover(Element::Wall));
    }
    #[test]
    fn uncovering_an_element_from_a_crate_or_sokoban() {
        assert_eq!(Element::Floor, Element::Crate.uncover());
        assert_eq!(Element::Goal, Element::Stored.uncover());
        assert_eq!(Element::Floor, Element::Sokoban.uncover());
        assert_eq!(Element::Goal, Element::OnGoal.uncover());
    }
}
