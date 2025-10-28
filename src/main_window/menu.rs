use adw::glib;
use adw::glib::Object;
use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{Box, MenuButton};

mod imp {
    use adw::subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt};
    use adw::{gio, glib};
    use gtk::prelude::BoxExt;
    use gtk::subclass::prelude::{BoxImpl, WidgetImpl};
    use gtk::{Box, MenuButton, PopoverMenu};

    #[derive(Default)]
    pub struct AppMenu {
        pub menu_button: MenuButton,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AppMenu {
        const NAME: &'static str = "QtoolsMenuButton";
        type Type = super::AppMenu;
        type ParentType = Box; // 改为继承 Box，因为 MenuButton 不可子类化
    }

    impl ObjectImpl for AppMenu {
        fn constructed(&self) {
            self.parent_constructed();

            // 创建菜单模型
            let menu_model = gio::Menu::new();

            // 添加菜单项 - 使用 app. 前缀引用应用程序级别的动作
            let file_section = gio::Menu::new();
            file_section.append(Some("新建"), Some("win.new"));
            file_section.append(Some("打开"), Some("win.open"));
            file_section.append(Some("保存"), Some("win.save"));

            let edit_section = gio::Menu::new();
            // preferences 现在是窗口级别动作
            edit_section.append(Some("首选项"), Some("win.preferences"));

            let help_section = gio::Menu::new();
            // 改为使用 app. 前缀引用应用程序级别的动作
            help_section.append(Some("关于"), Some("app.about"));
            help_section.append(Some("退出"), Some("app.quit"));

            menu_model.append_section(None, &file_section);
            menu_model.append_section(None, &edit_section);
            menu_model.append_section(None, &help_section);

            // 创建弹出菜单
            let popover = PopoverMenu::builder().menu_model(&menu_model).build();

            // 创建菜单按钮
            self.menu_button.set_icon_name("open-menu-symbolic");
            self.menu_button.set_popover(Some(&popover));

            // 将菜单按钮添加到自身（因为继承自 Box）
            self.obj().append(&self.menu_button);
        }
    }

    impl WidgetImpl for AppMenu {}

    // 添加 BoxImpl 实现
    impl BoxImpl for AppMenu {}
}

glib::wrapper! {
    pub struct AppMenu(ObjectSubclass<imp::AppMenu>)
        @extends Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl AppMenu {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn menu_button(&self) -> &MenuButton {
        &self.imp().menu_button
    }
}