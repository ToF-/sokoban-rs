pub type Coords = (usize, usize);

pub fn row(coords: Coords) -> usize {
    coords.0
}

pub fn col(coords: Coords) -> usize {
    coords.1
}
