use std::clone::Clone;
use std::string::{String, ToString};
use glib::subclass::prelude::ObjectSubclassIsExt;
use crate::smb::{SambaDirectoryEntry, SambaEntryType};

mod imp {
    use gtk::glib::subclass::prelude::*;
    use std::cell::RefCell;
    use std::string::String;
    use crate::smb::SambaEntryType;

    #[derive(Default)]
    pub struct SambaEntryObject {
        pub name: RefCell<String>,
        pub entry_type: RefCell<SambaEntryType>,
        pub server_path: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SambaEntryObject {
        const NAME: &'static str = "SambaEntryObject";
        type Type = super::SambaEntryObject;
    }

    impl ObjectImpl for SambaEntryObject {}
}

glib::wrapper! {
    pub struct SambaEntryObject(ObjectSubclass<imp::SambaEntryObject>);
}

impl SambaEntryObject {
    pub fn new(entry: SambaDirectoryEntry, server_path: String) -> Self {
        let obj: Self = glib::Object::new();
        obj.imp().name.replace(entry.name);
        obj.imp().entry_type.replace(entry.entry_type);
        obj.imp().server_path.replace(server_path);
        obj
    }

    pub fn name(&self) -> String {
        self.imp().name.borrow().to_string()
    }

    pub fn entry_type(&self) -> SambaEntryType {
        (*self.imp().entry_type.borrow()).clone()
    }

    pub fn server_path(&self) -> String {
        self.imp().server_path.borrow().to_string()
    }
}