use crate::direction::Direction;
use crate::gdk::Key;
use crate::level::Level;
use crate::puzzle::Puzzle;
use crate::state::State;
use crate::tile::TILES;
use crate::tile::Tiles;
use crate::tile::paintable;
use crate::tile::picture;
use crate::tile::solid_picture;
use gtk::gdk;
use gtk::prelude::*;
use gtk::{Application, Picture, gio, glib};
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub mod direction;
pub mod level;
pub mod puzzle;
pub mod state;
pub mod tile;

const UNIT: f64 = 32.0;

fn main() -> glib::ExitCode {
    gio::resources_register_include!("myapp.gresource").expect("Failed to register resources.");

    let app = Application::builder()
        .application_id("com.example.MyApp")
        .build();

    app.connect_activate(|app| {
        let bytes = gio::resources_lookup_data(
            "/com/example/MyApp/files/level_01.txt",
            gio::ResourceLookupFlags::NONE,
        )
        .expect("Failed to find resource");
        let fixed = gtk::Fixed::new();
        let level = Level::from_bytes(&bytes);
        let mut sokoban_pos = (0.0, 0.0);
        let mut sokoban_paintable = paintable(&State::Sokoban);
        let mut crates: HashMap<(usize, usize), Picture> = HashMap::new();
        let background = solid_picture(
            level.width() as i32 * UNIT as i32,
            level.height() as i32 * UNIT as i32,
            0xdfd7cfff,
        );
        fixed.put(&background, 0.0, 0.0);

        for row in 0..level.height() {
            for col in 0..level.width() {
                let state = level.state_at(row, col);
                let mut background = State::Floor;
                if state == State::Sokoban {
                    // do not draw sokoban yet
                } else if state == State::Crate {
                    // do not draw crates yet
                    let picture = picture(&State::Crate);
                    crates.insert((row, col), picture);
                } else if state != State::Floor {
                    let picture = picture(&state);
                    fixed.put(&picture, col as f64 * UNIT, row as f64 * UNIT);
                }
            }
        }
        for (coords, picture) in &crates {
            let (row, col) = coords;
            fixed.put(picture, *col as f64 * UNIT, *row as f64 * UNIT);
        }
        let puzzle_rc = Rc::new(RefCell::new(Puzzle::from_level(&level)));
        let crates_rc = Rc::new(RefCell::new(crates));
        let sokoban_picture = Picture::for_paintable(&sokoban_paintable);
        sokoban_picture.set_halign(gtk::Align::Center);
        sokoban_picture.set_valign(gtk::Align::Center);
        sokoban_picture.set_can_shrink(false);

        let (row, col) = {
            let puzzle = puzzle_rc.borrow();
            puzzle.sokoban_position()
        };
        fixed.put(&sokoban_picture, col as f64 * UNIT, row as f64 * UNIT);

        let position = Cell::new(sokoban_pos);
        let coords = position.get();
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .default_width(level.width() as i32 * 32)
            .default_height(level.height() as i32 * 32)
            .child(&fixed)
            .build();

        let key_controller = gtk::EventControllerKey::new();
        key_controller.connect_key_pressed(glib::clone!(
            #[weak]
            app,
            #[weak]
            fixed,
            #[weak]
            sokoban_picture,
            #[strong]
            puzzle_rc,
            #[strong]
            crates_rc,
            #[strong]
            position,
            #[upgrade_or]
            glib::Propagation::Proceed,
            move |_controller, keyval, _keycode, state| {
                match keyval {
                    gdk::Key::q => {
                        app.quit();
                        glib::Propagation::Stop
                    }
                    gdk::Key::Right
                    | gdk::Key::r
                    | gdk::Key::Left
                    | gdk::Key::t
                    | gdk::Key::Down
                    | gdk::Key::s
                    | gdk::Key::Up
                    | gdk::Key::d => {
                        let direction = match keyval {
                            gdk::Key::Right | gdk::Key::r => Direction::Right,
                            gdk::Key::Left | gdk::Key::t => Direction::Left,
                            gdk::Key::Up | gdk::Key::d => Direction::Up,
                            gdk::Key::Down | gdk::Key::s => Direction::Down,
                            _ => Direction::Right,
                        };
                        let mut puzzle = puzzle_rc.borrow_mut();
                        if puzzle.is_solved() {
                            app.quit();
                            glib::Propagation::Stop
                        } else {
                            if puzzle.can_move(direction) {
                                let origin = puzzle.sokoban_position();
                                let destination = direction.from(origin);
                                let target_opt = puzzle.r#move(direction);
                                if let Some(target) = target_opt {
                                    let mut crates = crates_rc.borrow_mut();
                                    if let Some(crate_picture) = crates.remove(&destination) {
                                        crates.insert(target, crate_picture.clone());
                                        fixed.move_(
                                            &crate_picture,
                                            target.1 as f64 * UNIT,
                                            target.0 as f64 * UNIT,
                                        );
                                    };
                                }
                                let position = puzzle.sokoban_position();
                                fixed.move_(
                                    &sokoban_picture,
                                    position.1 as f64 * UNIT,
                                    position.0 as f64 * UNIT,
                                );
                            };
                            if puzzle.is_solved() {
                                let crates = crates_rc.borrow();
                                for (coords, _) in crates.iter() {
                                    let picture = picture(&State::Stored);
                                    fixed.put(
                                        &picture,
                                        coords.1 as f64 * UNIT,
                                        coords.0 as f64 * UNIT,
                                    );
                                }
                            };
                            glib::Propagation::Proceed
                        }
                    }
                    _ => glib::Propagation::Proceed,
                }
            }
        ));
        window.add_controller(key_controller);
        window.present();
    });

    app.run()
}
