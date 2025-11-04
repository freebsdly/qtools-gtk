use adw::glib::Object;
use adw::subclass::prelude::ObjectSubclassIsExt;
use adw::{glib, NavigationPage};
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};

mod imp {
    use crate::components::ai_chat::AIChat;
    use crate::components::main_window::menu::AppMenu;
    use crate::components::main_window::sidebar;
    use crate::components::screen_shot::ScreenShot;
    use crate::components::welcome::Welcome;
    use adw::prelude::{BreakpointBinExt, NavigationPageExt, ToValue};
    use adw::subclass::prelude::*;
    use adw::{
        glib, Breakpoint, BreakpointCondition, BreakpointConditionLengthType, HeaderBar,
        LengthUnit, NavigationPage, ToolbarView,
    };
    use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
    use gtk::subclass::prelude::WidgetImpl;
    use gtk::{Label, Orientation};
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct MainContent {
        pub sidebar_toggle_button: RefCell<Option<gtk::Button>>,
        pub breakpoint_bin: RefCell<Option<adw::BreakpointBin>>,
        pub main_content_box: RefCell<Option<gtk::Box>>,
        pub welcome_page: RefCell<Option<Welcome>>,
        pub ai_chat_page: RefCell<Option<AIChat>>,
        pub screen_shot_page: RefCell<Option<ScreenShot>>,
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
            self.create_content();
        }
    }

    impl MainContent {
        fn create_content(&self) {
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

            // 创建 BreakpointBin 容器
            let breakpoint_bin = adw::BreakpointBin::builder().build();

            // 保存 breakpoint_bin 的引用
            self.breakpoint_bin.replace(Some(breakpoint_bin.clone()));

            let sidebar = sidebar::MainSidebar::new();

            // 创建欢迎页面和AI聊天页面
            let welcome_page = Welcome::new();
            let ai_chat_page = AIChat::new();
            let screen_shot_page = ScreenShot::new();

            // 保存页面引用
            self.welcome_page.replace(Some(welcome_page.clone()));
            self.ai_chat_page.replace(Some(ai_chat_page));
            self.screen_shot_page.replace(Some(screen_shot_page));

            // 创建主要内容区域
            let main_content = gtk::Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(10)
                .build();

            // 保存主内容区域引用
            self.main_content_box.replace(Some(main_content.clone()));

            // 默认显示欢迎页面
            main_content.append(&welcome_page);

            // 为主要内容区域添加CSS类
            main_content.add_css_class("main-content");

            // 创建分割视图（带侧边栏）- 使用 AdwOverlaySplitView 实现可折叠侧边栏
            let overlay_view = adw::OverlaySplitView::builder()
                .sidebar(&sidebar)
                .content(&main_content)
                .collapsed(true) // 默认收起侧边栏
                .min_sidebar_width(300.0)
                .max_sidebar_width(300.0)
                .build();

            // 设置分割视图垂直扩展以填满可用空间
            overlay_view.set_vexpand(true);
            // 将 overlay_view 添加到 breakpoint_bin 中
            breakpoint_bin.set_child(Some(&overlay_view));
            // 创建断点 - 当窗口宽度小于 768px 时折叠侧边栏
            let breakpoint = Breakpoint::new(BreakpointCondition::new_length(
                BreakpointConditionLengthType::MaxWidth,
                768.0,
                LengthUnit::Px,
            ));
            // 为断点添加条件应用的属性
            breakpoint.add_setter(&overlay_view, "collapsed", Option::from(&true.to_value()));
            // 将断点添加到 breakpoint_bin
            breakpoint_bin.add_breakpoint(breakpoint);

            // 创建工具栏视图
            let toolbar_view = ToolbarView::builder().content(&breakpoint_bin).build();

            toolbar_view.add_top_bar(&content_header);

            self.obj().set_child(Some(&toolbar_view));

            self.obj().setup_sidebar_toggle(overlay_view);

            // 更新侧边栏按钮图标状态
            if let Some(button) = &*self.sidebar_toggle_button.borrow() {
                button.set_icon_name("sidebar-show-symbolic");
            }
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

    // 添加切换到AI聊天页面的方法
    pub fn show_ai_chat(&self) {
        if let (Some(main_content_box), Some(ai_chat_page)) = (
            &*self.imp().main_content_box.borrow(),
            &*self.imp().ai_chat_page.borrow(),
        ) {
            // 清空当前内容
            while let Some(child) = main_content_box.first_child() {
                main_content_box.remove(&child);
            }

            // 添加AI聊天页面
            main_content_box.append(&ai_chat_page.clone());
        }
    }

    // 添加切换到欢迎页面的方法
    pub fn show_welcome(&self) {
        if let (Some(main_content_box), Some(welcome_page)) = (
            &*self.imp().main_content_box.borrow(),
            &*self.imp().welcome_page.borrow(),
        ) {
            // 清空当前内容
            while let Some(child) = main_content_box.first_child() {
                main_content_box.remove(&child);
            }

            // 添加欢迎页面
            main_content_box.append(&welcome_page.clone());
        }
    }

    pub fn show_screen_shot(&self) {
        if let (Some(main_content_box), Some(screen_shot_page)) = (
            &*self.imp().main_content_box.borrow(),
            &*self.imp().screen_shot_page.borrow(),
        ) {
            // 清空当前内容
            while let Some(child) = main_content_box.first_child() {
                main_content_box.remove(&child);
            }

            // 添加欢迎页面
            main_content_box.append(&screen_shot_page.clone());
        }
    }
}
