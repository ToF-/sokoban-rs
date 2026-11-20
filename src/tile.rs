use crate::HashMap;
use crate::Level;
use crate::State;
use crate::UNIT;
use crate::gdk::Paintable;
use gtk::Picture;
use gtk::gdk;
use gtk::gdk_pixbuf;
use gtk::gio;
use gtk::prelude::FixedExt;
use gtk::prelude::WidgetExt;
use std::cell::OnceCell;
use std::sync::OnceLock;

pub struct Tile {
    paintable: Paintable,
}

impl Tile {
    pub fn make_tile(state: &State) -> Self {
        let tile_name = match state {
            State::Crate => "crate",
            State::Floor => "floor",
            State::Goal => "goal",
            State::Outside => "outside",
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
        let tiles = (0usize..15)
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

    for row in 0..level.height() {
        for col in 0..level.width() {
            let state = level.state_at((row, col));
            let mut background = State::Floor;
            let fixed_state = if state == State::Stored {
                State::Goal
            } else {
                state
            };
            if fixed_state == State::Wall || fixed_state == State::Goal {
                let picture = picture(&fixed_state);
                fixed.put(&picture, col as f64 * UNIT, row as f64 * UNIT);
            }
        }
    }
    fixed
}
