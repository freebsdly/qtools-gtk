use adw::prelude::*;
use adw::{HeaderBar, NavigationPage, ToolbarView};
use gtk::{Box, Button, Label, Orientation, PolicyType, ScrolledWindow};
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
}

impl MainSidebar {
    pub fn new() -> Self {
        // 创建侧边栏标题栏
        let sidebar_header = HeaderBar::builder()
            .title_widget(&Label::new(Some("")))
            .show_end_title_buttons(true)
            .build();

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

        Self {
            page,
            sidebar_content,
            buttons: Vec::new(),
            menu_items: Vec::new(),
            callback: None,
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
    fn create_menu_button(
        &self,
        menu_item: &MenuItem,
        callback: MenuItemCallback,
    ) -> Button {
        let button = Button::builder().label(&menu_item.label).build();

        let item_clone = menu_item.clone();
        button.connect_clicked(move |_| {
            callback.borrow()(&item_clone);
        });

        button
    }
}