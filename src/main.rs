use crate::gdk::Key;
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
    assert!(
        gio::resources_lookup_data(
            "/com/example/MyApp/images/logo.png",
            gio::ResourceLookupFlags::NONE
        )
        .is_ok(),
        "Resource path not found!"
    );

    app.connect_activate(|app| {
        let bytes = gio::resources_lookup_data(
            "/com/example/MyApp/files/level_01.txt",
            gio::ResourceLookupFlags::NONE,
        )
        .expect("Failed to find resource");
        println!("{:?}", bytes);

        let logo = Picture::for_resource("/com/example/MyApp/images/logo.png");
        let block_1 = Picture::for_resource("/com/example/MyApp/images/block.png");
        let block_2 = Picture::for_resource("/com/example/MyApp/images/block.png");
        let block_3 = Picture::for_resource("/com/example/MyApp/images/block.png");
        logo.set_halign(gtk::Align::Center);
        logo.set_valign(gtk::Align::Center);
        logo.set_can_shrink(false);
        block_1.set_halign(gtk::Align::Center);
        block_1.set_valign(gtk::Align::Center);
        block_1.set_can_shrink(false);
        block_2.set_halign(gtk::Align::Center);
        block_2.set_valign(gtk::Align::Center);
        block_2.set_can_shrink(false);
        block_3.set_halign(gtk::Align::Center);
        block_3.set_valign(gtk::Align::Center);
        block_3.set_can_shrink(false);

        let position = Cell::new((100.0, 100.0));
        let fixed = gtk::Fixed::new();
        let coords = position.get();
        fixed.put(&block_1, 0.0, 0.0);
        fixed.put(&block_2, 32.0, 0.0);
        fixed.put(&block_3, 0.0, 32.0);
        fixed.put(&logo, coords.0, coords.1);
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .default_width(600)
            .default_height(400)
            .child(&fixed)
            .build();

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
        window.present();
    });

    app.run()
}
