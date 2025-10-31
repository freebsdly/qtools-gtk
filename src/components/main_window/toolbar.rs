use adw::glib::Object;
use adw::subclass::prelude::ObjectSubclassIsExt;
use adw::{glib, NavigationPage};

mod imp {
    use super::*;
    use adw::glib::clone::Downgrade;
    use adw::prelude::ButtonExt;
    use adw::prelude::NavigationPageExt;
    use adw::subclass::prelude::{
        NavigationPageImpl, ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt,
    };
    use adw::{glib, HeaderBar, ToolbarView};
    use gtk::prelude::{BoxExt, ToggleButtonExt, WidgetExt};
    use gtk::subclass::prelude::WidgetImpl;
    use gtk::{Label, Orientation, PolicyType, ScrolledWindow, ToggleButton};
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct MainToolbar {
        // 存储按钮引用，以便管理选中状态
        pub buttons: RefCell<Vec<ToggleButton>>,
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
            self.create_toolbar();
        }
    }

    impl MainToolbar {
        fn create_toolbar(&self) {
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
            let new_button = create_toolbar_button("document-new-symbolic", "新建");
            let open_button = create_toolbar_button("document-open-symbolic", "打开");
            let save_button = create_toolbar_button("document-save-symbolic", "保存");

            let obj = self.obj();
            let buttons_ref = &obj.imp().buttons;

            // 存储按钮引用
            buttons_ref.borrow_mut().push(new_button.clone());
            buttons_ref.borrow_mut().push(open_button.clone());
            buttons_ref.borrow_mut().push(save_button.clone());

            // 添加按钮点击事件处理，实现选中效果
            setup_button_click_handler(&new_button, buttons_ref.clone());
            setup_button_click_handler(&open_button, buttons_ref.clone());
            setup_button_click_handler(&save_button, buttons_ref.clone());

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
    fn setup_button_click_handler(button: &ToggleButton, buttons: RefCell<Vec<ToggleButton>>) {
        let button_weak = button.downgrade();
        button.connect_clicked(move |_| {
            if let Some(btn) = button_weak.upgrade() {
                // 清除其他按钮的选中状态
                for b in buttons.borrow().iter() {
                    if b != &btn {
                        b.set_active(false);
                    }
                }

                // 确保当前按钮被选中
                btn.set_active(true);
            }
        });
    }

    // 创建工具栏按钮的辅助函数
    fn create_toolbar_button(icon_name: &str, tooltip: &str) -> ToggleButton {
        let button = ToggleButton::new();
        let image = gtk::Image::from_icon_name(icon_name);
        image.set_icon_size(gtk::IconSize::Large);
        button.set_child(Some(&image));
        button.set_tooltip_text(Some(tooltip));
        button.add_css_class("toolbar-button");
        button.set_can_shrink(false);
        button
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
