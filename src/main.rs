use crate::model::game::Game;
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

    let game = Game::new();
    let game_rc = Rc::new(RefCell::new(game));

    drawing_area.set_draw_func(clone!(
        #[strong]
        game_rc,
        move |_area, cr, _width, _height| {
            cr.set_source_rgb(0.1, 0.1, 0.15);
            cr.paint().expect("invalid cairo surface state");

            let game = game_rc.borrow_mut();
            let puzzle = game.puzzle();
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
    let key_controller = gtk::EventControllerKey::new();
    key_controller.connect_key_pressed(glib::clone!(
        #[weak]
        app,
        #[strong]
        game_rc,
        #[strong]
        drawing_area,
        #[upgrade_or]
        glib::Propagation::Proceed,
        move |_controller, keyval, _keycode, state| {
            match keyval {
                    gdk::Key::q => {
                        app.quit();
                        glib::Propagation::Stop
                    }
                    gdk::Key::A => {
                        let mut game = game_rc.borrow_mut();
                        game.mark('a');
                        glib::Propagation::Proceed
                    }
                    gdk::Key::B => {
                        let mut game = game_rc.borrow_mut();
                        game.mark('b');
                        glib::Propagation::Proceed
                    }
                    gdk::Key::C => {
                        let mut game = game_rc.borrow_mut();
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
                        let mut game = game_rc.borrow_mut();
                        game.restart();
                        glib::Propagation::Proceed
                    }
                    gdk::Key::u | gdk::Key::BackSpace => {
                        {
                        let mut game = game_rc.borrow_mut();
                        game.undo();
                        }
                        drawing_area.queue_draw();
                        glib::Propagation::Proceed
                    }
                    gdk::Key::n | gdk::Key::Tab => {
                        {
                        let mut game = game_rc.borrow_mut();
                        game.next_level();
                        };
                        drawing_area.queue_draw();
                        glib::Propagation::Proceed
                    }
                    gdk::Key::p => {
                        {
                        let mut game = game_rc.borrow_mut();
                        game.previous_level();
                        }
                        drawing_area.queue_draw();
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
                        {
                        let mut game = game_rc.borrow_mut();
                        game.move_towards(direction);
                        }
                        drawing_area.queue_draw();
                        glib::Propagation::Proceed
                    }
                    _ => glib::Propagation::Proceed,
                }
            }));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Sokoban")
        .child(&drawing_area)
        .build();

    window.add_controller(key_controller);
    window.present();
}
