use adw::glib;
use adw::glib::Object;
use adw::subclass::prelude::ObjectSubclassIsExt;
use adw::NavigationPage;

mod imp {
    use super::*;
    use adw::glib::clone;
    use adw::prelude::NavigationPageExt;
    use adw::prelude::{ButtonExt, ObjectExt};
    use adw::subclass::prelude::*;
    use adw::{glib, HeaderBar, ToolbarView};
    use gtk::prelude::{BoxExt, ToggleButtonExt, WidgetExt};
    use gtk::subclass::prelude::WidgetImpl;
    use gtk::{Label, Orientation, PolicyType, ScrolledWindow, ToggleButton};
    use std::cell::RefCell;

    // 定义工具栏按钮的结构体
    #[derive(Debug, Clone)]
    struct ToolbarButton {
        icon_name: &'static str,
        tooltip: &'static str,
        action: ToolbarAction,
    }

    // 定义工具栏按钮的动作类型
    #[derive(Debug, Clone)]
    enum ToolbarAction {
        Signal(&'static str),  // 发送信号
        Toggle,                // 切换选中状态
    }

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
        fn signals() -> &'static [glib::subclass::Signal] {
            use once_cell::sync::Lazy;
            static SIGNALS: Lazy<Vec<glib::subclass::Signal>> = Lazy::new(|| {
                vec![
                    glib::subclass::Signal::builder("show-ai-chat")
                        .flags(glib::SignalFlags::RUN_LAST | glib::SignalFlags::ACTION)
                        .build(),
                ]
            });
            SIGNALS.as_ref()
        }

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

            let obj = self.obj();
            let buttons_ref = &obj.imp().buttons;

            // 定义工具栏按钮配置表
            let toolbar_buttons = vec![
                ToolbarButton {
                    icon_name: "document-new-symbolic",
                    tooltip: "新建/AI聊天",
                    action: ToolbarAction::Signal("show-ai-chat"),
                },
                ToolbarButton {
                    icon_name: "document-open-symbolic",
                    tooltip: "打开",
                    action: ToolbarAction::Toggle,
                },
                ToolbarButton {
                    icon_name: "document-save-symbolic",
                    tooltip: "保存",
                    action: ToolbarAction::Toggle,
                },
            ];

            // 根据配置表动态创建按钮
            for button_config in toolbar_buttons {
                let button = create_toolbar_button(button_config.icon_name, button_config.tooltip);
                buttons_ref.borrow_mut().push(button.clone());

                match button_config.action {
                    ToolbarAction::Signal(signal_name) => {
                        let obj_clone = obj.clone();
                        button.connect_clicked(move |_| {
                            obj_clone.emit_by_name::<()>(signal_name, &[]);
                        });
                    }
                    ToolbarAction::Toggle => {
                        setup_button_click_handler(&button, buttons_ref.clone());
                    }
                }

                toolbar_box.append(&button);
            }

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
        button.connect_clicked(clone!(
            #[weak]
            button,
            move |_| {
                for b in buttons.borrow().iter() {
                    if b != &button {
                        b.set_active(false);
                    }
                }

                // 确保当前按钮被选中
                button.set_active(true);
            }
        ));
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
