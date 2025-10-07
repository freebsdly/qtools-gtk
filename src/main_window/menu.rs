use gtk::{MenuButton, PopoverMenu, gio};

pub struct AppMenu {
    pub menu_button: MenuButton,
}

impl AppMenu {
    pub fn new() -> Self {
        // 创建菜单模型
        let menu_model = gio::Menu::new();

        // 添加菜单项
        let file_section = gio::Menu::new();
        file_section.append(Some("新建"), Some("app.new"));
        file_section.append(Some("打开"), Some("app.open"));
        file_section.append(Some("保存"), Some("app.save"));

        let edit_section = gio::Menu::new();
        edit_section.append(Some("首选项"), Some("app.preferences"));

        let help_section = gio::Menu::new();
        help_section.append(Some("关于"), Some("app.about"));
        help_section.append(Some("退出"), Some("app.quit"));

        menu_model.append_section(None, &file_section);
        menu_model.append_section(None, &edit_section);
        menu_model.append_section(None, &help_section);

        // 创建弹出菜单
        let popover = PopoverMenu::builder().menu_model(&menu_model).build();

        // 创建菜单按钮
        let menu_button = MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .popover(&popover)
            .build();

        Self { menu_button }
    }
}
