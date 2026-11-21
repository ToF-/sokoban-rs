use crate::Picture;
use crate::State;
use crate::UNIT;
use crate::coords::Coords;
use crate::paintable;
use crate::picture;
use gtk::Fixed;
use gtk::prelude::WidgetExt;
use gtk::prelude::*;

#[derive(Clone, Debug)]
pub struct Sprite {
    state: State,
    picture: Picture,
    coords: Coords,
    z_order: usize,
}

impl Sprite {
    pub fn make_sprite(state: &State, coords: Coords) -> Self {
        let picture = picture(&state);
        Self {
            state: state.clone(),
            picture: picture,
            coords: coords,
            z_order: state.z_order(),
        }
    }

    pub fn z_order(&self) -> usize {
        self.z_order
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

    pub fn remove(&mut self, fixed: &Fixed) {
        fixed.remove(&self.picture);
    }

    pub fn set_picture(&mut self, state: &State) {
        self.picture = picture(&state);
    }

    pub fn coords(&self) -> Coords {
        self.coords
    }
}
