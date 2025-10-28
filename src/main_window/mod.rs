// src/main_window/mod.rs
mod content;
pub mod menu;
mod sidebar;

use crate::app::QtoolsApplication;
use adw::glib::Object;
use adw::prelude::{ActionMapExt, AdwDialogExt, ApplicationExt};
use adw::{AboutDialog, glib};
use gtk::gio;
use gtk::prelude::GtkWindowExt;

mod imp {
    use crate::main_window::{content, sidebar};
    use adw::prelude::{ActionMapExt, AdwApplicationWindowExt};
    use adw::subclass::prelude::{
        AdwApplicationWindowImpl, ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt,
    };
    use adw::{gio, glib};
    use gtk::prelude::{GtkWindowExt, WidgetExt};
    use gtk::subclass::prelude::{ApplicationWindowImpl, WidgetImpl, WindowImpl};

    #[derive(Default)]
    pub struct MainWindow {}

    #[glib::object_subclass]
    impl ObjectSubclass for MainWindow {
        const NAME: &'static str = "QtoolsMainWindow";
        type Type = super::MainWindow;
        type ParentType = adw::ApplicationWindow;
    }

    impl ObjectImpl for MainWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let sidebar = sidebar::MainSidebar::new();

            // 创建主要内容区域
            let main_content = content::MainContent::new();

            // 创建分割视图（带侧边栏）- 使用 AdwOverlaySplitView 实现可折叠侧边栏
            let overlay_view = adw::OverlaySplitView::builder()
                .sidebar(&sidebar)
                .content(&main_content)
                .collapsed(false) // 默认展开
                .min_sidebar_width(50.0) // 最小宽度设为50，足够显示图标
                .max_sidebar_width(300.0)
                .build();

            // 设置分割视图垂直扩展以填满可用空间
            overlay_view.set_vexpand(true);

            let obj = self.obj();
            obj.set_title(Some("Qtools"));
            obj.set_default_size(1024, 768);
            obj.set_content(Some(&overlay_view));
            obj.setup_actions();
        }
    }

    impl WidgetImpl for MainWindow {}
    impl WindowImpl for MainWindow {}
    impl AdwApplicationWindowImpl for MainWindow {}
    impl ApplicationWindowImpl for MainWindow {}
}

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;

}

impl MainWindow {
    pub fn new(app: &QtoolsApplication) -> Self {
        Object::builder().property("application", app).build()
    }

    // 动作处理函数
    fn action_new(&self) {
        println!("新建文件");
    }

    fn action_open(&self) {
        println!("打开文件");
    }

    fn action_save(&self) {
        println!("保存文件");
    }

    fn action_save_as(&self) {
        println!("另存为");
    }

    fn action_preferences(&self) {
        println!("打开首选项");
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
        about_dialog.present(Some(self));
    }

    fn action_quit(&self) {
        println!("退出应用");
        self.application().unwrap().quit();
    }

    pub fn setup_actions(&self) {
        // 定义动作配置
        let actions = vec![
            ("new", Self::action_new as fn(&_)),
            ("open", Self::action_open),
            ("save", Self::action_save),
            ("save-as", Self::action_save_as),
            ("preferences", Self::action_preferences),
            ("about", Self::action_about),
            ("quit", Self::action_quit),
        ];

        for (name, callback) in actions {
            let action = gio::SimpleAction::new(name, None);
            let window = self.clone();
            action.connect_activate(move |_, _| {
                callback(&window);
            });
            self.add_action(&action);
        }
    }
}
