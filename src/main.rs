use gtk::{gio, glib, Application, Picture};
use gtk::gdk;
use gtk::prelude::*;

fn main() -> glib::ExitCode {
    gio::resources_register_include!("myapp.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder()
        .application_id("com.example.MyApp")
        .build();
assert!(
    gio::resources_lookup_data("/com/example/MyApp/images/logo.png", gio::ResourceLookupFlags::NONE).is_ok(),
    "Resource path not found!"
);


    app.connect_activate(|app| {
        let picture = Picture::for_resource("/com/example/MyApp/images/logo.png");

        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .child(&picture)
            .build();

        let key_controller = gtk::EventControllerKey::new();
        key_controller.connect_key_pressed(glib::clone!(
            #[weak]
            app,
            #[upgrade_or]
            glib::Propagation::Proceed,
            move
            |_controller, keyval, _keycode, state| {
            match keyval {
                gdk::Key::q => {
                    println!("{:?}", keyval);
                    app.quit();
                    glib::Propagation::Stop
            }
            _ => glib::Propagation::Proceed
            }
        }));
        window.add_controller(key_controller);
        window.present();
    });

    app.run()
}
