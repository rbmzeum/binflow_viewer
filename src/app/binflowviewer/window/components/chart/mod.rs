pub mod imp;

use gtk4::{
    glib,
    glib::closure_local,
    glib::ObjectExt,
    subclass::prelude::*,
    StringList,
    Widget,
    DrawingArea,
};

glib::wrapper! {
    pub struct BChartComponent(ObjectSubclass<imp::BChartComponent>)
        @extends Widget, DrawingArea;
}

impl BChartComponent {
}