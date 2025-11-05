use adw::glib::Object;
use adw::{glib, NavigationPage};

mod imp {
    use super::*;
    use adw::glib::clone;
    use adw::prelude::ListBoxRowExt;
    use adw::prelude::NavigationPageExt;
    use adw::subclass::prelude::{
        NavigationPageImpl, ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt,
    };
    use adw::Clamp;
    use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
    use gtk::subclass::prelude::WidgetImpl;
    use gtk::{Box, Button, Label, ListBox, ListBoxRow, ScrolledWindow, SelectionMode};

    #[derive(Default)]
    pub struct EntityList {}

    #[glib::object_subclass]
    impl ObjectSubclass for EntityList {
        const NAME: &'static str = "QtoolsEntityList";
        type Type = super::EntityList;
        type ParentType = NavigationPage;
    }

    impl ObjectImpl for EntityList {
        fn constructed(&self) {
            self.parent_constructed();
            self.build_ui();
        }
    }

    impl EntityList {
        fn build_ui(&self) {
            let list_box = ListBox::new();
            list_box.set_selection_mode(SelectionMode::Single);
            list_box.add_css_class("boxed-list");

            // 添加示例列表项（循环创建 20 个项）
            for i in 1..=80 {
                let row = self.create_list_item(&format!("列表项 {}", i));
                list_box.append(&row);
            }

            let clamp = Clamp::builder()
                .maximum_size(400)
                .margin_top(16)
                .margin_bottom(16)
                .margin_start(16)
                .margin_end(16)
                .tightening_threshold(300)
                .build();
            clamp.set_child(Some(&list_box));

            let scrolled_window = ScrolledWindow::builder()
                .vexpand(true)
                .hexpand(true)
                .min_content_height(400)
                .build();

            scrolled_window.set_child(Some(&clamp));

            self.obj().set_child(Some(&scrolled_window));
        }

        /// 辅助函数：创建单个列表项（ListBoxRow）
        fn create_list_item(&self, text: &str) -> ListBoxRow {
            // 列表项内部布局：水平 Box（左侧文本 + 右侧按钮）
            let row_box = Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .margin_bottom(12)
                .margin_top(12)
                .margin_start(12)
                .margin_end(12)
                .build();

            // 文本标签
            let label = Label::new(Some(text));
            label.set_hexpand(true); // 文本占满水平剩余空间（推按钮到右侧）

            // 示例按钮（可替换为其他组件）
            let button = Button::with_label("操作");
            button.connect_clicked(clone!(
                #[weak]
                label,
                move |_| {
                    println!("点击了「{}」的操作按钮", label.text());
                }
            ));

            // 组装列表项
            row_box.append(&label);
            row_box.append(&button);

            // 包装为 ListBoxRow（ListBox 只能接收 ListBoxRow 作为子组件）
            let row = ListBoxRow::new();
            row.set_child(Some(&row_box));

            row
        }
    }

    impl WidgetImpl for EntityList {}
    impl NavigationPageImpl for EntityList {}
}

glib::wrapper! {
    pub struct EntityList(ObjectSubclass<imp::EntityList>)
        @extends NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl EntityList {
    pub fn new() -> Self {
        Object::builder().property("title", "Entity List").build()
    }
}
