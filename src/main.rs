use crate::sprite::Sprite;
use crate::direction::Direction;
use crate::gdk::Key;
use crate::level::Level;
use crate::puzzle::Puzzle;
use crate::state::State;
use crate::tile::TILES;
use crate::tile::Tiles;
use crate::tile::fixed_for_level;
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

pub mod coords;
pub mod direction;
pub mod level;
pub mod puzzle;
pub mod sprite;
pub mod state;
pub mod tile;

const UNIT: f64 = 32.0;

fn main() -> glib::ExitCode {
    gio::resources_register_include!("myapp.gresource").expect("Failed to register resources.");

    let app = Application::builder()
        .application_id("com.example.MyApp")
        .build();

    app.connect_activate(|app| {
        let level_index = 0;
        let level_file_name = format!("/com/example/MyApp/files/level_{:02}.txt", level_index);
        let bytes = gio::resources_lookup_data(&level_file_name, gio::ResourceLookupFlags::NONE)
            .expect("Failed to find resource");
        let level = Level::from_bytes(&bytes);
        let puzzle = Puzzle::from_level(&level);
        let mut crates: Vec<RefCell<Sprite>> = Vec::new();
        for crate_index in 0..puzzle.max_crate() {
            let crate_coords = puzzle.crate_coords(crate_index);
            crates.push(RefCell::new(Sprite::make_sprite(&State::Crate, crate_coords)));
        }
        let mut sokoban_position = puzzle.sokoban_position();
        let fixed = fixed_for_level(&level);
        let mut sokoban_sprite = Sprite::make_sprite(&State::Sokoban, sokoban_position);
        sokoban_sprite.place(&fixed);
        for crate_index in 0..crates.len() {
            let mut sprite = crates[crate_index].borrow_mut();
            sprite.place(&fixed);
        }

        let puzzle_rc = Rc::new(RefCell::new(puzzle));
        let crates_rc = Rc::new(RefCell::new(crates));
        let sokoban_rc = Rc::new(RefCell::new(sokoban_sprite));

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
            #[strong]
            sokoban_rc,
            #[strong]
            puzzle_rc,
            #[strong]
            crates_rc,
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
                                    let index = crates.iter().position(|sprite_rc|
                                       sprite_rc.borrow().coords() == destination 
                                    ).expect("can't find crate");
                                    let mut sprite = crates[index].borrow_mut();
                                    sprite.r#move(&fixed, target);
                                    };
                                let position = puzzle.sokoban_position();
                                let mut sokoban_sprite = sokoban_rc.borrow_mut();
                                sokoban_sprite.r#move(&fixed, position);
                            };
                            if puzzle.is_solved() {
                                for crate_index in 0..puzzle.max_crate() {
                                    let coords = puzzle.crate_coords(crate_index);
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
