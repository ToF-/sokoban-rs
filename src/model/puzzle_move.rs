use crate::model::coords::Coords;
pub type PuzzleMove = (Coords, Coords);

pub fn move_(coords: Coords, destination: Coords) -> PuzzleMove {
    (coords, destination)
}
