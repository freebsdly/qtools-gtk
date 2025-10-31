use adw::glib::Object;
use adw::{glib, NavigationPage};

mod imp {
    use super::*;
    use adw::prelude::{BoxExt, NavigationPageExt, WidgetExt};
    use adw::subclass::prelude::{
        NavigationPageImpl, ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt,
    };
    use gtk::subclass::prelude::WidgetImpl;
    use gtk::{Box, Label, Orientation};

    #[derive(Default)]
    pub struct WelcomePage {}

    #[glib::object_subclass]
    impl ObjectSubclass for WelcomePage {
        const NAME: &'static str = "QtoolsWelcomePage";
        type Type = super::WelcomePage;
        type ParentType = NavigationPage;
    }

    impl ObjectImpl for WelcomePage {
        fn constructed(&self) {
            self.parent_constructed();
            self.create_welcome_page();
        }
    }

    impl WelcomePage {
        fn create_welcome_page(&self) {
            // 创建欢迎页面的主要内容容器
            let welcome_box = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(12)
                .margin_top(48)
                .margin_bottom(48)
                .margin_start(24)
                .margin_end(24)
                .build();

            // 添加标题
            let title = Label::builder()
                .label("欢迎使用 QTools")
                .css_classes(["title-1"])
                .build();

            // 添加描述文本
            let description = Label::builder()
                .label("一个基于 Rust 和 GTK 的实用工具集")
                .css_classes(["body"])
                .build();

            // 添加功能列表标题
            let features_title = Label::builder()
                .label("主要功能")
                .css_classes(["heading"])
                .margin_top(24)
                .build();

            // 添加功能列表
            let features_list = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(6)
                .margin_top(12)
                .build();

            let features = [
                "AI聊天助手",
                "文件处理工具",
                "系统信息查看",
                "实用小工具集合",
            ];

            for feature in &features {
                let feature_label = Label::builder()
                    .label(format!("• {}", feature))
                    .css_classes(["body"])
                    .halign(gtk::Align::Start)
                    .build();
                features_list.append(&feature_label);
            }

            // 添加开始使用按钮
            let start_button = gtk::Button::builder()
                .label("开始使用")
                .css_classes(["suggested-action", "pill"])
                .margin_top(24)
                .build();

            // 将所有组件添加到欢迎页面容器中
            welcome_box.append(&title);
            welcome_box.append(&description);
            welcome_box.append(&features_title);
            welcome_box.append(&features_list);
            welcome_box.append(&start_button);

            // 设置欢迎页面内容
            self.obj().set_child(Some(&welcome_box));
        }
    }

    impl WidgetImpl for WelcomePage {}
    impl NavigationPageImpl for WelcomePage {}
}

glib::wrapper! {
    pub struct WelcomePage(ObjectSubclass<imp::WelcomePage>)
        @extends NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl WelcomePage {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
