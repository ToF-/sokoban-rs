use gtk4::prelude::*;
use gtk4::{gio, glib, Application, Picture};

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

        let window = gtk4::ApplicationWindow::builder()
            .application(app)
            .child(&picture)
            .build();
        window.present();
    });

    app.run()
}

/*
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        window.present();
    });

    app.run()
}
*/
