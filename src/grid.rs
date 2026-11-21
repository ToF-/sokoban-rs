use crate::Sprite;
use crate::coords::Coords;
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

    pub fn put(&mut self, sprite: Sprite) {
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
        }
    }
}
