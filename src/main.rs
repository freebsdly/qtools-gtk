#![windows_subsystem = "windows"]

mod ai_chat;
mod app;
mod main_window;

use adw::prelude::*;
use app::QtoolsApplication;
use gtk::glib;
use gtk::{CssProvider, gdk};

const APP_ID: &str = "top.qinhuajun.app";

fn main() -> glib::ExitCode {
    let app = QtoolsApplication::new(APP_ID);

    // Connect to signals
    app.connect_startup(|app| {
        setup_shortcuts(app);
        load_css();
    });
    app.connect_activate(|app| {
        let main_window = main_window::MainWindow::new(app);
        main_window.present();
    });
    app.run()
}

fn setup_shortcuts(app: &QtoolsApplication) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
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
