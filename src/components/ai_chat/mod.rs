use adw::glib::Object;
use adw::{glib, NavigationPage, ToolbarView};
use gtk::prelude::*;
use gtk::{Box, Button, Entry, Orientation, PolicyType, ScrolledWindow, TextView};

mod imp {
    use super::*;
    use adw::prelude::NavigationPageExt;
    use adw::subclass::prelude::{
        NavigationPageImpl, ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt,
    };
    use gtk::subclass::prelude::WidgetImpl;

    #[derive(Default)]
    pub struct AIChat {}

    #[glib::object_subclass]
    impl ObjectSubclass for AIChat {
        const NAME: &'static str = "QtoolsAIChat";
        type Type = super::AIChat;
        type ParentType = NavigationPage;
    }

    impl ObjectImpl for AIChat {
        fn constructed(&self) {
            self.parent_constructed();
            self.create_ai_chat();
        }
    }

    impl AIChat {
        fn create_ai_chat(&self) {
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
                .hexpand(true)
                .build();

            // 创建发送按钮
            let send_button = Button::builder()
                .label("发送")
                .css_classes(["suggested-action"])
                .build();

            // 创建底部输入区域容器
            let input_box = Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(10)
                .build();

            input_box.append(&input_entry);
            input_box.append(&send_button);

            // 创建主内容区域
            let main_content = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(10)
                .build();

            main_content.append(&chat_scroll);
            main_content.append(&input_box);

            // 为AI聊天区域添加CSS类
            main_content.add_css_class("ai-chat");

            // 创建工具栏视图
            let toolbar_view = ToolbarView::builder().content(&main_content).build();

            // 创建AI聊天页面
            self.obj().set_child(Some(&toolbar_view));
            self.obj().set_title("AI Chat");
        }
    }

    impl WidgetImpl for AIChat {}
    impl NavigationPageImpl for AIChat {}
}

glib::wrapper! {
    pub struct AIChat(ObjectSubclass<imp::AIChat>)
        @extends NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl AIChat {
    pub fn new() -> Self {
        Object::builder()
            .property("title", "AI Chat")
            .build()
    }
}
