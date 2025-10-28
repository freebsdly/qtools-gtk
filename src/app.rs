use adw::glib::Object;
use adw::{gio, glib};

mod imp {
    use adw::glib;
    use adw::subclass::prelude::{
        AdwApplicationImpl, ApplicationImpl, ObjectImpl, ObjectImplExt, ObjectSubclass,
    };
    use gtk::subclass::prelude::GtkApplicationImpl;

    #[derive(Default)]
    pub struct QtoolsApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for QtoolsApplication {
        const NAME: &'static str = "QtoolsApplication";
        type Type = super::QtoolsApplication;
        type ParentType = adw::Application;

        fn new() -> Self {
            Self {}
        }
    }

    impl ObjectImpl for QtoolsApplication {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl ApplicationImpl for QtoolsApplication {}
    impl GtkApplicationImpl for QtoolsApplication {}
    impl AdwApplicationImpl for QtoolsApplication {}
}

glib::wrapper! {
    pub struct QtoolsApplication(ObjectSubclass<imp::QtoolsApplication>)
        @extends gio::Application, adw::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Buildable;
}

impl QtoolsApplication {
    pub fn new(app_id: &str) -> Self {
        Object::builder().property("application-id", app_id).build()
    }
}
