use crate::HashMap;
use crate::Level;
use crate::Sprite;
use crate::State;
use crate::UNIT;
use crate::coords::Coords;
use crate::gdk::Paintable;
use crate::grid::Grid;
use crate::state::STATE_MAX;
use glib::ControlFlow;
use glib::clone;
use gtk::Fixed;
use gtk::Picture;
use gtk::gdk;
use gtk::gdk_pixbuf;
use gtk::gio;
use gtk::prelude::Cast;
use gtk::prelude::FixedExt;
use gtk::prelude::ObjectExt;
use gtk::prelude::WidgetExt;
use gtk::prelude::*;
use gtk::{Snapshot, graphene, WidgetPaintable, gsk};
use rand::Rng;
use rand::RngExt;
use rand::prelude::SliceRandom;
use rand::rng;
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

pub fn fill_tiles_for_level(
    level: &Level,
    crate_sprites: &Vec<Sprite>,
    grid_rc: Rc<RefCell<Grid>>,
    fixed: &gtk::Fixed,
) {
    let fixed = fixed.clone();
    let level = level.clone();
    let crate_sprites_rc = Rc::new(RefCell::new(crate_sprites.clone()));
    let mut all_coords: Vec<Coords> = Vec::new();
    for row in 0..level.height() {
        for col in 0..level.width() {
            all_coords.push((row, col))
        }
    }
    {
        let mut grid = grid_rc.borrow_mut();
        grid.clear();
    };

    all_coords.shuffle(&mut rng());
    let index_rc = Rc::new(Cell::new(0));
    let total = level.size();
    glib::timeout_add_local(
        Duration::from_millis(1),
        clone!(
            #[strong]
            grid_rc,
            #[strong]
            crate_sprites_rc,
            #[strong]
            index_rc,
            move || {
                let mut grid = grid_rc.borrow_mut();
                let mut crate_sprites = crate_sprites_rc.borrow_mut();
                let mut index = index_rc.get();
                // stop when all tiles are put
                if index >= total {
                    let result = save_fixed_to_png(&fixed, "/Users/tof/Coding/sokoban-rs/foo.png");
                    dbg!(&result);
                    glib::ControlFlow::Break
                } else {
                    let (row, col) = all_coords[index];
                    let state = level.state_at((row, col));
                    let mut crate_index_opt: Option<usize> = None;
                    let picture_state = match state {
                        // if state is crate, put the floor first, then the crate
                        State::Crate => {
                            crate_index_opt = crate_sprites
                                .iter()
                                .position(|sprite| sprite.coords() == ((row, col)));
                            State::Floor
                        }
                        // if the state is stored crate, put the goal first, then the crate
                        State::Stored => {
                            crate_index_opt = crate_sprites
                                .iter()
                                .position(|sprite| sprite.coords() == ((row, col)));
                            State::Goal
                        }
                        // if the state is sokoban don't put it, as it comes later
                        State::Sokoban => State::Floor,
                        State::OnGoal => State::Goal,
                        _ => state,
                    };
                    let mut sprite = Sprite::make_sprite(&state, (row, col));
                    if state != State::Crate && state != State::Stored && state != State::Sokoban {
                        sprite.put(&fixed, (row, col));
                    }
                    // grid.put(&sprite);
                    if let Some(crate_index) = crate_index_opt {
                        let mut sprite = &mut crate_sprites[crate_index];
                        // grid.put(&sprite)
                    };
                    index_rc.set(index + 1);
                    glib::ControlFlow::Continue
                }
            }
        ),
    );
}
pub fn picture_at(fixed: &gtk::Fixed, x: f64, y: f64) -> Option<Picture> {
    let mut child = fixed.first_child();

    while let Some(widget) = child {
        let next = widget.next_sibling();

        if widget.is::<gtk::Picture>() {
            let (child_x, child_y) = fixed.child_position(&widget);

            if child_x == x && child_y == y {
                let picture = widget
                    .downcast::<Picture>()
                    .expect("can't downcast widget to picture");
                return Some(picture.clone());
            }
        }

        child = next;
    }
    None
}


fn save_fixed_to_png(fixed: &gtk::Fixed, filename: &str) -> Result<(), glib::BoolError> {
    let width = fixed.width() as f32;
    let height = fixed.height() as f32;

    if width <= 0.0 || height <= 0.0 {
        eprintln!("Fixed has 0 size: {}x{}", width, height);
        return Ok(());
    }

    let snapshot = Snapshot::new();

    // Manually walk every child of Fixed and record it into the Snapshot
    let mut child = fixed.first_child();
    while let Some(widget) = child {
        if widget.is_visible() {
            let (x, y) = fixed.child_position(&widget);
            snapshot.save();
            snapshot.translate(&graphene::Point::new(x as f32, y as f32));

            // Use WidgetPaintable on each Picture (or snapshot_child if subclassed)
            let paintable = gtk::WidgetPaintable::new(Some(&widget));
            paintable.snapshot(&snapshot, widget.width() as f64, widget.height() as f64);

            snapshot.restore();
        }
        child = widget.next_sibling();
    }

    let Some(node) = snapshot.to_node() else {
        eprintln!("Snapshot produced no render node!");
        return Ok(());
    };

    let renderer = fixed
        .native()
        .expect("Fixed must be inside a realized window")
        .renderer()
        .expect("Surface must have a renderer");

    // Pass explicit bounds so transparent areas aren't cropped out
    let bounds = graphene::Rect::new(0.0, 0.0, width, height);
    let texture: gdk::Texture = renderer.render_texture(&node, Some(&bounds));

    texture.save_to_png(filename)
}

