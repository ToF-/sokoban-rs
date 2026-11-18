use gtk::prelude::FixedExt;
use crate::UNIT;
use gtk::Fixed;
use crate::picture;
use crate::paintable;
use crate::coords::Coords;
use crate::Picture;
use crate::State;

#[derive(Clone, Debug)]
pub struct Sprite {
    state: State,
    picture: Picture,
    coords: Coords,
}

impl Sprite {
    pub fn make_sprite(state: &State, coords: Coords) -> Self {
        let picture = picture(&state);
        Self {
            state: state.clone(),
            picture: picture,
            coords: coords,
        }
    }

    pub fn r#move(&mut self, fixed: &Fixed, coords: Coords) {
        self.coords = coords;
        fixed.move_(
            &self.picture,
            coords.1 as f64 * UNIT,
            coords.0 as f64 * UNIT,
        )
    }

    pub fn place(&mut self, fixed: &Fixed) {
        fixed.put(
            &self.picture,
            self.coords.1 as f64 * UNIT,
            self.coords.0 as f64 * UNIT,
        )
    }

    pub fn put(&mut self, fixed: &Fixed, coords: Coords) {
        self.coords = coords;
        fixed.put(
            &self.picture,
            coords.1 as f64 * UNIT,
            coords.0 as f64 * UNIT,
        )
    }

    pub fn coords(&self) -> Coords {
        self.coords
    }

}
