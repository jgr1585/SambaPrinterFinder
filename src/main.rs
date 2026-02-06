use std::path::Path;
use gtk::gdk::Display;
use gtk::{Application, IconTheme, Window};
use gtk::prelude::*;
use samba_printer_finder::gui::build_ui;

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("io.github.jgr1585.SambaPrinterFinder")
        .build();

    // Set default application icon
    application.connect_startup(|_app| {
        if let Some(display) = Display::default() {
            let icon_theme = IconTheme::for_display(&display);
            let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");

            if let Some(path) = icon_path.to_str() {
                icon_theme.add_search_path(path);
                Window::set_default_icon_name("icon");
            }
        }
    });

    // Build the UI when the application is activated
    application.connect_activate(build_ui);
    application.run()
}