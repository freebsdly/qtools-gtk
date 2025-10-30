use adw::glib::Object;
use adw::{glib, NavigationPage};

mod imp {
    use super::*;
    use adw::prelude::ButtonExt;
    use adw::prelude::NavigationPageExt;
    use adw::subclass::prelude::{
        NavigationPageImpl, ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt,
    };
    use adw::{glib, HeaderBar, ToolbarView};
    use gtk::prelude::{BoxExt, WidgetExt};
    use gtk::subclass::prelude::WidgetImpl;
    use gtk::{Label, Orientation, PolicyType, ScrolledWindow};
    use std::cell::RefCell;
    use adw::glib::clone::Downgrade;

    #[derive(Default)]
    pub struct MainToolbar {
        // 存储按钮引用，以便管理选中状态
        pub buttons: RefCell<Vec<gtk::Button>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainToolbar {
        const NAME: &'static str = "QtoolsMainToolbar";
        type Type = super::MainToolbar;
        type ParentType = NavigationPage;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for MainToolbar {
        fn constructed(&self) {
            self.parent_constructed();

            // 创建侧边栏标题栏
            let sidebar_header = HeaderBar::builder()
                .title_widget(&Label::new(Some("")))
                .show_end_title_buttons(true)
                .build();

            sidebar_header.add_css_class("header-bar");

            // 创建工具栏按钮容器
            let toolbar_box = gtk::Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(0)
                .margin_start(0)
                .margin_end(0)
                .margin_top(0)
                .margin_bottom(0)
                .build();

            // 创建带图标大小的按钮
            let new_button = gtk::Button::new();
            let new_image = gtk::Image::from_icon_name("document-new-symbolic");
            new_image.set_icon_size(gtk::IconSize::Large);
            new_button.set_child(Some(&new_image));
            new_button.set_tooltip_text(Some("新建"));
            new_button.add_css_class("flat");
            new_button.add_css_class("toolbar-button");
            new_button.set_can_shrink(false);
            new_button.set_size_request(48, 48);

            let open_button = gtk::Button::new();
            let open_image = gtk::Image::from_icon_name("document-open-symbolic");
            open_image.set_icon_size(gtk::IconSize::Large);
            open_button.set_child(Some(&open_image));
            open_button.set_tooltip_text(Some("打开"));
            open_button.add_css_class("flat");
            open_button.add_css_class("toolbar-button");
            open_button.set_can_shrink(false);
            open_button.set_size_request(48, 48);

            let save_button = gtk::Button::new();
            let save_image = gtk::Image::from_icon_name("document-save-symbolic");
            save_image.set_icon_size(gtk::IconSize::Large);
            save_button.set_child(Some(&save_image));
            save_button.set_tooltip_text(Some("保存"));
            save_button.add_css_class("flat");
            save_button.add_css_class("toolbar-button");
            save_button.set_can_shrink(false);
            save_button.set_size_request(48, 48);

            // 存储按钮引用
            self.buttons.borrow_mut().push(new_button.clone());
            self.buttons.borrow_mut().push(open_button.clone());
            self.buttons.borrow_mut().push(save_button.clone());

            // 添加按钮点击事件处理，实现选中效果
            setup_button_click_handler(&new_button, self.buttons.clone());
            setup_button_click_handler(&open_button, self.buttons.clone());
            setup_button_click_handler(&save_button, self.buttons.clone());

            toolbar_box.append(&new_button);
            toolbar_box.append(&open_button);
            toolbar_box.append(&save_button);

            // 创建侧边栏内容
            let sidebar_content = gtk::Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(0)
                .build();

            // 添加工具栏到侧边栏内容顶部
            sidebar_content.append(&toolbar_box);

            // 为侧边栏内容添加CSS类
            sidebar_content.add_css_class("main-toolbar");

            // 将侧边栏内容放入滚动视图中
            let sidebar_scroll = ScrolledWindow::builder()
                .vexpand(true)
                .hscrollbar_policy(PolicyType::Never)
                .child(&sidebar_content)
                .build();

            // 创建工具栏视图
            let toolbar_view = ToolbarView::builder().content(&sidebar_scroll).build();

            toolbar_view.add_top_bar(&sidebar_header);

            self.obj().set_child(Some(&toolbar_view));
        }
    }

    // 添加按钮点击处理器，用于切换选中状态
    fn setup_button_click_handler(button: &gtk::Button, buttons: RefCell<Vec<gtk::Button>>) {
        let button_weak = button.downgrade();
        button.connect_clicked(move |_| {
            if let Some(btn) = button_weak.upgrade() {
                // 清除所有按钮的选中状态
                for b in buttons.borrow().iter() {
                    b.remove_css_class("checked");
                }
                
                // 设置当前按钮为选中状态
                btn.add_css_class("checked");
            }
        });
    }

    // Trait shared by all widgets
    impl WidgetImpl for MainToolbar {}

    impl NavigationPageImpl for MainToolbar {}
}

glib::wrapper! {
    pub struct MainToolbar(ObjectSubclass<imp::MainToolbar>)
        @extends NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl MainToolbar {
    pub fn new() -> Self {
        Object::builder().build()
    }
}