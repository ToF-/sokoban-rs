use crate::game::Game;
use crate::direction::Direction;
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
        let label = gtk::Label::new(Some("0"));
        let game_rc = Rc::new(RefCell::new(Game::new(&levels, 0)));
        let game = game_rc.borrow();
        game.fixed.put(&label, 0.0, 0.0);
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
            #[weak]
            label,
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
                    gdk::Key::asterisk => {
                        let game = game_rc.borrow_mut();
                        let mut puzzle = game.puzzle_rc.borrow_mut();
                        if !puzzle.is_solved() {
                            {
                                let mut steps = game.steps_rc.borrow_mut();
                                while !steps.is_empty() {
                                    if let Some(step) = steps.pop() {
                                        let reversed = step.reverse();
                                        let backwards =
                                            Direction::between(reversed.from(), reversed.to());
                                        puzzle.r#move(backwards);
                                        if let Some((crate_index, boxed_crate_step)) =
                                            reversed.pushing()
                                        {
                                            let crate_step = *boxed_crate_step;
                                            let crate_origin = crate_step.from();
                                            let crate_destination = crate_step.to();
                                            puzzle.move_piece_at(crate_origin, backwards);
                                            let mut crate_sprites = game.crate_sprites_rc.borrow_mut();
                                            let mut sprite = &mut crate_sprites[crate_index];
                                            sprite.r#move(&game.fixed, crate_destination);
                                        };
                                        let mut sokoban_sprite = game.sokoban_sprite_rc.borrow_mut();
                                        let position = puzzle.sokoban_position();
                                        sokoban_sprite.r#move(&game.fixed, position);
                                    }
                                }
                            }
                            let max_step = {
                                let steps = game.steps_rc.borrow();
                                steps.len()
                            };
                            label.set_text(&format!("{}", max_step));
                        };

                        glib::Propagation::Proceed
                    }
                    gdk::Key::u | gdk::Key::BackSpace => {
                        let game = game_rc.borrow();
                        let mut puzzle = game.puzzle_rc.borrow_mut();
                        let fixed = &game.fixed;
                        if !puzzle.is_solved() {
                            {
                                let mut steps = game.steps_rc.borrow_mut();
                                if let Some(step) = steps.pop() {
                                    let reversed = step.reverse();
                                    let backwards =
                                        Direction::between(reversed.from(), reversed.to());
                                    puzzle.r#move(backwards);
                                    if let Some((crate_index, boxed_crate_step)) =
                                        reversed.pushing()
                                    {
                                        let crate_step = *boxed_crate_step;
                                        let crate_origin = crate_step.from();
                                        let crate_destination = crate_step.to();
                                        puzzle.move_piece_at(crate_origin, backwards);
                                        let mut crate_sprites = game.crate_sprites_rc.borrow_mut();
                                        let mut sprite = &mut crate_sprites[crate_index];
                                        sprite.r#move(&fixed, crate_destination);
                                    };
                                    let mut sokoban_sprite = game.sokoban_sprite_rc.borrow_mut();
                                    let position = puzzle.sokoban_position();
                                    sokoban_sprite.r#move(&fixed, position);
                                }
                            }
                            let max_step = {
                                let steps = game.steps_rc.borrow();
                                steps.len()
                            };
                            label.set_text(&format!("{}", max_step));
                        };
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
                        let mut puzzle = game.puzzle_rc.borrow_mut();
                        let fixed = &game.fixed;
                        if puzzle.is_solved() {
                            app.quit();
                            glib::Propagation::Stop
                        } else {
                            if puzzle.can_move(direction) {
                                let origin = puzzle.sokoban_position();
                                let destination = direction.from(origin);
                                let target_opt = puzzle.r#move(direction);
                                let mut crate_index = 0;
                                if let Some(target) = target_opt {
                                    let mut crate_sprites = game.crate_sprites_rc.borrow_mut();
                                    crate_index = crate_sprites
                                        .iter()
                                        .position(|mut sprite| {
                                            sprite.coords() == destination
                                        })
                                        .expect("can't find crate");
                                    let mut sprite = &mut crate_sprites[crate_index];
                                    sprite.r#move(&fixed, target);
                                };
                                let position = puzzle.sokoban_position();
                                let mut sokoban_sprite = game.sokoban_sprite_rc.borrow_mut();
                                sokoban_sprite.r#move(&fixed, position);
                                let mut steps = game.steps_rc.borrow_mut();
                                let pushing = if let Some(target) = target_opt {
                                    Some(crate_index)
                                } else {
                                    None
                                };
                                steps.push(Step::new(origin, direction, pushing));
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
                            let max_step = {
                                let steps = game.steps_rc.borrow();
                                steps.len()
                            };
                            label.set_text(&format!("{}", max_step));
                            glib::Propagation::Proceed
                        }
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
