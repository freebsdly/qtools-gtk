use adw::prelude::*;
use adw::{HeaderBar, NavigationPage, ToolbarView, glib};
use gtk::{Box, Button, Label, Orientation, PolicyType, ScrolledWindow, Widget};
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

pub struct MainSidebar {
    pub page: NavigationPage,
    sidebar_content: Box,
    buttons: Vec<Button>,
    menu_items: Vec<MenuItem>,
    callback: Option<MenuItemCallback>,
    toggle_button: Button,
    // 保存对分割视图的弱引用，用于控制折叠状态
    split_view: Option<glib::WeakRef<adw::OverlaySplitView>>,
    sidebar_container: Box, // 将容器作为结构体的一部分
    toolbar_buttons: Vec<Button>,
}

impl MainSidebar {
    pub fn new() -> Self {
        // 创建展开/收起按钮
        let toggle_button = Button::builder()
            .icon_name("sidebar-show-right-symbolic")
            .build();

        // 创建侧边栏标题栏
        let sidebar_header = HeaderBar::builder()
            .title_widget(&Label::new(Some("")))
            .show_end_title_buttons(true)
            .build();

        // 将展开/收起按钮添加到标题栏的开头
        sidebar_header.pack_start(&toggle_button);

        sidebar_header.add_css_class("header-bar");

        // 创建侧边栏内容
        let sidebar_content = Box::builder()
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

        // 创建侧边栏页面
        let page = NavigationPage::builder()
            .child(&toolbar_view)
            .title("侧边栏") // 为NavigationPage设置标题
            .build();

        // 创建侧边栏容器（用于显示工具栏或完整侧边栏）
        let sidebar_container = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .build();

        let mut sidebar = Self {
            page,
            sidebar_content,
            buttons: Vec::new(),
            menu_items: Vec::new(),
            callback: None,
            toggle_button,
            split_view: None,
            sidebar_container,
            toolbar_buttons: Vec::new(),
        };

        // 初始化容器内容
        sidebar.init_sidebar_container();

        sidebar
    }

    // 获取侧边栏容器的引用
    pub fn get_sidebar_container(&self) -> &Box {
        &self.sidebar_container
    }

    // 设置分割视图引用，用于直接控制
    pub fn set_split_view(&mut self, split_view: &adw::OverlaySplitView) {
        self.split_view = Some(split_view.downgrade());

        // 设置切换按钮的功能
        self.setup_toggle_button();

        // 监听分割视图的折叠状态变化
        self.setup_collapse_listener();
    }

    // 初始化侧边栏容器内容
    fn init_sidebar_container(&mut self) {
        // 先清空容器
        while let Some(child) = self.sidebar_container.first_child() {
            self.sidebar_container.remove(&child);
        }

        // 添加完整侧边栏
        self.sidebar_container.append(&self.page);
    }

    // 设置切换按钮的功能
    fn setup_toggle_button(&mut self) {
        let toggle_button = self.toggle_button.clone();

        if let Some(split_view) = &self.split_view {
            if let Some(split_view_ref) = split_view.upgrade() {
                let split_view_clone = split_view_ref.clone();

                self.toggle_button.connect_clicked(move |_| {
                    let collapsed = split_view_clone.is_collapsed();
                    split_view_clone.set_collapsed(!collapsed);
                });
            }
        }
    }

    // 监听分割视图的折叠状态变化并更新按钮图标
    fn setup_collapse_listener(&mut self) {
        if let Some(split_view) = &self.split_view {
            if let Some(split_view_ref) = split_view.upgrade() {
                let toggle_button = self.toggle_button.clone();
                let page = self.page.clone();
                let menu_items = self.menu_items.clone();
                let callback = self.callback.clone();
                let sidebar_container = self.sidebar_container.clone();

                // 监听折叠状态变化
                split_view_ref.connect_collapsed_notify(move |split_view| {
                    let collapsed = split_view.is_collapsed();

                    if collapsed {
                        // 当折叠时，显示工具栏模式
                        toggle_button.set_icon_name("sidebar-show-right-symbolic");

                        // 清空容器并添加工具栏按钮
                        while let Some(child) = sidebar_container.first_child() {
                            sidebar_container.remove(&child);
                        }

                        // 创建工具栏按钮
                        let toolbar = Box::builder()
                            .orientation(Orientation::Vertical)
                            .spacing(5)
                            .margin_start(5)
                            .margin_end(5)
                            .margin_top(10)
                            .margin_bottom(10)
                            .build();

                        toolbar.add_css_class("toolbar");

                        if let Some(cb) = &callback {
                            let callback_ref = cb.clone();
                            for item in &menu_items {
                                let button = Button::builder()
                                    .icon_name(&item.icon)
                                    .css_classes(vec!["flat".to_string(), "compact".to_string()])
                                    .width_request(40)
                                    .height_request(40)
                                    .build();

                                let item_clone = item.clone();
                                let cb_clone = callback_ref.clone();
                                button.connect_clicked(move |_| {
                                    cb_clone.borrow()(&item_clone);
                                });

                                toolbar.append(&button);
                            }
                        }

                        sidebar_container.append(&toolbar);
                    } else {
                        // 当展开时，显示完整侧边栏
                        toggle_button.set_icon_name("sidebar-hide-right-symbolic");

                        // 清空容器并添加完整侧边栏
                        while let Some(child) = sidebar_container.first_child() {
                            sidebar_container.remove(&child);
                        }

                        sidebar_container.append(&page);
                    }
                });
            }
        }
    }

    // 添加菜单项到侧边栏
    pub fn add_menu_item(&mut self, menu_item: MenuItem) {
        self.menu_items.push(menu_item);
    }

    // 设置菜单项点击回调
    pub fn set_callback<F: Fn(&MenuItem) + 'static>(&mut self, callback: F) {
        self.callback = Some(Rc::new(RefCell::new(callback)));
    }

    // 根据菜单项生成菜单界面
    pub fn build_menu(&mut self) {
        // 清空现有内容
        while let Some(child) = self.sidebar_content.first_child() {
            self.sidebar_content.remove(&child);
        }

        self.buttons.clear();

        // 如果没有设置回调函数，则不创建按钮
        let callback = match &self.callback {
            Some(cb) => cb.clone(),
            None => return,
        };

        // 根据菜单项生成按钮
        for item in &self.menu_items {
            let button = self.create_menu_button(item, callback.clone());
            self.sidebar_content.append(&button);
            self.buttons.push(button);
        }
    }

    // 创建菜单按钮
    fn create_menu_button(&self, menu_item: &MenuItem, callback: MenuItemCallback) -> Button {
        let button = Button::builder()
            .label(&menu_item.label)
            .icon_name(&menu_item.icon)
            .build();

        let item_clone = menu_item.clone();
        button.connect_clicked(move |_| {
            callback.borrow()(&item_clone);
        });

        button
    }
}
