use adw::glib::Object;
use adw::{glib, Dialog};
use gtk::prelude::*;

mod imp {
    use super::*;
    use adw::prelude::AdwDialogExt;
    use adw::subclass::dialog::AdwDialogImpl;
    use adw::subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt};
    use gtk::subclass::prelude::{WidgetImpl, WindowImpl};

    #[derive(Default)]
    pub struct TutorialDialog {}

    #[glib::object_subclass]
    impl ObjectSubclass for TutorialDialog {
        const NAME: &'static str = "QtoolsTutorialDialog";
        type Type = super::TutorialDialog;
        type ParentType = Dialog;
    }

    impl ObjectImpl for TutorialDialog {
        fn constructed(&self) {
            self.parent_constructed();
            self.build_ui();
        }
    }

    impl TutorialDialog {
        fn build_ui(&self) {
            let obj = self.obj();
            obj.set_content_width(600);
            obj.set_content_height(500);
            obj.set_can_close(true);

            // 创建引导页的主要内容容器
            let tutorial_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(12)
                .margin_top(24)
                .margin_bottom(24)
                .margin_start(24)
                .margin_end(24)
                .build();

            // 添加 Logo 或图标占位符
            let image = gtk::Image::builder()
                .icon_name("applications-development")
                .pixel_size(128)
                .margin_bottom(12)
                .build();

            // 添加标题
            let title = gtk::Label::builder()
                .label("QTools 工具集")
                .css_classes(["title-1"])
                .build();

            // 添加描述文本
            let description = gtk::Label::builder()
                .label("一个基于 Rust 和 GTK 的实用工具集，为您提供各种实用功能")
                .css_classes(["body"])
                .wrap(true)
                .justify(gtk::Justification::Center)
                .margin_bottom(12)
                .build();

            // 添加特性列表
            let features_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(6)
                .margin_top(12)
                .margin_bottom(12)
                .build();

            let features = [
                "🤖 AI聊天助手 - 与人工智能进行智能对话",
                "📁 文件处理 - 强大的文件处理和批量操作功能",
                "🖥️ 系统信息 - 全面的系统信息查看和监控",
                "🔧 实用工具 - 各种日常使用的便捷小工具",
            ];

            for feature in &features {
                let feature_label = gtk::Label::builder()
                    .label(*feature)
                    .css_classes(["body"])
                    .halign(gtk::Align::Start)
                    .build();
                features_box.append(&feature_label);
            }

            // 添加开始按钮
            let start_button = gtk::Button::builder()
                .label("开始使用")
                .css_classes(["suggested-action", "pill"])
                .halign(gtk::Align::Center)
                .margin_top(12)
                .build();

            // 关闭对话框
            let obj_clone = obj.clone();
            start_button.connect_clicked(move |_| {
                AdwDialogExt::close(&obj_clone);
            });

            // 组装所有组件
            tutorial_box.append(&image);
            tutorial_box.append(&title);
            tutorial_box.append(&description);
            tutorial_box.append(&features_box);
            tutorial_box.append(&start_button);

            AdwDialogExt::set_child(&*obj, Some(&tutorial_box));
        }
    }

    impl WidgetImpl for TutorialDialog {}
    impl WindowImpl for TutorialDialog {}
    impl AdwDialogImpl for TutorialDialog {}
}

glib::wrapper! {
    pub struct TutorialDialog(ObjectSubclass<imp::TutorialDialog>)
        @extends Dialog, adw::Window, gtk::Window, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl TutorialDialog {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
