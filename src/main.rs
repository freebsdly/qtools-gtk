#![windows_subsystem = "windows"]

mod app;
mod components;

use adw::prelude::*;
use app::QtoolsApplication;
use components::tutorial::TutorialDialog;
use gtk::glib;
use gtk::{gdk, CssProvider};

const APP_ID: &str = "top.qinhuajun.app";

fn main() -> glib::ExitCode {
    pretty_env_logger::init();

    let app = QtoolsApplication::new(APP_ID);

    // Connect to signals
    app.connect_startup(|app| {
        load_css();
    });
    app.connect_activate(|app| {
        // 创建并显示引导页
        let tutorial_dialog = TutorialDialog::new();

        // 创建主窗口
        let main_window = components::main_window::MainWindow::new(app);
        main_window.present();

        // 在主窗口上显示引导页模态框
        AdwDialogExt::present(&tutorial_dialog, Some(&main_window));
    });
    app.run()
}

fn load_css() {
    // Load the CSS file
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
