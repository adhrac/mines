use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};

mod mines_application;

use mines_application::MinesWindow;
const APP_ID: &str = "org.gtk_rs.HelloWorld1";


fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let grid = gtk::Grid::new();

    let label = gtk::Label::new(None);
    grid.attach(&label, 0, 3, 3, 1);

    for row in 0..3 {
        for col in 0..3 {
            let button = gtk::Button::builder()
                .label((row * 3 + col + 1).to_string())
                .margin_bottom(3)
                .margin_top(3)
                .margin_start(3)
                .margin_end(3)
                .build();

            button.connect_clicked(glib::clone!(
                #[weak] label,
                #[weak] grid,
                move |_| {
                    let mut s = label.text().to_string();
                    s = s + &(row * 3 + col + 1).to_string();
                    label.set_label(&s);
                    grid.child_at(0, 0).unwrap().set_property("label", "foo");
            }));

            grid.attach(&button, col, row, 1, 1);
        }
    }

    // Create a button with label and margins
    // Create a window and set the title
    let window: MinesWindow = glib::Object::builder()
        .property("application", app)
        .property("title", "My GTK App")
        .property("child", &grid)
        .build();

    let button = gtk::Button::builder()
        .margin_bottom(3)
        .margin_top(3)
        .margin_start(3)
        .margin_end(3)
        .label("Inc")
        .build();

    grid.attach(&button, 0, 4, 3, 1);
    button.connect_clicked(glib::clone!(
        #[weak] window,
        move |button| {
            window.inc();
            button.set_label(&format!("{:?}", window.imp().data));
        }
    ));

    // Present window
    window.present();
}