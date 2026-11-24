use crate::Picture;
use crate::UNIT;
use crate::model::coords::Coords;
use crate::model::state::State;
use crate::paintable;
use crate::picture;
use crate::tile::picture_at;
use gtk::Fixed;
use gtk::prelude::WidgetExt;
use gtk::prelude::*;

#[derive(Clone, Debug)]
pub struct Sprite {
    state: State,
    picture: Picture,
    coords: Coords,
    z_order: usize,
    is_visible: bool,
}

impl Sprite {
    pub fn make_sprite(state: &State, coords: Coords) -> Self {
        let picture = picture(&state);
        Self {
            state: state.clone(),
            picture: picture,
            coords: coords,
            z_order: state.z_order(),
            is_visible: false,
        }
    }

    pub fn z_order(&self) -> usize {
        self.z_order
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn r#move(&mut self, fixed: &Fixed, coords: Coords) {
        self.coords = coords;
        let (x, y) = (coords.1 as f64 * UNIT, coords.0 as f64 * UNIT);
        if let Some(previous) = picture_at(fixed, x, y) {
            fixed.remove(&previous);
            fixed.put(&previous, x, y)
        };
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
        );
        self.is_visible = true;
    }

    pub fn put(&mut self, fixed: &Fixed, coords: Coords) {
        self.coords = coords;
        fixed.put(
            &self.picture,
            coords.1 as f64 * UNIT,
            coords.0 as f64 * UNIT,
        );
        self.is_visible = true;
    }

    pub fn remove(&mut self, fixed: &Fixed) {
        fixed.remove(&self.picture);
        self.is_visible = false;
    }

    pub fn set_picture(&mut self, state: &State) {
        self.picture = picture(&state);
    }

    pub fn coords(&self) -> Coords {
        self.coords
    }
}
