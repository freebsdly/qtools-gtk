use adw::glib::Object;
use adw::prelude::{ActionMapExt, AdwDialogExt};
use adw::{AboutDialog, gio, glib};
use gtk::prelude::{ApplicationExt, GtkApplicationExt};

mod imp {
    use adw::glib;
    use adw::subclass::prelude::{
        AdwApplicationImpl, ApplicationImpl, ApplicationImplExt, ObjectImpl, ObjectImplExt,
        ObjectSubclass, ObjectSubclassExt,
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

    impl ApplicationImpl for QtoolsApplication {
        // 在这里设置应用程序级别的动作
        fn startup(&self) {
            self.parent_startup();
            let app = self.obj();
            app.setup_actions();
        }
    }

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

    fn action_about(&self) {
        let about_dialog = AboutDialog::builder()
            .application_name("QTools")
            .application_icon("applications-development")
            .developer_name("Qinhuajun")
            .version("0.1.0")
            .comments("一个基于 Rust 和 GTK 的实用工具集")
            .website("https://github.com/qinhuajun/qtools")
            .issue_url("https://github.com/qinhuajun/qtools/issues")
            .developers(vec!["Qinhuajun https://github.com/qinhuajun"])
            .copyright("© 2025 Qinhuajun")
            .license_type(gtk::License::MitX11)
            .build();

        // 获取当前活动窗口作为父窗口
        if let Some(window) = self.active_window() {
            about_dialog.present(Some(&window));
        } else {
            // 如果没有活动窗口，则不指定父窗口
            about_dialog.present(None::<&gtk::Window>);
        }
    }

    fn action_quit(&self) {
        println!("退出应用");
        self.quit();
    }

    pub fn setup_actions(&self) {
        // 定义应用程序级动作
        let actions = vec![
            ("quit", Self::action_quit as fn(&_)),
            ("about", Self::action_about),
        ];

        for (name, callback) in actions {
            let action = gio::SimpleAction::new(name, None);
            let app = self.clone();
            action.connect_activate(move |_, _| {
                callback(&app);
            });
            self.add_action(&action);
        }
    }
}
