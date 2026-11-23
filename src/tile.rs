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
use gtk::{Snapshot, WidgetPaintable, graphene, gsk};
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
    picture.set_can_shrink(false);
    picture.set_size_request(32, 32);
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
    fixed_rc: Rc<RefCell<Fixed>>,
    crate_sprites_rc: Rc<RefCell<Vec<Sprite>>>,
    sokoban_sprite_rc: Rc<RefCell<Sprite>>,
    level_ready_rc: Rc<Cell<bool>>,
) {
    let level = level.clone();
    {
        let fixed = fixed_rc.borrow_mut();
        clear_fixed(&fixed);
    }
    let mut all_coords: Vec<Coords> = Vec::new();
    for row in 0..level.height() {
        for col in 0..level.width() {
            all_coords.push((row, col))
        }
    }
    all_coords.shuffle(&mut rng());
    let index_rc = Rc::new(Cell::new(0));
    let total = level.size();
    let background_ready_rc = Rc::new(Cell::new(false));
    glib::timeout_add_local(
        Duration::from_millis(10),
        clone!(
            #[strong]
            fixed_rc,
            #[strong]
            index_rc,
            #[strong]
            background_ready_rc,
            move || {
                let mut index = index_rc.get();
                let mut background_ready = background_ready_rc.get();
                let (visible_crate_count, crate_total) = {
                    let crate_sprites = crate_sprites_rc.borrow();
                    (
                        crate_sprites
                            .iter()
                            .filter(|sprite| sprite.is_visible())
                            .count(),
                        crate_sprites.len(),
                    )
                };
                // stop when all tiles are put and snapshot a texture for background
                if index >= total && !background_ready {
                    let background_texture = background_texture(fixed_rc.clone());
                    if let Ok(fixed) = fixed_rc.try_borrow_mut() {
                        clear_fixed(&fixed);
                        let background_picture = gtk::Picture::for_paintable(&background_texture);
                        background_picture.set_size_request(
                            background_texture.width(),
                            background_texture.height(),
                        );
                        fixed.put(&background_picture, 0.0, 0.0);
                        background_ready_rc.set(true);
                    } else {
                        println!("can't borrow mutably fixed");
                    };
                    dbg!("foo");
                    glib::ControlFlow::Continue
                } else if index >= total && background_ready && visible_crate_count < crate_total {
                    if let Ok(fixed) = fixed_rc.try_borrow_mut() {
                        let mut crate_sprites = crate_sprites_rc.borrow_mut();
                        let sprite_index = crate_sprites
                            .iter()
                            .position(|sprite| !sprite.is_visible())
                            .expect("can't find a not yet placed sprite");
                        let mut sprite = &mut crate_sprites[sprite_index];
                        sprite.place(&fixed);
                    } else {
                        println!("can't borrow mutably fixed");
                    }
                    glib::ControlFlow::Continue
                } else if background_ready && visible_crate_count == crate_total {
                    if let Ok(mut sokoban_sprite) = sokoban_sprite_rc.try_borrow_mut() {
                        if let Ok(fixed) = fixed_rc.try_borrow_mut() {
                            sokoban_sprite.place(&fixed);
                            level_ready_rc.set(true);
                            let data = picture_data(&fixed);
                            glib::ControlFlow::Break
                        } else {
                            println!("can't borrow mutably fixed");
                            glib::ControlFlow::Continue
                        }
                    } else {
                        println!("can't mutably borrow sokoban_sprite");
                        glib::ControlFlow::Continue
                    }
                } else {
                    if let Ok(fixed) = fixed_rc.try_borrow_mut() {
                        let (row, col) = all_coords[index];
                        let state = level.state_at((row, col));
                        let mut crate_index_opt: Option<usize> = None;
                        let picture_state = match state {
                            // if state is crate, put the floor first, then the crate
                            State::Crate => State::Floor,
                            State::Stored => State::Goal,
                            State::Sokoban => State::Floor,
                            State::OnGoal => State::Goal,
                            _ => state,
                        };
                        if picture_state != State::Crate
                            && picture_state != State::Stored
                            && picture_state != State::Sokoban
                        {
                            let (x, y) = (col as f64 * UNIT, row as f64 * UNIT);
                            let picture = picture(&picture_state);
                            fixed.put(&picture, x, y);
                            let picture_data = picture_data(&fixed);
                            index_rc.set(index + 1);
                        }
                    } else {
                        println!("can't borrow mutably fixed");
                    };
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

fn background_texture(fixed_rc: Rc<RefCell<Fixed>>) -> gdk::Texture {
    if let Ok(fixed) = fixed_rc.try_borrow_mut() {
        let width = fixed.width() as f32;
        let height = fixed.height() as f32;

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
            panic!("can't snapshot.to_node()")
        };

        let renderer = fixed
            .native()
            .expect("Fixed must be inside a realized window")
            .renderer()
            .expect("Surface must have a renderer");

        // Pass explicit bounds so transparent areas aren't cropped out
        let bounds = graphene::Rect::new(0.0, 0.0, width, height);
        let texture: gdk::Texture = renderer.render_texture(&node, Some(&bounds));
        texture
    } else {
        panic!("can't borrow mutably fixed")
    }
}

fn picture_data(fixed: &gtk::Fixed) -> Vec<(f64, f64, bool)> {
    let mut result = Vec::new();
    let mut child = fixed.first_child();
    while let Some(widget) = child {
        let (x, y) = fixed.child_position(&widget);
        let visible = widget.is_visible();
        child = widget.next_sibling();
        let picture = widget.downcast::<gtk::Picture>().expect("not a picture");
        result.push((x, y, visible))
    }
    result.sort_by(|a, b| {
        a.1.total_cmp(&b.1)
            .then_with(|| a.0.total_cmp(&b.0))
            .then_with(|| a.2.cmp(&b.2))
    });
    result
}

fn clear_fixed(fixed: &gtk::Fixed) {
    let mut child = fixed.first_child();
    while let Some(widget) = child {
        child = widget.next_sibling();
        fixed.remove(&widget);
    }
}
