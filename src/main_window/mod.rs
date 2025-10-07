// src/main_window/mod.rs
pub mod about;
mod content;
pub mod menu;
mod sidebar;

use adw::ApplicationWindow;
use adw::prelude::*;
use content::MainContent;
use gtk::gio;
use sidebar::MainSidebar;

pub struct MainWindow {
    pub window: ApplicationWindow,
}

impl MainWindow {
    pub fn new(app: &adw::Application) -> Self {
        // 创建侧边栏
        let sidebar = MainSidebar::new();

        // 创建主要内容区域
        let content = MainContent::new();

        // 创建分割视图（带侧边栏）
        let split_view = adw::NavigationSplitView::builder()
            .sidebar(&sidebar.page)
            .content(&content.page)
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

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
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
        about::AppAboutDialog::show(window);
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
