// src/main_window/mod.rs
pub mod about;
mod content;
pub mod menu;
mod sidebar;

use crate::ai_chat;
use adw::ApplicationWindow;
use adw::prelude::*;
use content::MainContent;
use gtk::{Label, gio};
use sidebar::{MainSidebar, MenuItem};

pub struct MainWindow {
    pub window: ApplicationWindow,
    ai_chat: ai_chat::AIChat,
    main_content: MainContent,
    sidebar: MainSidebar,
}

impl MainWindow {
    pub fn new(app: &adw::Application) -> Self {
        // 创建侧边栏
        let mut sidebar = MainSidebar::new();

        // 添加菜单项
        Self::setup_menu_items(&mut sidebar);

        // 创建主要内容区域
        let main_content = MainContent::new();

        // 创建AI聊天模块
        let ai_chat = ai_chat::AIChat::new();

        // 创建分割视图（带侧边栏）
        let split_view = adw::NavigationSplitView::builder()
            .sidebar(&sidebar.page)
            .content(&main_content.page)
            .min_sidebar_width(200.0)
            .max_sidebar_width(300.0)
            .build();

        // 设置分割视图垂直扩展以填满可用空间
        split_view.set_vexpand(true);

        // 创建窗口
        let window = ApplicationWindow::builder()
            .application(app)
            .title("My GTK App")
            .default_width(1000)
            .default_height(700)
            .content(&split_view)
            .build();

        let mut main_window = Self {
            window,
            ai_chat,
            main_content,
            sidebar,
        };

        // 设置菜单项点击事件
        main_window.setup_menu_callbacks();

        main_window
    }

    pub fn present(&self) {
        self.window.present();
    }

    // 设置菜单项
    fn setup_menu_items(sidebar: &mut MainSidebar) {
        // 添加AI聊天菜单项
        let ai_chat_item = MenuItem::new("ai_chat", "AI Chat", "chat-symbolic");
        sidebar.add_menu_item(ai_chat_item);

        // 添加demo菜单项
        let demo_item = MenuItem::new("demo", "Demo", "emblem-system-symbolic");
        sidebar.add_menu_item(demo_item);

        // 构建菜单
        sidebar.build_menu(|item| {
            println!("点击了菜单项: {}", item.id);
        });
    }

    // 设置菜单回调
    fn setup_menu_callbacks(&mut self) {
        let main_content_box = self.main_content.get_content_box().clone();
        let ai_chat_widget = self.ai_chat.page.clone();

        // 重新构建菜单并设置正确的回调
        self.sidebar.build_menu(move |item| {
            match item.id.as_str() {
                "ai_chat" => {
                    // 清除主内容区域的所有子组件
                    while let Some(child) = main_content_box.first_child() {
                        main_content_box.remove(&child);
                    }
                    // 添加AI聊天界面到主内容区域
                    main_content_box.append(&ai_chat_widget);
                }
                "demo" => {
                    // 清除主内容区域的所有子组件
                    while let Some(child) = main_content_box.first_child() {
                        main_content_box.remove(&child);
                    }
                    // 添加Demo界面到主内容区域
                    main_content_box
                        .append(&Label::new(Some("这是一个Demo页面".to_string().as_str())));
                }
                _ => {
                    println!("未处理的菜单项: {}", item.id);
                }
            }
        });
    }

    // 动作处理函数
    fn action_new(_window: &ApplicationWindow) {
        println!("新建文件");
    }

    fn action_open(_window: &ApplicationWindow) {
        println!("打开文件");
    }

    fn action_save(_window: &ApplicationWindow) {
        println!("保存文件");
    }

    fn action_save_as(_window: &ApplicationWindow) {
        println!("另存为");
    }

    fn action_preferences(_window: &ApplicationWindow) {
        println!("打开首选项");
    }

    fn action_about(window: &ApplicationWindow) {
        crate::main_window::about::AppAboutDialog::show(window);
    }

    fn action_quit(_window: &ApplicationWindow) {
        println!("退出应用");
    }

    pub fn setup_actions(&self, app: &adw::Application) {
        // 创建应用级别的动作
        let new_action = gio::SimpleAction::new("new", None);
        let window = self.window.clone();
        new_action.connect_activate(move |_, _| {
            Self::action_new(&window);
        });
        app.add_action(&new_action);

        let open_action = gio::SimpleAction::new("open", None);
        let window = self.window.clone();
        open_action.connect_activate(move |_, _| {
            Self::action_open(&window);
        });
        app.add_action(&open_action);

        let save_action = gio::SimpleAction::new("save", None);
        let window = self.window.clone();
        save_action.connect_activate(move |_, _| {
            Self::action_save(&window);
        });
        app.add_action(&save_action);

        let save_as_action = gio::SimpleAction::new("save-as", None);
        let window = self.window.clone();
        save_as_action.connect_activate(move |_, _| {
            Self::action_save_as(&window);
        });
        app.add_action(&save_as_action);

        let preferences_action = gio::SimpleAction::new("preferences", None);
        let window = self.window.clone();
        preferences_action.connect_activate(move |_, _| {
            Self::action_preferences(&window);
        });
        app.add_action(&preferences_action);

        let about_action = gio::SimpleAction::new("about", None);
        let window = self.window.clone();
        about_action.connect_activate(move |_, _| {
            Self::action_about(&window);
        });
        app.add_action(&about_action);

        let quit_action = gio::SimpleAction::new("quit", None);
        let window = self.window.clone();
        quit_action.connect_activate(move |_, _| {
            Self::action_quit(&window);
        });
        app.add_action(&quit_action);
    }
}
