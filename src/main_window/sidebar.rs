use adw::prelude::*;
use adw::{HeaderBar, NavigationPage, ToolbarView};
use gtk::{Box, Label, Orientation, PolicyType, ScrolledWindow};

pub struct MainSidebar {
    pub page: NavigationPage,
}

impl MainSidebar {
    pub fn new() -> Self {
        // 创建侧边栏标题栏
        let sidebar_header = HeaderBar::builder()
            .title_widget(&Label::new(Some("侧边栏")))
            .show_end_title_buttons(true)
            .build();

        sidebar_header.add_css_class("header-bar");

        // 创建侧边栏内容
        let sidebar_content = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .build();
        sidebar_content.append(&Label::new(Some("侧边栏项目 1")));
        sidebar_content.append(&Label::new(Some("侧边栏项目 2")));
        sidebar_content.append(&Label::new(Some("侧边栏项目 3")));

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

        Self { page }
    }
}
