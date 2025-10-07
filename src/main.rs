#![windows_subsystem = "windows"]

mod main_window;

use adw::prelude::*;
use adw::Application;
use gtk::glib;
use gtk::{gdk, CssProvider};

const APP_ID: &str = "top.qinhuajun.app";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(|app| {
        setup_shortcuts(app);
        load_css();
    });
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let main_window = main_window::MainWindow::new(app);
    // Setup actions with window reference
    main_window.setup_actions(app);
    main_window.present();
    

}

fn setup_shortcuts(app: &Application) {
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
