use gtk::prelude::*;
use samba_printer_finder::gui::build_ui;

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("at.jeb.samba_printer_finder")
        .build();

    application.connect_activate(build_ui);
    application.run()
}