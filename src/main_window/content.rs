use adw::prelude::*;
use adw::{NavigationPage, ToolbarView, HeaderBar};
use gtk::{Box, Label, Orientation};
use crate::main_window::menu::AppMenu;

pub struct MainContent {
    pub page: NavigationPage,
    content_box: Box,
}

impl MainContent {
    pub fn new() -> Self {
        // 创建菜单
        let app_menu = AppMenu::new();
        
        // 创建内容区域标题栏
        let content_header = HeaderBar::builder()
            .title_widget(&Label::new(Some("")))
            .show_start_title_buttons(true)
            .build();
            
        // 将菜单按钮添加到标题栏的开始位置
        content_header.pack_end(&app_menu.menu_button);

        content_header.add_css_class("header-bar");
        
        // 创建主要内容区域
        let main_content = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .build();
        main_content.append(&Label::new(Some("主要内容区域")));
        
        // 为主要内容区域添加CSS类
        main_content.add_css_class("main-content");
        
        // 创建工具栏视图
        let toolbar_view = ToolbarView::builder()
            .content(&main_content)
            .build();
        
        toolbar_view.add_top_bar(&content_header);
        
        // 创建主内容页面
        let page = NavigationPage::builder()
            .child(&toolbar_view)
            .title("主要内容") // 为NavigationPage设置标题
            .build();
        
        Self { 
            page,
            content_box: main_content,
        }
    }
    
    // 获取主内容区域的Box，用于添加或替换内容
    pub fn get_content_box(&self) -> &Box {
        &self.content_box
    }
}