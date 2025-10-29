// src/main_window/mod.rs
mod content;
pub mod menu;
mod sidebar;

use crate::app::QtoolsApplication;
use adw::glib::Object;
use adw::prelude::{
    ActionMapExt, ActionRowExt, AdwDialogExt, FileExt, PreferencesGroupExt, PreferencesPageExt,
};
use adw::subclass::prelude::ObjectSubclassIsExt;
use adw::{Dialog, glib};
use gtk::gio;
use gtk::prelude::{GtkApplicationExt, GtkWindowExt, WidgetExt};

mod imp {
    use crate::main_window::{content, sidebar};
    use adw::prelude::BreakpointBinExt;
    use adw::prelude::{AdwApplicationWindowExt, ToValue};
    use adw::subclass::prelude::{
        AdwApplicationWindowImpl, ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt,
    };
    use adw::{Breakpoint, BreakpointCondition, BreakpointConditionLengthType, LengthUnit, glib};
    use gtk::SizeRequestMode::ConstantSize;
    use gtk::prelude::{GtkWindowExt, WidgetExt};
    use gtk::subclass::prelude::{ApplicationWindowImpl, WidgetImpl, WindowImpl};
    use log::info;
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct MainWindow {
        pub breakpoint_bin: RefCell<Option<adw::BreakpointBin>>,
    }

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

            // 创建 BreakpointBin 容器
            let breakpoint_bin = adw::BreakpointBin::builder().build();

            // 保存 breakpoint_bin 的引用
            self.breakpoint_bin.replace(Some(breakpoint_bin.clone()));

            // 创建分割视图（带侧边栏）- 使用 AdwOverlaySplitView 实现可折叠侧边栏
            let overlay_view = adw::OverlaySplitView::builder()
                .sidebar(&sidebar)
                .content(&main_content)
                .collapsed(false) // 默认展开
                .min_sidebar_width(200.0) // 最小宽度设为200，足够显示图标
                .max_sidebar_width(300.0)
                .build();

            // 设置分割视图垂直扩展以填满可用空间
            overlay_view.set_vexpand(true);

            // 将 overlay_view 添加到 breakpoint_bin 中
            breakpoint_bin.set_child(Some(&overlay_view));

            let obj = self.obj();
            obj.set_title(Some("Qtools"));
            obj.set_default_size(1024, 768);
            obj.set_content(Some(&breakpoint_bin));

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

            // 在窗口构造完成后初始化窗口级别的动作
            obj.setup_actions();

            // 设置侧边栏切换按钮事件处理器
            main_content.setup_sidebar_toggle(overlay_view);
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

    // 获取BreakpointBin的引用
    pub fn breakpoint_bin(&self) -> Option<adw::BreakpointBin> {
        self.imp().breakpoint_bin.borrow().clone()
    }

    // 动作处理函数保留在窗口中，因为它们与特定窗口相关
    fn action_new(&self) {
        // 创建文件保存对话框
        let dialog = gtk::FileDialog::builder()
            .title("新建文件")
            .modal(true)
            .build();

        // 添加文件过滤器
        let filter = gtk::FileFilter::new();
        filter.set_name(Some("文本文件"));
        filter.add_mime_type("text/plain");

        let filters = gio::ListStore::new::<gtk::FileFilter>();
        filters.append(&filter);
        dialog.set_filters(Some(&filters));

        let window = self.clone();
        dialog.save(Some(self), None::<&gio::Cancellable>, move |result| {
            if let Ok(file) = result {
                println!("创建新文件: {:?}", file.path());
                // 这里可以添加实际创建文件的逻辑
            }
        });
    }

    fn action_open(&self) {
        // 创建文件打开对话框
        let dialog = gtk::FileDialog::builder()
            .title("打开文件")
            .modal(true)
            .build();

        // 添加文件过滤器
        let filter = gtk::FileFilter::new();
        filter.set_name(Some("文本文件"));
        filter.add_mime_type("text/plain");

        let filters = gio::ListStore::new::<gtk::FileFilter>();
        filters.append(&filter);
        dialog.set_filters(Some(&filters));

        let window = self.clone();
        dialog.open(Some(self), None::<&gio::Cancellable>, move |result| {
            if let Ok(file) = result {
                println!("打开文件: {:?}", file.path());
                // 这里可以添加实际打开文件的逻辑
            }
        });
    }

    fn action_save(&self) {
        println!("保存文件");
    }

    fn action_save_as(&self) {
        println!("另存为");
    }

    fn action_preferences(&self) {
        // 创建首选项对话框 (使用新的 AdwDialog API)
        let preferences = Dialog::builder()
            .title("首选项")
            .content_width(600)
            .content_height(500)
            .build();

        // 创建一个示例设置页面
        let general_page = adw::PreferencesPage::new();
        general_page.set_title("常规");
        general_page.set_icon_name(Some("preferences-system-symbolic"));

        let general_group = adw::PreferencesGroup::new();
        general_group.set_title("常规设置");

        // 添加示例设置项
        let switch = gtk::Switch::new();
        switch.set_valign(gtk::Align::Center);
        switch.set_active(true);

        let demo_row = adw::ActionRow::builder()
            .title("示例设置")
            .subtitle("这是一个示例设置项")
            .activatable_widget(&switch)
            .build();
        demo_row.add_suffix(&switch);

        general_group.add(&demo_row);
        general_page.add(&general_group);

        // 将页面添加到首选项对话框
        preferences.set_child(Some(&general_page));

        // 显示首选项对话框
        preferences.present(Some(self));
    }

    // 窗口级别的动作初始化方法
    pub fn setup_actions(&self) {
        // 定义窗口级动作配置
        let actions = vec![
            ("new", Self::action_new as fn(&_)),
            ("open", Self::action_open),
            ("save", Self::action_save),
            ("save-as", Self::action_save_as),
            ("preferences", Self::action_preferences),
        ];

        for (name, callback) in actions {
            let action = gio::SimpleAction::new(name, None);
            let window = self.clone();
            action.connect_activate(move |_, _| {
                callback(&window);
            });
            self.add_action(&action);
        }

        // 获取应用程序引用以设置快捷键
        if let Some(app) = self.application() {
            // 注册窗口级快捷键
            app.set_accels_for_action("win.new", &["<Ctrl>n"]);
            app.set_accels_for_action("win.open", &["<Ctrl>o"]);
            app.set_accels_for_action("win.save", &["<Ctrl>s"]);
        }
    }
}
