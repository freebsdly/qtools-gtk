use crate::APP_ID;
use adw::gio::Settings;
use adw::subclass::prelude::ObjectSubclassIsExt;
use adw::{Application, gio, glib};

mod imp;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::QToolAppWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    fn setup_settings(&self) {
        // need APP_ID.gschema.xml file in the specified schema directory.
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }
}
