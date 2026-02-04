mod samba_entry_object;
mod smb_login_dialog;
pub mod printer_setup_dialog;

use std::cell::RefCell;
use std::rc::Rc;
use crate::gui::samba_entry_object::SambaEntryObject;
use crate::smb::{SambaConnection, SambaEntryType};
use glib::{clone, MainContext, Propagation};
use gtk::gio::{ListModel, ListStore};
use gtk::{prelude::*, Align, Application, ApplicationWindow, Box, Button, GestureClick, Label, ListItem, ListView, NoSelection, Orientation, ScrolledWindow, SignalListItemFactory};
use crate::cups::{CupsManager, PpdInfo};
use crate::gui::printer_setup_dialog::show_printer_setup_dialog;


pub fn build_ui(application: &Application) {
    let factory = SignalListItemFactory::new();
    let smb_state: Rc<RefCell<Option<Rc<SambaConnection>>>> = Rc::new(RefCell::new(None));
    let cups_manager = CupsManager::new();

    let list_store = ListStore::new::<SambaEntryObject>();
    let no_selection = NoSelection::new(Some(list_store.clone().upcast::<ListModel>()));
    let app_window_holder: Rc<RefCell<Option<ApplicationWindow>>> = Rc::new(RefCell::new(None));


    factory.connect_setup({
        let smb_state = smb_state.clone();
        let list_store = list_store.clone();
        let app_window_holder = app_window_holder.clone();
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
            let app_window_holder_cl = app_window_holder.clone();
            let cups_manager = cups_manager.clone();

            gesture.connect_pressed(move |_gesture, n_press, _x, _y| {
                if n_press >= 1 {
                    if let Some(entry) = li.item().and_downcast::<SambaEntryObject>() {
                        let server = entry.server_path();
                        let server = match server {
                            Some(s) => s,
                            None => {
                                eprintln!("No server path found for this entry");
                                return;
                            }
                        };

                        match entry.entry_type() {
                            SambaEntryType::Directory => {
                                if let Some(conn_rc) = smb_state_cl.borrow().as_ref() {
                                    match conn_rc.list_directory(&server) {
                                        Ok(entries) => {
                                            list_store_cl.remove_all();
                                            for entry in entries {
                                                let server_path = server
                                                    .join(&entry.name)
                                                    .expect("Failed to join paths {} and {}");

                                                let obj = SambaEntryObject::new(&entry, &server_path);
                                                list_store_cl.append(&obj);
                                            }
                                        }
                                        Err(e) => eprintln!("Error listing directory: {}", e),
                                    }
                                }
                            }
                            SambaEntryType::Printer => {
                                // Spawn the printer setup dialog on the main context

                                let holder = app_window_holder_cl.clone();
                                let cups_manager = cups_manager.clone();
                                let smb_state_cl = smb_state_cl.clone();

                                MainContext::default().spawn_local(async move {
                                    if let Some(parent) = holder.borrow().as_ref() {
                                        if let Some(result) = show_printer_setup_dialog(parent, &cups_manager.ppds, Option::from(entry.name())).await {
                                            println!("Chosen: {} {} {} {}", result.manufacturer, result.model, result.printer_name, result.location);
                                            let mut ppd_file: Option<&PpdInfo> = None;

                                            for ppd in &cups_manager.ppds {
                                                if ppd.make == result.manufacturer && ppd.product == result.model {
                                                    ppd_file = Some(ppd);
                                                    break;
                                                }
                                            }

                                            cups_manager.connect_to_printer(
                                                smb_state_cl
                                                    .borrow()
                                                    .as_ref()
                                                    .expect("Samba connection should be established")
                                                    .credentials
                                                    .clone(),
                                                &server,
                                                &result,
                                                ppd_file
                                            );
                                        }
                                    }
                                });
                            }

                            _ => {}
                        }
                    } else {
                        println!("No model item found for this row");
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

    // store window so asynchronous closures can access it later
    *app_window_holder.borrow_mut() = Some(window.clone());

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