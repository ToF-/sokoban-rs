use crate::game::Game;
use crate::gdk::Key;
use crate::graphics::char_display::CHAR_TILES;
use crate::graphics::char_display::CharTiles;
use crate::graphics::char_display::char_tile_pixbuf;
use crate::model::coords::MAX_COL;
use crate::model::coords::MAX_ROW;
use crate::model::direction::Direction;
use crate::model::level::Level;
use crate::model::level::levels_from_bytes;
use crate::model::puzzle::Puzzle;
use crate::sprite::Sprite;
use crate::step::Step;
use crate::tile::TILES;
use crate::tile::Tiles;
use crate::tile::fixed_for_level;
use crate::tile::paintable;
use crate::tile::picture;
use crate::tile::solid_picture;
use glib::clone;
use gtk::gdk;
use gtk::gdk::prelude::GdkCairoContextExt;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::prelude::DrawingAreaExtManual;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea, Picture, gio, glib};
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub mod game;
pub mod graphics;
pub mod grid;
pub mod model;
pub mod sprite;
pub mod step;
pub mod tile;

const UNIT: f64 = 32.0;

fn main() -> glib::ExitCode {
    gio::resources_register_include!("myapp.gresource").expect("Failed to register resources.");

    let app = Application::builder()
        .application_id("com.example.MyApp")
        .build();

    app.connect_activate(build_ascii_char_ui);
    app.run()
}
pub fn build_ascii_char_ui(app: &Application) {
    let drawing_area = DrawingArea::builder()
        .content_width(MAX_COL as i32 * UNIT as i32)
        .content_height(MAX_ROW as i32 * UNIT as i32)
        .build();

    let levels_file_name = format!("/com/example/MyApp/files/levels.txt");
    let bytes = gio::resources_lookup_data(&levels_file_name, gio::ResourceLookupFlags::NONE)
        .expect("Failed to find resource");
    let levels = levels_from_bytes(&bytes);
    let level_index = 1;
    let puzzle = Puzzle::from_level(&levels[level_index]);
    let puzzle_rc = Rc::new(RefCell::new(puzzle));
    drawing_area.set_draw_func(clone!(
        #[strong]
        puzzle_rc,
        move |_area, cr, _width, _height| {
            cr.set_source_rgb(0.1, 0.1, 0.15);
            cr.paint().expect("invalid cairo surface state");

            let levels_file_name = format!("/com/example/MyApp/files/levels.txt");
            let bytes =
                gio::resources_lookup_data(&levels_file_name, gio::ResourceLookupFlags::NONE)
                    .expect("Failed to find resource");
            let levels = levels_from_bytes(&bytes);
            let level_index = 1;
            let puzzle = Puzzle::from_level(&levels[level_index]);
            println!("{}", puzzle.pretty_print());
            let puzzle_rc = Rc::new(RefCell::new(puzzle.clone()));
            for row in 0..puzzle.height() {
                for col in 0..puzzle.width() {
                    let element = puzzle.element_at((row, col));
                    let pixbuf = CHAR_TILES.with(|cell| {
                        let char_tiles = cell.get_or_init(CharTiles::from_picture_files);
                        char_tile_pixbuf(&element).clone()
                    });
                    let (x, y) = (col as f64 * UNIT as f64, row as f64 * UNIT as f64);
                    cr.set_source_pixbuf(&pixbuf.clone(), x, y);
                    cr.paint().expect("failed to paint sprite");
                }
            }
        }
    ));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Sokoban")
        .child(&drawing_area)
        .build();

    window.present();
}

/*
    app.connect_activate(|app| {
        let levels_file_name = format!("/com/example/MyApp/files/levels.txt");
        let bytes = gio::resources_lookup_data(&levels_file_name, gio::ResourceLookupFlags::NONE)
            .expect("Failed to find resource");
        let levels = levels_from_bytes(&bytes);
        let level_index = 1;
        let puzzle = Puzzle::from_level(&levels[level_index]);
        let puzzle_rc = Rc::new(RefCell::new(puzzle));
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .default_width(MAX_COL as i32 * UNIT as i32)
            .default_height(MAX_ROW as i32 * UNIT as i32)
            .build();


        let key_controller = gtk::EventControllerKey::new();
        key_controller.connect_key_pressed(glib::clone!(
            #[weak]
            app,
            #[strong]
            game_rc,
            #[strong]
            level_ready_rc,
            #[upgrade_or]
            glib::Propagation::Proceed,
            move |_controller, keyval, _keycode, state| {
                let level_ready = level_ready_rc.get();
                if !level_ready {
                    glib::Propagation::Stop
                } else {
                    match keyval {
                        gdk::Key::q => {
                            app.quit();
                            glib::Propagation::Stop
                        }
                        gdk::Key::A => {
                            let game = game_rc.borrow_mut();
                            game.mark('a');
                            glib::Propagation::Proceed
                        }
                        gdk::Key::B => {
                            let game = game_rc.borrow_mut();
                            game.mark('b');
                            glib::Propagation::Proceed
                        }
                        gdk::Key::C => {
                            let game = game_rc.borrow_mut();
                            game.mark('c');
                            glib::Propagation::Proceed
                        }
                        gdk::Key::a => {
                            let mut game = game_rc.borrow_mut();
                            game.goto('a');
                            glib::Propagation::Proceed
                        }
                        gdk::Key::b => {
                            let mut game = game_rc.borrow_mut();
                            game.goto('b');
                            glib::Propagation::Proceed
                        }
                        gdk::Key::c => {
                            let mut game = game_rc.borrow_mut();
                            game.goto('c');
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
                            *game = Game::next_level(&game, level_ready_rc.clone());
                            let window = app
                                .active_window()
                                .expect("can't access application window");
                            window.set_child(Some(&game.fixed));
                            window.set_title(Some(&format!(
                                "Sokoban - Level {:02}",
                                game.level_index()
                            )));
                            window.set_default_size(game.fixed.width(), game.fixed.height());
                            glib::Propagation::Proceed
                        }
                        gdk::Key::p => {
                            let mut game = game_rc.borrow_mut();
                            *game = Game::previous_level(&game, level_ready_rc.clone());
                            let window = app
                                .active_window()
                                .expect("can't access application window");
                            window.set_child(Some(&game.fixed));
                            window.set_title(Some(&format!(
                                "Sokoban - Level {:02}",
                                game.level_index()
                            )));
                            glib::Propagation::Proceed
                        }
                        gdk::Key::x => {
                            let directions =
                                vec![Direction::Right, Direction::Right, Direction::Right];
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
            }
        ));
        window.add_controller(key_controller);
        window.present();
    });

    app.run()
}
*/
