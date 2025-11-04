use adw::glib::Object;
use adw::{glib, NavigationPage};

mod imp {
    use super::*;
    use adw::gdk::cairo;
    use adw::prelude::NavigationPageExt;
    use adw::subclass::prelude::{
        NavigationPageImpl, ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt,
    };
    use gtk::prelude::{BoxExt, DrawingAreaExtManual};
    use gtk::subclass::prelude::WidgetImpl;
    use gtk::{DrawingArea, Orientation};
    use plotters::prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, LineSeries, RED, WHITE};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Default)]
    pub struct ScreenShot {
        drawing_area: RefCell<Option<DrawingArea>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ScreenShot {
        const NAME: &'static str = "QtoolsScreenShot";
        type Type = super::ScreenShot;
        type ParentType = NavigationPage;
    }

    impl ObjectImpl for ScreenShot {
        fn constructed(&self) {
            self.parent_constructed();
            self.build_ui();
        }
    }

    impl ScreenShot {
        fn build_ui(&self) {
            let content_box = gtk::Box::builder()
                .orientation(Orientation::Vertical)
                .build();

            let image_data: Rc<RefCell<Option<Vec<u8>>>> = Rc::new(RefCell::new(None));

            // 预先生成图表位图
            let width = 600;
            let height = 400;
            let mut buf = vec![0u8; (width * height * 4) as usize]; // RGBA格式，每个像素4字节
            {
                let backend = BitMapBackend::with_buffer(&mut buf, (width, height));
                let root = backend.into_drawing_area();
                root.fill(&WHITE).unwrap();
                let mut chart = ChartBuilder::on(&root)
                    .caption("y = sin(x)", ("sans-serif", 20))
                    .margin(10)
                    .x_label_area_size(30)
                    .y_label_area_size(30)
                    .build_cartesian_2d(-std::f64::consts::PI..std::f64::consts::PI, -1.2..1.2)
                    .unwrap();
                chart.configure_mesh().draw().unwrap();
                let data: Vec<_> = (-100..=100)
                    .map(|x| {
                        let x = x as f64 * std::f64::consts::PI / 100.0;
                        (x, x.sin())
                    })
                    .collect();
                chart.draw_series(LineSeries::new(data, &RED)).unwrap();
            }
            *image_data.borrow_mut() = Some(buf);

            // 创建 DrawingArea

            let drawing_area = DrawingArea::builder()
                .content_width(width as i32)
                .content_height(height as i32)
                .build();

            self.drawing_area.replace(Some(drawing_area.clone()));

            // 克隆引用用于闭包

            let image_data_clone = image_data.clone();

            self.drawing_area.borrow().as_ref().unwrap().set_draw_func(
                move |_da, cr, width, height| {
                    if let Some(data) = &*image_data_clone.borrow() {
                        // 创建 Cairo ImageSurface（ARGB32格式，每个像素4字节）
                        let surface = cairo::ImageSurface::create_for_data(
                            data.clone(),
                            cairo::Format::ARgb32, // 使用Argb32格式
                            width,
                            height,
                            width * 4, // 步长为width * 4，因为每个像素4字节
                        )
                        .expect("Failed to create image surface");

                        // 将表面绘制到 Cairo 上下文

                        cr.set_source_surface(&surface, 0.0, 0.0).unwrap();

                        cr.paint().unwrap();
                    }
                },
            );
            content_box.append(&drawing_area);
            self.obj().set_child(Some(&content_box))
        }
    }

    impl WidgetImpl for ScreenShot {}
    impl NavigationPageImpl for ScreenShot {}
}

glib::wrapper! {
    pub struct ScreenShot(ObjectSubclass<imp::ScreenShot>)
        @extends NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ScreenShot {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn take_screenshot(&self) {
        // 实现截图逻辑
    }
}
