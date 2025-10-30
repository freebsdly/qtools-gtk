use adw::glib::Object;
use adw::{glib, NavigationPage};

mod imp {
    use super::*;
    use adw::prelude::NavigationPageExt;
    use adw::subclass::prelude::{
        NavigationPageImpl, ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt,
    };
    use adw::{glib, HeaderBar, ToolbarView};
    use gtk::prelude::{BoxExt, WidgetExt};
    use gtk::subclass::prelude::WidgetImpl;
    use gtk::{Label, Orientation, PolicyType, ScrolledWindow};

    #[derive(Default)]
    pub struct MainToolbar {}

    #[glib::object_subclass]
    impl ObjectSubclass for MainToolbar {
        const NAME: &'static str = "QtoolsMainToolbar";
        type Type = super::MainToolbar;
        type ParentType = NavigationPage;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for MainToolbar {
        fn constructed(&self) {
            self.parent_constructed();

            // 创建侧边栏标题栏
            let sidebar_header = HeaderBar::builder()
                .title_widget(&Label::new(Some("")))
                .show_end_title_buttons(true)
                .build();

            sidebar_header.add_css_class("header-bar");

            // 创建工具按钮
            let new_button = gtk::Button::from_icon_name("document-new-symbolic");
            new_button.set_tooltip_text(Some("新建"));
            new_button.add_css_class("flat");

            let open_button = gtk::Button::from_icon_name("document-open-symbolic");
            open_button.set_tooltip_text(Some("打开"));
            open_button.add_css_class("flat");

            let save_button = gtk::Button::from_icon_name("document-save-symbolic");
            save_button.set_tooltip_text(Some("保存"));
            save_button.add_css_class("flat");

            // 创建工具栏按钮容器
            let toolbar_box = gtk::Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(6)
                .margin_start(6)
                .margin_end(6)
                .margin_top(6)
                .margin_bottom(6)
                .build();

            toolbar_box.append(&new_button);
            toolbar_box.append(&open_button);
            toolbar_box.append(&save_button);

            // 创建侧边栏内容
            let sidebar_content = gtk::Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(10)
                .build();

            // 添加工具栏到侧边栏内容顶部
            sidebar_content.append(&toolbar_box);

            // 为侧边栏内容添加CSS类
            sidebar_content.add_css_class("main-toolbar");

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
    impl WidgetImpl for MainToolbar {}

    impl NavigationPageImpl for MainToolbar {}
}

glib::wrapper! {
    pub struct MainToolbar(ObjectSubclass<imp::MainToolbar>)
        @extends NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl MainToolbar {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
