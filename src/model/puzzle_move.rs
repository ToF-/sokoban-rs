use crate::model::coords::Coords;
pub type PuzzleMove = (Coords, Coords);

pub fn move_(coords: Coords, destination: Coords) -> PuzzleMove {
    (coords, destination)
}

pub fn reverse_move(puzzle_move: &PuzzleMove) -> PuzzleMove {
    let (coords, destination) = puzzle_move;
    (*destination, *coords)
}
