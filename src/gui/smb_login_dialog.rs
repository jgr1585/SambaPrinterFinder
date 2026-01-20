use std::cell::RefCell;
use std::clone::Clone;
use std::option::Option;
use std::option::Option::{None, Some};
use std::prelude::v1::{Err, Ok};
use std::rc::Rc;
use std::string::String;

use crate::gui::samba_entry_object::SambaEntryObject;
use crate::smb::{SambaConnection, SambaCredentials};
use glib::clone;
use gtk::gio::ListStore;
use gtk::{prelude::*, Align, Button, Entry, Grid, Label, Orientation, PasswordEntry, Window};
use oneshot::channel;

pub async fn show_dialog<W: IsA<Window>>(parent: W, list_store: ListStore, smb_state: Rc<RefCell<Option<Rc<SambaConnection>>>>) {
    let dialog = Window::builder()
        .title("SMB Authentication")
        .modal(true)
        .transient_for(&parent)
        .default_width(360)
        .resizable(false)
        .build();

    let vbox = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let grid = Grid::builder()
        .row_spacing(6)
        .column_spacing(6)
        .build();

    let credentials = &smb_state.borrow().as_ref().cloned();
    let credentials = match credentials {
        Some(conn) => conn.credentials.clone(),
        None => SambaCredentials {
            workgroup: String::new(),
            username: String::new(),
            password: String::new(),
        },
    };

    let server = Entry::builder()
        .placeholder_text("Server address")

        .build();
    let username = Entry::builder()
        .text(credentials.username.clone())
        .build();
    let password = PasswordEntry::builder()
        .text(credentials.password.clone())
        .build();
    let domain = Entry::builder()
        .placeholder_text("WORKGROUP (optional)")
        .text(credentials.workgroup.clone())
        .build();

    grid.attach(&Label::new(Some("Server:")), 0, 0, 1, 1);
    grid.attach(&server, 1, 0, 1, 1);

    grid.attach(&Label::new(Some("Username:")), 0, 1, 1, 1);
    grid.attach(&username, 1, 1, 1, 1);

    grid.attach(&Label::new(Some("Password:")), 0, 2, 1, 1);
    grid.attach(&password, 1, 2, 1, 1);

    grid.attach(&Label::new(Some("Domain:")), 0, 3, 1, 1);
    grid.attach(&domain, 1, 3, 1, 1);

    let buttons = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::End)
        .spacing(6)
        .build();

    let cancel = Button::with_label("Cancel");
    let login = Button::with_label("Login");

    buttons.append(&cancel);
    buttons.append(&login);

    vbox.append(&grid);
    vbox.append(&buttons);
    dialog.set_child(Some(&vbox));
    dialog.set_visible(true);

    // ---- async result ----
    let (tx, rx) = channel::<Option<(String, String, String, String)>>();
    let tx = Rc::new(RefCell::new(Some(tx)));

    // Cancel button
    cancel.connect_clicked(clone!(
        #[weak]
        dialog,
        #[strong]
        tx,
        move |_| {
            if let Some(sender) = tx.borrow_mut().take() {
                let _ = sender.send(None);
            }
            dialog.close();
        }
    ));

    login.connect_clicked(clone!(
            #[weak]
            dialog,
            move |_| {
                if let Some(sender) = tx.borrow_mut().take() {
                    let creds = (
                        server.text().to_string(),
                        username.text().to_string(),
                        password.text().to_string(),
                        domain.text().to_string(),
                    );
                    let _ = sender.send(Some(creds));
                }
                dialog.close();
            }
        ));

    // ---- await result ----
    if let Ok(Some((server, user, pass, domain))) = rx.await {
        let mut server = server;

        let creds = SambaCredentials {
            workgroup: domain,
            username: user,
            password: pass,
        };


        if !server.starts_with("smb://") {
            server = format!("smb://{}", server);
        }

        match SambaConnection::connect(creds) {
            Ok(conn) => {

                let conn_rc = Rc::new(conn);
                *smb_state.borrow_mut() = Some(conn_rc.clone());

                match conn_rc.list_directory(&server) {
                    Ok(entries) => {
                        for entry in entries {
                            let server_path = format!("{server}/{}", &entry.name);
                            let obj = SambaEntryObject::new(entry, server_path);
                            list_store.append(&obj);
                        }
                    }
                    Err(e) => eprintln!("Error listing directory: {}", e),
                }
            }
            Err(e) => {
                eprintln!("Error connecting to SMB: {}", e);
            }
        }
    }
}
