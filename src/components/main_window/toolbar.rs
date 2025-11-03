use adw::NavigationPage;
use adw::glib;
use adw::glib::Object;
use adw::subclass::prelude::ObjectSubclassIsExt;

mod imp {
    use super::*;
    use crate::components::main_window::toolbar_config::{TOOLBAR_BUTTONS, ToolbarAction};
    use adw::glib::clone;
    use adw::prelude::NavigationPageExt;
    use adw::prelude::{ButtonExt, ObjectExt};
    use adw::subclass::prelude::*;
    use adw::{HeaderBar, ToolbarView, glib};
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
        fn signals() -> &'static [glib::subclass::Signal] {
            use once_cell::sync::Lazy;

            static SIGNALS: Lazy<Vec<glib::subclass::Signal>> = Lazy::new(|| {
                // 收集所有需要创建的信号
                TOOLBAR_BUTTONS
                    .iter()
                    .filter_map(|button| match &button.action {
                        ToolbarAction::Signal(signal_name) => {
                            let flags = button
                                .signal_flags
                                .unwrap_or(glib::SignalFlags::RUN_LAST | glib::SignalFlags::ACTION);
                            Some(
                                glib::subclass::Signal::builder(signal_name)
                                    .flags(flags)
                                    .build(),
                            )
                        }
                        _ => None,
                    })
                    .collect()
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
            let toolbar_header = HeaderBar::builder()
                .title_widget(&Label::new(Some("")))
                .show_end_title_buttons(true)
                .build();

            // // 创建logo图像
            // let logo = gtk::Image::from_icon_name("applications-science-symbolic");
            // logo.set_icon_size(gtk::IconSize::Normal);
            // logo.set_tooltip_text(Some("Qtools"));
            //
            // // 创建一个固定大小的容器来放置logo
            // let logo_container = gtk::Box::new(Orientation::Horizontal, 0);
            // logo_container.set_size_request(48, 48);
            // logo_container.append(&logo);
            // logo_container.set_css_classes(&["toolbar-logo-container"]);
            //
            // sidebar_header.pack_start(&logo_container);

            toolbar_header.add_css_class("header-bar");

            // 创建工具栏按钮容器
            let toolbar_box = gtk::Box::builder()
                .orientation(Orientation::Vertical)
                .build();

            toolbar_box.add_css_class("toolbar-box");

            let obj = self.obj();
            let buttons_ref = &obj.imp().buttons;

            // 根据配置表动态创建按钮
            for button_config in TOOLBAR_BUTTONS.iter() {
                let button = create_toolbar_button(button_config.icon_name, button_config.tooltip);
                buttons_ref.borrow_mut().push(button.clone());

                match button_config.action {
                    ToolbarAction::Signal(signal_name) => {
                        button.connect_clicked(clone!(
                            #[weak]
                            obj,
                            #[weak]
                            button,
                            move |_| {
                                obj.imp().select_button(&button);
                                obj.emit_by_name::<()>(signal_name, &[]);
                            }
                        ));
                    }
                    ToolbarAction::Toggle => {
                        button.connect_clicked(clone!(
                            #[weak]
                            obj,
                            #[weak]
                            button,
                            move |_| {
                                obj.imp().select_button(&button);
                            }
                        ));
                    }
                }

                toolbar_box.append(&button);
            }

            // 创建侧边栏内容
            let toolbar_content = gtk::Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(0)
                .build();

            // 添加工具栏到侧边栏内容顶部
            toolbar_content.append(&toolbar_box);

            // 为侧边栏内容添加CSS类
            toolbar_content.add_css_class("main-toolbar");

            // 将侧边栏内容放入滚动视图中
            let toolbar_scroll = ScrolledWindow::builder()
                .vexpand(true)
                .hscrollbar_policy(PolicyType::Never)
                .child(&toolbar_content)
                .build();

            // 创建工具栏视图
            let toolbar_view = ToolbarView::builder().content(&toolbar_scroll).build();

            toolbar_view.add_top_bar(&toolbar_header);
            self.obj().set_child(Some(&toolbar_view));
        }
        // 统一处理按钮选择逻辑的公共方法
        fn select_button(&self, selected_button: &ToggleButton) {
            self.obj()
                .imp()
                .buttons
                .borrow_mut()
                .iter()
                .for_each(|button| {
                    if button != selected_button {
                        button.set_active(false);
                    }
                });
            selected_button.set_active(true);
        }
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
