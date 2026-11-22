use crate::HashMap;
use crate::Level;
use crate::Sprite;
use crate::State;
use crate::UNIT;
use crate::gdk::Paintable;
use crate::state::STATE_MAX;
use glib::ControlFlow;
use glib::clone;
use gtk::Fixed;
use gtk::Picture;
use gtk::gdk;
use gtk::gdk_pixbuf;
use gtk::gio;
use gtk::prelude::FixedExt;
use gtk::prelude::WidgetExt;
use rand::Rng;
use rand::RngExt;
use std::cell::Cell;
use std::cell::OnceCell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::OnceLock;
use std::time::Duration;

pub struct Tile {
    paintable: Paintable,
}

impl Tile {
    pub fn make_tile(state: &State) -> Self {
        let tile_name = match state {
            State::Crate => "crate",
            State::Floor => "floor",
            State::Goal => "goal",
            State::Outside => "grass_2",
            State::Sokoban => "sokoban",
            State::Stored => "stored",
            State::Wall => "wall",
            State::Down => "arrow_down",
            State::Right => "arrow_right",
            State::Up => "arrow_up",
            State::Left => "arrow_left",
            State::Mark_A => "mark_a",
            State::Mark_B => "mark_b",
            State::Mark_C => "mark_c",
            State::Grass_1 => "grass_1",
            State::Grass_2 => "grass_2",
            _ => "floor",
        };
        let file_name = format!("/com/example/MyApp/images/{}.png", tile_name);
        let picture = Picture::for_resource(&file_name);
        let paintable = picture.paintable().expect("can't access to paintable");
        Self {
            paintable: paintable.clone(),
        }
    }

    pub fn paintable(&self) -> Paintable {
        self.paintable.clone()
    }
}

pub struct Tiles {
    tiles: Vec<Tile>,
}

impl Tiles {
    pub fn from_picture_files() -> Self {
        let tiles = (0usize..STATE_MAX)
            .map(|index| {
                let state = State::try_from(index).expect("index out of bound for State");
                Tile::make_tile(&state)
            })
            .collect();
        Self { tiles }
    }

    pub fn paintable(&self, state: &State) -> Paintable {
        self.tiles[*state as usize].paintable()
    }
}
thread_local! {
pub static TILES: OnceCell<Tiles> = const { OnceCell::new() };
}

pub fn picture(state: &State) -> Picture {
    let paintable = paintable(state);
    let picture = Picture::for_paintable(&paintable);
    picture.set_halign(gtk::Align::Center);
    picture.set_valign(gtk::Align::Center);
    picture.set_can_shrink(false);
    picture
}

pub fn solid_picture(width: i32, height: i32, rgba: u32) -> gtk::Picture {
    let pixbuf = gdk_pixbuf::Pixbuf::new(
        gdk_pixbuf::Colorspace::Rgb,
        true, // has_alpha
        8,    // bits_per_sample
        width,
        height,
    )
    .expect("Failed to create Pixbuf");

    pixbuf.fill(rgba);

    let texture = gdk::Texture::for_pixbuf(&pixbuf);

    let picture = gtk::Picture::for_paintable(&texture);
    picture.set_can_shrink(false);
    picture
}

pub fn paintable(state: &State) -> Paintable {
    TILES.with(|cell| {
        let tiles = cell.get_or_init(Tiles::from_picture_files);
        tiles.paintable(state).clone()
    })
}

pub fn fixed_for_level(level: &Level) -> gtk::Fixed {
    let fixed = gtk::Fixed::new();
    let background = solid_picture(
        level.width() as i32 * UNIT as i32,
        level.height() as i32 * UNIT as i32,
        0xdfd7cfff,
    );
    fixed.put(&background, 0.0, 0.0);
    fixed
}

pub fn fill_tiles_for_level(level: &Level, crate_sprites: &Vec<Sprite>, fixed: Fixed) {
    let level = level.clone();
    let crate_sprites_rc = Rc::new(RefCell::new(crate_sprites.clone()));
    let done = vec![vec![false; level.width()]; level.height()];
    let done_rc = Rc::new(RefCell::new(done.clone()));
    let count_rc = Rc::new(Cell::new(0));
    let crate_count_rc = Rc::new(Cell::new(0));
    let fixed_squares_count = level.size() - level.crate_max() - 1;
    glib::timeout_add_local(
        Duration::from_millis(1),
        clone!(
            #[strong]
            crate_sprites_rc,
            #[strong]
            done_rc,
            #[strong]
            count_rc,
            #[strong]
            crate_count_rc,
            move || {
                let mut crate_sprites = crate_sprites_rc.borrow_mut();
                let mut done = done_rc.borrow_mut();
                let mut count = count_rc.get();
                let mut crate_count = crate_count_rc.get();
                // stop when all tiles are put
                if count >= level.size() {
                    glib::ControlFlow::Break
                } else {
                    // put the non-crate tiles first, because their z_order must be lower
                    if count < fixed_squares_count {
                        let mut rng = rand::rng();
                        let mut row: usize = rng.random_range(0..level.height());
                        let mut col: usize = rng.random_range(0..level.width());
                        let mut state = level.state_at((row, col));
                        if state == State::Stored {
                            state = State::Goal
                        };
                        while done[row][col] {
                            row = rng.random_range(0..level.height());
                            col = rng.random_range(0..level.width());
                            state = level.state_at((row, col));
                        }
                        let state = level.state_at((row, col));
                        let fixed_state = if state == State::Stored {
                            State::Goal
                        } else {
                            state
                        };
                        if fixed_state == State::Wall
                            || fixed_state == State::Goal
                            || fixed_state == State::Outside
                            || fixed_state == State::Grass_1
                            || fixed_state == State::Grass_2
                            || fixed_state == State::Floor
                        {
                            let picture = picture(&fixed_state);
                            fixed.put(&picture, col as f64 * UNIT, row as f64 * UNIT);
                            done[row][col] = true;
                            count += 1;
                            count_rc.set(count);
                        };
                        glib::ControlFlow::Continue
                    } else {
                        if crate_count < level.crate_max() {
                            let mut sprite = &mut crate_sprites[crate_count];
                            sprite.place(&fixed);
                            crate_count += 1;
                            count += 1;
                            crate_count_rc.set(crate_count);
                            count_rc.set(count);
                            glib::ControlFlow::Continue
                        } else {
                            glib::ControlFlow::Break
                        }
                    }
                }
            }
        ),
    );
}
