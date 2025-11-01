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

            // 添加 Banner
            let banner = adw::Banner::builder()
                .title("欢迎使用 QTools! 这是一个功能强大的工具集")
                .button_label("了解更多")
                .revealed(true)
                .build();

            // 连接 Banner 按钮的点击事件
            banner.connect_button_clicked(|_| {
                println!("Banner按钮被点击");
            });

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

            let carousel_box = gtk::Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(12)
                .vexpand(true)
                .hexpand(true)
                .valign(gtk::Align::Fill)
                .halign(gtk::Align::Fill)
                .build();

            // 添加开始使用按钮
            let start_button = gtk::Button::builder()
                .label("开始使用")
                .css_classes(["suggested-action", "pill"])
                .margin_top(24)
                .build();

            // 创建 Carousel
            let carousel = adw::Carousel::builder()
                .allow_scroll_wheel(true)
                .allow_long_swipes(true)
                .reveal_duration(300)
                .vexpand(true)
                .hexpand(true)
                .valign(gtk::Align::Fill)
                .halign(gtk::Align::Fill)
                .build();

            // 创建第一页
            let page1_content = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(12)
                .vexpand(true)
                .hexpand(true)
                .valign(gtk::Align::Center)
                .halign(gtk::Align::Center)
                .build();

            let page1_title = Label::builder()
                .label("AI聊天助手")
                .css_classes(["title-2"])
                .build();

            let page1_desc = Label::builder()
                .label("与AI进行智能对话，获取您需要的信息和帮助")
                .css_classes(["body"])
                .wrap(true)
                .justify(gtk::Justification::Center)
                .build();

            page1_content.append(&page1_title);
            page1_content.append(&page1_desc);

            // 创建第二页
            let page2_content = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(12)
                .vexpand(true)
                .hexpand(true)
                .valign(gtk::Align::Center)
                .halign(gtk::Align::Center)
                .build();

            let page2_title = Label::builder()
                .label("文件处理工具")
                .css_classes(["title-2"])
                .build();

            let page2_desc = Label::builder()
                .label("强大的文件处理功能，支持多种格式转换和批量操作")
                .css_classes(["body"])
                .wrap(true)
                .justify(gtk::Justification::Center)
                .build();

            page2_content.append(&page2_title);
            page2_content.append(&page2_desc);

            // 创建第三页
            let page3_content = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(12)
                .vexpand(true)
                .hexpand(true)
                .valign(gtk::Align::Center)
                .halign(gtk::Align::Center)
                .build();

            let page3_title = Label::builder()
                .label("系统信息查看")
                .css_classes(["title-2"])
                .build();

            let page3_desc = Label::builder()
                .label("全面的系统信息监控，实时了解设备状态")
                .css_classes(["body"])
                .wrap(true)
                .justify(gtk::Justification::Center)
                .build();

            page3_content.append(&page3_title);
            page3_content.append(&page3_desc);

            // 将页面添加到 Carousel
            carousel.append(&page1_content);
            carousel.append(&page2_content);
            carousel.append(&page3_content);

            // 创建指示器
            let indicator_lines = adw::CarouselIndicatorLines::builder()
                .carousel(&carousel)
                .build();

            carousel_box.append(&carousel);
            carousel_box.append(&indicator_lines);

            // 将所有组件添加到欢迎页面容器中
            welcome_box.append(&banner);
            welcome_box.append(&title);
            welcome_box.append(&description);
            welcome_box.append(&features_title);
            welcome_box.append(&features_list);
            welcome_box.append(&start_button);
            welcome_box.append(&carousel_box);

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
