// src/main_window/mod.rs
pub mod about;
mod content;
pub mod menu;
mod sidebar;

use crate::app::QtoolsApplication;
use adw::glib;
use adw::glib::Object;
use gtk::gio;

mod imp {
    use crate::main_window::{content, sidebar};
    use adw::glib;
    use adw::prelude::AdwApplicationWindowExt;
    use adw::subclass::prelude::{
        AdwApplicationWindowImpl, ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt,
    };
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
}
