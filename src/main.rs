use crate::gdk::Key;
use crate::level::Level;
use crate::state::State;
use gtk::gdk;
use gtk::prelude::*;
use gtk::{Application, Picture, gio, glib};
use std::cell::Cell;

pub mod level;
pub mod state;

const MOVE_LENGTH: f64 = 32.0;

fn main() -> glib::ExitCode {
    gio::resources_register_include!("myapp.gresource").expect("Failed to register resources.");

    let app = Application::builder()
        .application_id("com.example.MyApp")
        .build();

    app.connect_activate(|app| {
        let floor = Picture::for_resource("/com/example/MyApp/images/floor.png");
        let floor_paintable = floor.paintable().expect("can't access to paintable");
        let wall = Picture::for_resource("/com/example/MyApp/images/wall.png");
        let wall_paintable = wall.paintable().expect("can't access to paintable");
        let _crate = Picture::for_resource("/com/example/MyApp/images/crate.png");
        let crate_paintable = _crate.paintable().expect("can't access to paintable");
        let goal = Picture::for_resource("/com/example/MyApp/images/goal.png");
        let goal_paintable = goal.paintable().expect("can't access to paintable");
        let stored = Picture::for_resource("/com/example/MyApp/images/stored.png");
        let stored_paintable = stored.paintable().expect("can't access to paintable");
        let sokoban = Picture::for_resource("/com/example/MyApp/images/sokoban.png");
        let sokoban_paintable = sokoban.paintable().expect("can't access to paintable");

        let bytes = gio::resources_lookup_data(
            "/com/example/MyApp/files/level_01.txt",
            gio::ResourceLookupFlags::NONE,
        )
        .expect("Failed to find resource");
        let fixed = gtk::Fixed::new();
        let level = Level::from_bytes(&bytes);
        println!("{}", level.pretty_print());
        let mut sokoban_pos = (0.0, 0.0);
        for row in 0..level.height() {
            for col in 0..level.width() {
                let state = level.state_at(row, col);
                let paintable = match state {
                    State::Wall => wall_paintable.clone(),
                    State::Crate => crate_paintable.clone(),
                    State::Goal => goal_paintable.clone(),
                    State::Sokoban => floor_paintable.clone(),
                    State::Floor => floor_paintable.clone(),
                    _ => floor_paintable.clone(),
                };
                let picture = Picture::for_paintable(&paintable);
                picture.set_halign(gtk::Align::Center);
                picture.set_valign(gtk::Align::Center);
                picture.set_can_shrink(false);
                let x: f64 = col as f64 * 32.0;
                let y: f64 = row as f64 * 32.0;
                fixed.put(&picture, x, y);
                if state == State::Sokoban {
                    sokoban_pos = (col as f64 * 32.0, row as f64 * 32.0);
                }
            }
        }
        let paintable = sokoban_paintable.clone();
        let picture = Picture::for_paintable(&paintable);
        picture.set_halign(gtk::Align::Center);
        picture.set_valign(gtk::Align::Center);
        picture.set_can_shrink(false);
        let (x,y) = sokoban_pos;
        fixed.put(&picture, x, y);

        let position = Cell::new((100.0, 100.0));
        let coords = position.get();
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .default_width(level.width() as i32 * 32)
            .default_height(level.height() as i32 * 32)
            .child(&fixed)
            .build();

        /*
        let key_controller = gtk::EventControllerKey::new();
        key_controller.connect_key_pressed(glib::clone!(
            #[weak]
            app,
            #[weak]
            fixed,
            #[weak]
            logo,
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
                        let mut coords = position.get();
                        coords.0 += MOVE_LENGTH;
                        position.set(coords);
                        let coords = position.get();
                        fixed.move_(&logo, coords.0, coords.1);
                        glib::Propagation::Proceed
                    }
                    gdk::Key::Left => {
                        let mut coords = position.get();
                        coords.0 -= MOVE_LENGTH;
                        position.set(coords);
                        let coords = position.get();
                        fixed.move_(&logo, coords.0, coords.1);
                        glib::Propagation::Proceed
                    }
                    gdk::Key::Up => {
                        let mut coords = position.get();
                        coords.1 -= MOVE_LENGTH;
                        position.set(coords);
                        let coords = position.get();
                        fixed.move_(&logo, coords.0, coords.1);
                        glib::Propagation::Proceed
                    }
                    gdk::Key::Down => {
                        let mut coords = position.get();
                        coords.1 += MOVE_LENGTH;
                        position.set(coords);
                        let coords = position.get();
                        fixed.move_(&logo, coords.0, coords.1);
                        glib::Propagation::Proceed
                    }
                    _ => glib::Propagation::Proceed,
                }
            }
        ));
        window.add_controller(key_controller);
        */
        window.present();
    });

    app.run()
}
