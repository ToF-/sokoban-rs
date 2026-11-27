use crate::model::coords::Coords;
pub type Move_ = (Coords, Coords);

pub fn move_(coords: Coords, destination: Coords) -> Move_ {
    (coords, destination)
}

pub fn reverse_move(puzzle_move: &Move_) -> Move_ {
    let (coords, destination) = puzzle_move;
    (*destination, *coords)
}
