use std::cell::OnceCell;
use adw::{glib, ApplicationWindow};
use adw::gio::Settings;
use adw::subclass::prelude::*;
use gtk::{Button, CompositeTemplate};
use glib::subclass::InitializingObject;
use gtk::prelude::ButtonExt;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/top/qinhuajun/app/main_window.ui")]
pub struct QToolAppWindow {
    pub settings: OnceCell<Settings>,
    // #[template_child]
    // pub header_bar: TemplateChild<adw::HeaderBar>,
    // #[template_child]
    // pub greet_button: TemplateChild<Button>,
    // #[template_child]
    // pub clear_button: TemplateChild<Button>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for QToolAppWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "QToolAppWindow";
    type Type = super::Window;
    type ParentType = ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for QToolAppWindow {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        // Setup
        let obj = self.obj();
        obj.setup_settings();
    }
}
// ANCHOR_END: object_impl

// Trait shared by all widgets
impl WidgetImpl for QToolAppWindow {}

// ANCHOR: window_impl
// Trait shared by all windows
impl WindowImpl for QToolAppWindow {}
// ANCHOR_END: window_impl

// Trait shared by all application windows
impl ApplicationWindowImpl for QToolAppWindow {}

// Trait shared by all adwaita application windows
impl AdwApplicationWindowImpl for QToolAppWindow {}

// impl callbacks
#[gtk::template_callbacks]
impl QToolAppWindow {
    #[template_callback]
    fn on_greet_clicked(button: &Button) {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
        println!("greet clicked")
    }

    #[template_callback]
    fn on_clear_clicked(button: &Button) {
        // Set the label to "Hello World!" after the button has been clicked on
        println!("clear clicked")
    }
}
