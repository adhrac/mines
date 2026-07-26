use gtk::glib;
use gtk::gio;
use glib::Object;
use gtk::glib::subclass::types::ObjectSubclassIsExt;

mod imp;

glib::wrapper! {
    pub struct MinesWindow(ObjectSubclass<imp::MinesWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MinesWindow {
    fn new() -> Self {
        Object::builder().build()
    }

    pub fn inc(&self) {
        self.imp().data.update(|x| x + 1);
    }
}