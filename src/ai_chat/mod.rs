use adw::prelude::*;
use adw::{NavigationPage, ToolbarView};
use gtk::{Box, Entry, Orientation, PolicyType, ScrolledWindow, TextView};

pub struct AIChat {
    pub page: NavigationPage,
}

impl AIChat {
    pub fn new() -> Self {
        // 创建聊天历史区域
        let chat_history = TextView::builder()
            .editable(false)
            .wrap_mode(gtk::WrapMode::Word)
            .build();

        let chat_scroll = ScrolledWindow::builder()
            .vexpand(true)
            .hscrollbar_policy(PolicyType::Never)
            .child(&chat_history)
            .build();

        // 创建输入区域
        let input_entry = Entry::builder()
            .placeholder_text("输入消息...")
            .build();

        // 创建主内容区域
        let main_content = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .build();

        main_content.append(&chat_scroll);
        main_content.append(&input_entry);

        // 为AI聊天区域添加CSS类
        main_content.add_css_class("ai-chat");

        // 创建工具栏视图
        let toolbar_view = ToolbarView::builder()
            .content(&main_content)
            .build();

        // 创建AI聊天页面
        let page = NavigationPage::builder()
            .child(&toolbar_view)
            .title("AI Chat")
            .build();

        Self { page }
    }
}