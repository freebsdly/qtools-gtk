use adw::glib::{Object, clone};
use adw::subclass::prelude::ObjectSubclassIsExt;
use adw::{NavigationPage, glib};
use gtk::prelude::ButtonExt;

mod imp {
    use crate::main_window::menu::AppMenu;
    use adw::prelude::NavigationPageExt;
    use adw::subclass::prelude::{
        NavigationPageImpl, ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt,
    };
    use adw::{HeaderBar, NavigationPage, ToolbarView, glib};
    use gtk::prelude::{BoxExt, WidgetExt};
    use gtk::subclass::prelude::WidgetImpl;
    use gtk::{Label, Orientation};
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct MainContent {
        pub sidebar_toggle_button: RefCell<Option<gtk::Button>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainContent {
        const NAME: &'static str = "QtoolsMainContent";
        type Type = super::MainContent;
        type ParentType = NavigationPage;
    }

    impl ObjectImpl for MainContent {
        fn constructed(&self) {
            self.parent_constructed();
            // 创建菜单
            let app_menu = AppMenu::new();

            // 创建内容区域标题栏
            let content_header = HeaderBar::builder()
                .title_widget(&Label::new(Some("")))
                .show_start_title_buttons(true)
                .build();

            // 创建sidebar展开收起按钮
            let sidebar_toggle_button = gtk::Button::builder()
                .icon_name("sidebar-show-symbolic")
                .build();

            // 保存按钮引用
            self.sidebar_toggle_button
                .replace(Some(sidebar_toggle_button.clone()));

            // 将菜单按钮添加到标题栏的开始位置
            content_header.pack_start(&sidebar_toggle_button);
            content_header.pack_end(&app_menu);

            content_header.add_css_class("header-bar");

            // 创建主要内容区域
            let main_content = gtk::Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(10)
                .build();
            main_content.append(&Label::new(Some("主要内容区域")));

            // 为主要内容区域添加CSS类
            main_content.add_css_class("main-content");

            // 创建工具栏视图
            let toolbar_view = ToolbarView::builder().content(&main_content).build();

            toolbar_view.add_top_bar(&content_header);

            self.obj().set_child(Some(&toolbar_view));
        }
    }

    impl WidgetImpl for MainContent {}

    impl NavigationPageImpl for MainContent {}
}

glib::wrapper! {
    pub struct MainContent(ObjectSubclass<imp::MainContent>)
        @extends NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl MainContent {
    pub fn new() -> Self {
        Object::builder().build()
    }

    // 设置侧边栏切换按钮的点击事件处理器
    pub fn setup_sidebar_toggle(&self, overlay_split_view: adw::OverlaySplitView) {
        if let Some(button) = &*self.imp().sidebar_toggle_button.borrow() {
            let button_clone = button.clone();
            button.connect_clicked(move |_| {
                let collapsed = overlay_split_view.is_collapsed();
                overlay_split_view.set_collapsed(!collapsed);
                
                // 根据侧边栏状态更新按钮图标
                if collapsed {
                    button_clone.set_icon_name("sidebar-show-symbolic");
                } else {
                    button_clone.set_icon_name("sidebar-show-symbolic");
                }
            });
        }
    }
}
