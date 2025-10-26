use adw::glib::Object;
use adw::subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt};
use adw::{NavigationPage, glib};
use std::cell::RefCell;
use std::rc::Rc;

// 定义菜单项结构
#[derive(Clone)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub icon: String,
}

impl MenuItem {
    pub fn new(id: &str, label: &str, icon: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            icon: icon.to_string(),
        }
    }
}

// 定义菜单项点击回调类型
pub type MenuItemCallback = Rc<RefCell<dyn Fn(&MenuItem) + 'static>>;

mod imp {
    use super::*;
    use adw::prelude::NavigationPageExt;
    use adw::subclass::prelude::{
        NavigationPageImpl, ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt,
    };
    use adw::{HeaderBar, ToolbarView, glib};
    use gtk::prelude::WidgetExt;
    use gtk::subclass::prelude::WidgetImpl;
    use gtk::{Label, Orientation, PolicyType, ScrolledWindow};

    #[derive(Default)]
    pub struct MainSidebar {
        menu_items: Vec<MenuItem>,
        callback: Option<MenuItemCallback>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainSidebar {
        const NAME: &'static str = "QtoolsMainSidebar";
        type Type = super::MainSidebar;
        type ParentType = NavigationPage;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for MainSidebar {
        fn constructed(&self) {
            self.parent_constructed();

            // 创建侧边栏标题栏
            let sidebar_header = HeaderBar::builder()
                .title_widget(&Label::new(Some("")))
                .show_end_title_buttons(true)
                .build();

            sidebar_header.add_css_class("header-bar");

            // 创建侧边栏内容
            let sidebar_content = gtk::Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(10)
                .build();

            // 为侧边栏内容添加CSS类
            sidebar_content.add_css_class("main-sidebar");

            // 将侧边栏内容放入滚动视图中
            let sidebar_scroll = ScrolledWindow::builder()
                .vexpand(true)
                .hscrollbar_policy(PolicyType::Never)
                .child(&sidebar_content)
                .build();

            // 创建工具栏视图
            let toolbar_view = ToolbarView::builder().content(&sidebar_scroll).build();

            toolbar_view.add_top_bar(&sidebar_header);

            self.obj().set_child(Some(&toolbar_view));
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for MainSidebar {}

    impl NavigationPageImpl for MainSidebar {}
}

glib::wrapper! {
    pub struct MainSidebar(ObjectSubclass<imp::MainSidebar>)
        @extends NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl MainSidebar {
    pub fn new() -> Self {
        Object::builder().build()
    }

    // 设置菜单项点击回调
    pub fn set_callback<F: Fn(&MenuItem) + 'static>(&self, callback: F) {
        let imp = self.imp();
        imp.obj();
    }
}
