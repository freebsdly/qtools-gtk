use adw::prelude::*;
use adw::{AboutDialog, ApplicationWindow};

pub struct AppAboutDialog;

impl AppAboutDialog {
    pub fn show(app: &ApplicationWindow) {
        let about_dialog = AboutDialog::builder()
            .application_name("QTools")
            .application_icon("applications-development")
            .developer_name("Qinhuajun")
            .version("0.1.0")
            .comments("一个基于 Rust 和 GTK 的实用工具集")
            .website("https://github.com/qinhuajun/qtools")
            .issue_url("https://github.com/qinhuajun/qtools/issues")
            .developers(vec!["Qinhuajun https://github.com/qinhuajun"])
            .copyright("© 2025 Qinhuajun")
            .license_type(gtk::License::MitX11)
            .build();
        
        about_dialog.present(Some(app));
    }
}