use crate::direction::Direction;
use crate::game::Game;
use crate::gdk::Key;
use crate::level::Level;
use crate::level::levels_from_bytes;
use crate::puzzle::Puzzle;
use crate::sprite::Sprite;
use crate::state::State;
use crate::step::Step;
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
pub mod game;
pub mod level;
pub mod path_finder;
pub mod puzzle;
pub mod sprite;
pub mod state;
pub mod step;
pub mod tile;

const UNIT: f64 = 32.0;

fn main() -> glib::ExitCode {
    gio::resources_register_include!("myapp.gresource").expect("Failed to register resources.");

    let app = Application::builder()
        .application_id("com.example.MyApp")
        .build();

    app.connect_activate(|app| {
        let levels_file_name = format!("/com/example/MyApp/files/levels.txt");
        let bytes = gio::resources_lookup_data(&levels_file_name, gio::ResourceLookupFlags::NONE)
            .expect("Failed to find resource");
        let levels = levels_from_bytes(&bytes);
        let level_index = 8;
        let game_rc = Rc::new(RefCell::new(Game::new(&levels, 1)));
        let game = game_rc.borrow();
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .default_width(20 as i32 * UNIT as i32)
            .default_height(20 as i32 * UNIT as i32)
            .child(&game.fixed)
            .build();

        let key_controller = gtk::EventControllerKey::new();
        key_controller.connect_key_pressed(glib::clone!(
            #[weak]
            app,
            #[strong]
            game_rc,
            #[upgrade_or]
            glib::Propagation::Proceed,
            move |_controller, keyval, _keycode, state| {
                match keyval {
                    gdk::Key::q => {
                        app.quit();
                        glib::Propagation::Stop
                    }
                    gdk::Key::a => {
                        let game = game_rc.borrow_mut();
                        game.mark('a');
                        glib::Propagation::Proceed
                    }
                    gdk::Key::A => {
                        let game = game_rc.borrow_mut();
                        game.unmark('a');
                        glib::Propagation::Proceed
                    }
                    gdk::Key::b => {
                        let game = game_rc.borrow_mut();
                        game.mark('b');
                        glib::Propagation::Proceed
                    }
                    gdk::Key::B => {
                        let game = game_rc.borrow_mut();
                        game.unmark('b');
                        glib::Propagation::Proceed
                    }
                    gdk::Key::c => {
                        let game = game_rc.borrow_mut();
                        game.mark('c');
                        glib::Propagation::Proceed
                    }
                    gdk::Key::C => {
                        let game = game_rc.borrow_mut();
                        game.unmark('c');
                        glib::Propagation::Proceed
                    }
                    gdk::Key::asterisk => {
                        let game = game_rc.borrow_mut();
                        game.restart();
                        glib::Propagation::Proceed
                    }
                    gdk::Key::u | gdk::Key::BackSpace => {
                        let game = game_rc.borrow();
                        game.undo();
                        glib::Propagation::Proceed
                    }
                    gdk::Key::n | gdk::Key::Tab => {
                        let mut game = game_rc.borrow_mut();
                        *game = Game::next_level(&game);
                        let window = app
                            .active_window()
                            .expect("can't access application window");
                        window.set_child(Some(&game.fixed));
                        window
                            .set_title(Some(&format!("Sokoban - Level {:02}", game.level_index())));
                        glib::Propagation::Proceed
                    }
                    gdk::Key::p => {
                        let mut game = game_rc.borrow_mut();
                        *game = Game::previous_level(&game);
                        let window = app
                            .active_window()
                            .expect("can't access application window");
                        window.set_child(Some(&game.fixed));
                        window
                            .set_title(Some(&format!("Sokoban - Level {:02}", game.level_index())));
                        glib::Propagation::Proceed
                    }
                    gdk::Key::x => {
                        let directions = vec![Direction::Right, Direction::Right, Direction::Right];
                        let mut game = game_rc.borrow_mut();
                        game.follow_path(&directions);
                        glib::Propagation::Proceed
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
                        let game = game_rc.borrow();
                        game.move_towards(direction);
                        glib::Propagation::Proceed
                    }
                    gdk::Key::dollar => {
                        let game = game_rc.borrow_mut();
                        let puzzle = game.puzzle_rc.borrow();
                        println!("{}", puzzle.pretty_print());
                        let crate_sprites = game.crate_sprites_rc.borrow();
                        for sprite in crate_sprites.iter() {
                            println!("{:?}", sprite);
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
