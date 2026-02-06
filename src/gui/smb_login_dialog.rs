use std::cell::RefCell;
use std::clone::Clone;
use std::ops::Deref;
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
use url::Url;

// This is the
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

    let server = Entry::builder()
        .placeholder_text("Server address")
        .build();
    let username = Entry::builder()
        .build();
    let password = PasswordEntry::builder()
        .build();
    let domain = Entry::builder()
        .placeholder_text("WORKGROUP (optional)")
        .build();

    // Pre-fill fields if smb_state has existing connection
    if let Some(conn) = &smb_state.borrow().deref() {
        server.set_text(&conn.server_root);
        username.set_text(&conn.credentials.username);
        password.set_text(&conn.credentials.password);
        domain.set_text(&conn.credentials.workgroup);
    }

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

        // Attempt to connect to SMB server with provided credentials
        match SambaConnection::connect(creds, &server) {
            Ok(conn) => {

                if !server.starts_with("smb://") {
                    server = format!("smb://{}", server);
                }

                let server_url = Url::parse(&server).expect("Invalid SMB URL");

                let conn_rc = Rc::new(conn);
                *smb_state.borrow_mut() = Some(conn_rc.clone());

                // List the root directory of the SMB server and populate the list store
                match conn_rc.list_directory(&server_url) {
                    Ok(entries) => {
                        for entry in entries {
                            let sever_url = server_url.join(&entry.name).ok();
                            if sever_url.is_some() {
                                let obj = SambaEntryObject::new(&entry, sever_url.as_ref().unwrap());
                                list_store.append(&obj);
                            }
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
