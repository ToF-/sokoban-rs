use crate::tile::picture;
use crate::direction::Direction;
use crate::gdk::Key;
use crate::level::Level;
use crate::puzzle::Puzzle;
use crate::state::State;
use crate::tile::TILES;
use crate::tile::Tiles;
use crate::tile::paintable;
use gtk::gdk;
use gtk::prelude::*;
use gtk::{Application, Picture, gio, glib};
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

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
        let mut crate_pictures: Vec<Vec<Picture>> =
            vec![vec![Picture::new(); level.width()]; level.height()];

        println!("{}", level.pretty_print());
        for row in 0..level.height() {
            for col in 0..level.width() {
                let state = level.state_at(row, col);
                let mut background = State::Floor;
                if state == State::Sokoban {
                    let picture = picture(&State::Floor);
                    let x: f64 = col as f64 * 32.0;
                    let y: f64 = row as f64 * 32.0;
                    fixed.put(&picture, x, y);
                } else if state == State::Crate {
                    let bg_picture = picture(&State::Floor);
                    let x: f64 = col as f64 * 32.0;
                    let y: f64 = row as f64 * 32.0;
                    fixed.put(&bg_picture, x, y);
                    let fg_picture = picture(&State::Crate);
                    let x: f64 = col as f64 * 32.0;
                    let y: f64 = row as f64 * 32.0;
                    fixed.put(&fg_picture, x, y);
                } else {
                    let picture = picture(&state);
                    let x: f64 = col as f64 * 32.0;
                    let y: f64 = row as f64 * 32.0;
                    fixed.put(&picture, x, y);
                }
            }
        }
        let puzzle_rc = Rc::new(RefCell::new(Puzzle::from_level(&level)));
        let sokoban_picture = Picture::for_paintable(&sokoban_paintable);
        sokoban_picture.set_halign(gtk::Align::Center);
        sokoban_picture.set_valign(gtk::Align::Center);
        sokoban_picture.set_can_shrink(false);

        let (row, col) = {
            let puzzle = puzzle_rc.borrow();
            puzzle.sokoban_position()
        };
        let x: f64 = col as f64 * 32.0;
        let y: f64 = row as f64 * 32.0;
        fixed.put(&sokoban_picture, x, y);

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
            position,
            #[upgrade_or]
            glib::Propagation::Proceed,
            move |_controller, keyval, _keycode, state| {
                match keyval {
                    gdk::Key::q => {
                        app.quit();
                        glib::Propagation::Stop
                    }
                    gdk::Key::Right | gdk::Key::Left | gdk::Key::Down | gdk::Key::Up => {
                        let direction = match keyval {
                            gdk::Key::Right => Direction::Right,
                            gdk::Key::Left => Direction::Left,
                            gdk::Key::Up => Direction::Up,
                            gdk::Key::Down => Direction::Down,
                            _ => Direction::Right,
                        };
                        let mut puzzle = puzzle_rc.borrow_mut();
                        if puzzle.can_move(direction) {
                            let origin = puzzle.sokoban_position();
                            let destination = direction.from(origin);
                            let target_opt = puzzle.r#move(direction);
                            if let Some(target) = target_opt {
                                let destination_state = puzzle.state_at(destination);
                            }
                            let position = puzzle.sokoban_position();
                            fixed.move_(
                                &sokoban_picture,
                                position.1 as f64 * MOVE_LENGTH,
                                position.0 as f64 * MOVE_LENGTH,
                            );
                        }
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
