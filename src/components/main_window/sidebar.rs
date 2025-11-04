use adw::glib::Object;
use adw::{glib, NavigationPage};

mod imp {
    use super::*;
    use adw::glib;
    use adw::prelude::NavigationPageExt;
    use adw::subclass::prelude::{
        NavigationPageImpl, ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt,
    };
    use gtk::prelude::WidgetExt;
    use gtk::subclass::prelude::WidgetImpl;
    use gtk::{Orientation, PolicyType, ScrolledWindow};

    #[derive(Default)]
    pub struct MainSidebar {}

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
            self.build_ui();
        }
    }

    impl MainSidebar {
        fn build_ui(&self) {
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

            self.obj().set_child(Some(&sidebar_scroll));
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
}
