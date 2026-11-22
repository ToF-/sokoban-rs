use crate::Sprite;
use crate::UNIT;
use crate::coords::Coords;
use crate::tile::picture_at;
use gtk::prelude::FixedExt;
use gtk::prelude::WidgetExt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Grid {
    width: usize,
    height: usize,
    sprites: HashMap<Coords, Rc<RefCell<Vec<Sprite>>>>,
    screen: gtk::Fixed,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            sprites: HashMap::new(),
            screen: gtk::Fixed::new(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn screen(&self) -> gtk::Fixed {
        self.screen.clone()
    }

    pub fn put(&mut self, sprite: &Sprite) {
        let coords = sprite.coords();
        let z_order = sprite.z_order();
        if let Some(background_sprites_rc) = self.sprites.get(&coords) {
            let mut background_sprites = background_sprites_rc.borrow_mut();
            background_sprites.push(sprite.clone());
            background_sprites.sort_by(|a, b| a.z_order().cmp(&b.z_order()));
        } else {
            let mut background_sprites = Vec::new();
            background_sprites.push(sprite.clone());
            let background_sprites_rc = Rc::new(RefCell::new(background_sprites));
            let _ = self.sprites.insert(coords, background_sprites_rc);
        };
        let (x, y) = (coords.1 as f64 * UNIT, coords.0 as f64 * UNIT);
        while let Some(picture) = picture_at(&self.screen, x, y) {
            self.screen.remove(&picture);
        }
        if let Some(background_sprites_rc) = self.sprites.get(&coords) {
            let mut sprites_at_coord = background_sprites_rc.borrow_mut();
            for mut sprite in sprites_at_coord.iter_mut() {
                sprite.place(&self.screen)
            }
        }
    }

    pub fn clear(&mut self) {
        self.sprites = HashMap::new();
        let mut child = self.screen.first_child();
        while let Some(widget) = child {
            let next = widget.next_sibling();
            self.screen.remove(&widget);
            child = next;
        }
    }
}
