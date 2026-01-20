mod samba_entry_object;
mod smb_login_dialog;

use std::cell::RefCell;
use std::clone::Clone;
use std::option::Option;
use std::option::Option::{None, Some};
use std::prelude::v1::{Err, Ok};
use std::rc::Rc;

use crate::gui::samba_entry_object::SambaEntryObject;
use crate::smb::{SambaConnection, SambaEntryType};
use glib::{clone, MainContext, Propagation};
use gtk::gio::{ListModel, ListStore};
use gtk::{prelude::*, Align, Application, ApplicationWindow, Box, Button, GestureClick, Label, ListItem, ListView, NoSelection, Orientation, ScrolledWindow, SignalListItemFactory};
use crate::cups::connect_to_printer;

pub fn build_ui(application: &Application) {
    let factory = SignalListItemFactory::new();
    let smb_state: Rc<RefCell<Option<Rc<SambaConnection>>>> = Rc::new(RefCell::new(None));

    let list_store = ListStore::new::<SambaEntryObject>();
    let no_selection = NoSelection::new(Some(list_store.clone().upcast::<ListModel>()));


    factory.connect_setup({
        let smb_state = smb_state.clone();
        let list_store = list_store.clone();
        move |_, obj| {
            let list_item = obj
                .downcast_ref::<ListItem>()
                .expect("Needs to be a ListItem");

            let label = Label::new(None);
            label.set_xalign(0.0);

            let gesture = GestureClick::new();

            let li = list_item.clone();
            let smb_state_cl = smb_state.clone();
            let list_store_cl = list_store.clone();
            gesture.connect_pressed(move |_gesture, n_press, _x, _y| {
                if n_press == 2 {
                    if let Some(entry) = li.item().and_downcast::<SambaEntryObject>() {
                        let server = entry.server_path();
                        println!("Clicked entry: {}", server);

                        match entry.entry_type() {
                            SambaEntryType::Directory => {
                                if let Some(conn_rc) = smb_state_cl.borrow().as_ref() {
                                    match conn_rc.list_directory(&server) {
                                        Ok(entries) => {
                                            list_store_cl.remove_all();
                                            for entry in entries {
                                                let server_path = format!("{server}/{}", &entry.name);
                                                let obj = SambaEntryObject::new(entry, server_path);
                                                list_store_cl.append(&obj);
                                            }
                                        }
                                        Err(e) => eprintln!("Error listing directory: {}", e),
                                    }
                                }
                            }
                            SambaEntryType::Printer => {
                                connect_to_printer(
                                    smb_state_cl
                                        .borrow()
                                        .as_ref()
                                        .expect("Samba connection should be established")
                                        .credentials
                                        .clone(),
                                    server,
                                    "printer".to_string(),
                                );
                            }

                            _ => {}
                        }
                    } else {
                        println!("Kein Model-Item gefunden für diese Zeile");
                    }
                }
            });

            label.add_controller(gesture);
            list_item.set_child(Some(&label));
        }
    });

    factory.connect_bind(|_, obj| {
        let list_item = obj
            .downcast_ref::<ListItem>()
            .expect("Needs to be a ListItem");

        let entry = list_item
            .item()
            .and_downcast::<SambaEntryObject>()
            .expect("Needs to be SambaEntryObject");

        let label = list_item
            .child()
            .unwrap()
            .downcast::<Label>()
            .unwrap();

        let icon = match entry.entry_type() {
            SambaEntryType::Directory => "📁",
            SambaEntryType::File => "📄",
            SambaEntryType::Printer => "🖨️",
            SambaEntryType::Unknown => "❓",
        };

        label.set_text(&format!("{} {}", icon, entry.name()));
    });

    let list_view = ListView::new(
        Some(no_selection),
        Some(factory),
    );

    let scrolled = ScrolledWindow::builder()
        .child(&list_view)
        .vexpand(true)
        .build();

    let connect_button = Button::builder()
        .label("Connect to SMB")
        .halign(Align::Center)
        .valign(Align::Center)
        .build();

    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(6)
        .margin_end(6)
        .build();

    vbox.append(&connect_button);
    vbox.append(&scrolled);

    // ---- Window ----
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Samba Browser")
        .default_width(500)
        .default_height(400)
        .child(&vbox)
        .visible(true)
        .build();

    connect_button.connect_clicked(clone!(
            #[weak]
            window,
            move |_| {
                MainContext::default()
                    .spawn_local(smb_login_dialog::show_dialog(window.clone(), list_store.clone(), smb_state.clone()));
            }
        ));

    window.connect_close_request(|window| {
        if let Some(app) = window.application() {
            app.remove_window(window);
        }
        Propagation::Proceed
    });

    connect_button.emit_clicked();
}