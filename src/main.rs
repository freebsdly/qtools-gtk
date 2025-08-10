#![windows_subsystem = "windows"]

mod main_window;

use adw::prelude::*;
use gtk::{gio, glib};
use std::env;
use adw::Application;
use crate::main_window::Window;

const APP_ID: &str = "top.qinhuajun.app";
const SCHEMA_DIR_KEY: &str = "GSETTINGS_SCHEMA_DIR";

fn main() -> glib::ExitCode {
    setup_gsettings_schema_dir();

    gio::resources_register_include!("app.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(setup_shortcuts);
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a new custom window and present it
    let window = Window::new(app);
    window.present();
}

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}

fn setup_gsettings_schema_dir() {
    let possible_paths = [
        "resources",                   // 相对于可执行文件的resources目录
        "../resources",                // 开发时的resources目录
        "./schemas",                   // 常见的schemas目录
        "../schemas",                  // 开发时的schemas目录
        "/usr/share/glib-2.0/schemas", // 系统默认路径
    ];

    for path in &possible_paths {
        if std::path::Path::new(path).exists() {
            unsafe {
                env::set_var(SCHEMA_DIR_KEY, path);
            }
            println!("Set GSETTINGS_SCHEMA_DIR to: {}", path);
            return;
        }
    }

    // 如果没有找到合适的路径，使用当前目录
    unsafe {
        env::set_var(SCHEMA_DIR_KEY, ".");
    }
    println!("Set GSETTINGS_SCHEMA_DIR to current directory");
}