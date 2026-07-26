use gtk::glib;
use gtk::subclass::prelude::*;
use std::cell::Cell;

#[derive(Default)]
pub struct MinesWindow {
    pub data: Cell<i32>,
}

#[glib::object_subclass]
impl ObjectSubclass for MinesWindow {
    const NAME: &'static str = "jasminesWindow";
    type Type = super::MinesWindow;
    type ParentType = gtk::ApplicationWindow;
}

impl WidgetImpl for MinesWindow {

}

impl WindowImpl for MinesWindow {

}

impl ApplicationWindowImpl for MinesWindow {

}

impl ObjectImpl for MinesWindow {

}