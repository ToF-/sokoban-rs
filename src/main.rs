use crate::direction::Direction;
use std::cell::RefCell;
use std::rc::Rc;
use crate::puzzle::Puzzle;
use crate::gdk::Key;
use crate::level::Level;
use crate::state::State;
use crate::tile::TILES;
use crate::tile::Tiles;
use crate::tile::paintable;
use gtk::gdk;
use gtk::prelude::*;
use gtk::{Application, Picture, gio, glib};
use std::cell::Cell;

pub mod direction;
pub mod level;
pub mod puzzle;
pub mod state;
pub mod tile;

const MOVE_LENGTH: f64 = 32.0;

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

        for row in 0..level.height() {
            for col in 0..level.width() {
                let state = level.state_at(row, col);
                let mut state_to_paint = State::Floor;
                if state == State::Sokoban {
                    sokoban_pos = (col as f64 * 32.0, row as f64 * 32.0);
                } else {
                    state_to_paint = state;
                }
                let paintable = paintable(&state_to_paint);
                let picture = Picture::for_paintable(&paintable);
                picture.set_halign(gtk::Align::Center);
                picture.set_valign(gtk::Align::Center);
                picture.set_can_shrink(false);
                let x: f64 = col as f64 * 32.0;
                let y: f64 = row as f64 * 32.0;
                fixed.put(&picture, x, y);
            }
        }
        let sokoban_picture = Picture::for_paintable(&sokoban_paintable);
        sokoban_picture.set_halign(gtk::Align::Center);
        sokoban_picture.set_valign(gtk::Align::Center);
        sokoban_picture.set_can_shrink(false);
        let (x, y) = sokoban_pos;
        fixed.put(&sokoban_picture, x, y);

        let position = Cell::new(sokoban_pos);
        let puzzle_rc = Rc::new(RefCell::new(Puzzle::from_level(&level)));
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
            position,
            #[upgrade_or]
            glib::Propagation::Proceed,
            move |_controller, keyval, _keycode, state| {
                match keyval {
                    gdk::Key::q => {
                        app.quit();
                        glib::Propagation::Stop
                    }
                    gdk::Key::Right => {
                        let mut puzzle = puzzle_rc.borrow_mut();
                        if puzzle.can_move(Direction::Right) {
                            let origin = puzzle.sokoban_position();
                            let destination = Direction::Right.from(origin);
                            puzzle.r#move(Direction::Right);
                        }
                        let position = puzzle.sokoban_position();
                        dbg!(position);
                        fixed.move_(&sokoban_picture, position.1 as f64 * MOVE_LENGTH, position.0 as f64 * MOVE_LENGTH);
                        glib::Propagation::Proceed
                    }
                    gdk::Key::Left => {
                        let mut puzzle = puzzle_rc.borrow_mut();
                        if puzzle.can_move(Direction::Left) {
                            let origin = puzzle.sokoban_position();
                            let destination = Direction::Left.from(origin);
                            puzzle.r#move(Direction::Left);
                        }
                        let position = puzzle.sokoban_position();
                        dbg!(position);
                        fixed.move_(&sokoban_picture, position.1 as f64 * MOVE_LENGTH, position.0 as f64 * MOVE_LENGTH);
                        glib::Propagation::Proceed
                    }
                    gdk::Key::Up => {
                        let mut puzzle = puzzle_rc.borrow_mut();
                        if puzzle.can_move(Direction::Up) {
                            let origin = puzzle.sokoban_position();
                            let destination = Direction::Up.from(origin);
                            puzzle.r#move(Direction::Up);
                        }
                        let position = puzzle.sokoban_position();
                        dbg!(position);
                        fixed.move_(&sokoban_picture, position.1 as f64 * MOVE_LENGTH, position.0 as f64 * MOVE_LENGTH);
                        glib::Propagation::Proceed
                    }
                    gdk::Key::Down => {
                        let mut puzzle = puzzle_rc.borrow_mut();
                        if puzzle.can_move(Direction::Down) {
                            let origin = puzzle.sokoban_position();
                            let destination = Direction::Down.from(origin);
                            puzzle.r#move(Direction::Down);
                        }
                        let position = puzzle.sokoban_position();
                        dbg!(position);
                        fixed.move_(&sokoban_picture, position.1 as f64 * MOVE_LENGTH, position.0 as f64 * MOVE_LENGTH);
                        glib::Propagation::Proceed
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
