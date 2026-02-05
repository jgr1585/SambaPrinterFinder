use std::cell::RefCell;
use std::rc::Rc;

use glib::{clone, Object};
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::gio::{ListModel, ListStore};
use gtk::{prelude::*, Align, Box, Button, Entry, Frame, Grid, Label, ListItem, ListView, Orientation, PolicyType, ScrolledWindow, SignalListItemFactory, SingleSelection, Window};
use oneshot::channel;
use crate::cups::PpdInfo;

/// Result from the printer setup dialog
#[derive(Debug, Clone)]
pub struct PrinterSetupResult {
    pub manufacturer: String,
    pub model: String,
    pub printer_name: String,
    pub description: String,
    pub location: String,
}

mod manufacturer_object {
    use glib::subclass::prelude::*;
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct ManufacturerObject {
        pub name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ManufacturerObject {
        const NAME: &'static str = "ManufacturerObject";
        type Type = super::ManufacturerObject;
    }

    impl ObjectImpl for ManufacturerObject {}
}

glib::wrapper! {
    pub struct ManufacturerObject(ObjectSubclass<manufacturer_object::ManufacturerObject>);
}

impl ManufacturerObject {
    pub fn new(name: &str) -> Self {
        let obj: Self = Object::new();
        obj.imp().name.replace(name.to_string());
        obj
    }

    pub fn name(&self) -> String {
        self.imp().name.borrow().clone()
    }

    fn imp(&self) -> &manufacturer_object::ManufacturerObject {
        ObjectSubclassIsExt::imp(self)
    }
}

mod model_object {
    use glib::subclass::prelude::*;
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct ModelObject {
        pub name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ModelObject {
        const NAME: &'static str = "PrinterModelObject";
        type Type = super::ModelObject;
    }

    impl ObjectImpl for ModelObject {}
}

glib::wrapper! {
    pub struct ModelObject(ObjectSubclass<model_object::ModelObject>);
}

impl ModelObject {
    pub fn new(name: &str) -> Self {
        let obj: Self = glib::Object::new();
        obj.imp().name.replace(name.to_string());
        obj
    }

    pub fn name(&self) -> String {
        self.imp().name.borrow().clone()
    }

    fn imp(&self) -> &model_object::ModelObject {
        ObjectSubclassIsExt::imp(self)
    }
}

fn create_list_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();

    factory.connect_setup(|_, obj| {
        let list_item = obj
            .downcast_ref::<ListItem>()
            .expect("Needs to be a ListItem");

        let label = Label::new(None);
        label.set_xalign(0.0);
        label.set_margin_start(6);
        label.set_margin_end(6);
        label.set_margin_top(4);
        label.set_margin_bottom(4);
        list_item.set_child(Some(&label));
    });

    factory
}

/// Shows the printer setup dialog and returns the user's selections.
/// 
/// # Arguments
/// * `parent` - The parent window for the dialog
/// * `manufacturers` - List of available printer manufacturers and their models
/// 
/// # Returns
/// `Some(PrinterSetupResult)` if the user confirms, `None` if cancelled
pub async fn show_printer_setup_dialog<W: IsA<Window>>(
    parent: &W,
    manufacturers: &Vec<PpdInfo>,
    printer_name: Option<String>,
) -> Option<PrinterSetupResult> {
    let dialog = Window::builder()
        .title("Printer Setup")
        .modal(true)
        .transient_for(parent)
        .default_width(600)
        .default_height(500)
        .resizable(true)
        .build();

    let main_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // --- Manufacturer and Model Selection ---
    let lists_hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .vexpand(true)
        .build();

    // Manufacturer List
    let manufacturer_frame = Frame::builder()
        .label("Manufacturer")
        .hexpand(true)
        .vexpand(true)
        .build();

    let manufacturer_store = ListStore::new::<ManufacturerObject>();
    let mut model_make = Vec::new();

    for mfr in manufacturers {
        if model_make.contains(&&mfr.make) {
            continue;
        }
        model_make.push(&mfr.make);
    }

    for make in model_make {
        manufacturer_store.append(&ManufacturerObject::new(make));
    }

    let manufacturer_factory = create_list_factory();
    manufacturer_factory.connect_bind(|_, obj| {
        let list_item = obj
            .downcast_ref::<ListItem>()
            .expect("Needs to be a ListItem");

        let item = list_item
            .item()
            .and_downcast::<ManufacturerObject>()
            .expect("Item should be ManufacturerObject");

        let label = list_item
            .child()
            .and_downcast::<Label>()
            .expect("Child should be Label");

        label.set_text(&item.name());
    });

    let manufacturer_selection = SingleSelection::new(Some(
        manufacturer_store.clone().upcast::<ListModel>(),
    ));

    let manufacturer_list = ListView::builder()
        .model(&manufacturer_selection)
        .factory(&manufacturer_factory)
        .build();

    let manufacturer_scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .child(&manufacturer_list)
        .build();

    manufacturer_frame.set_child(Some(&manufacturer_scroll));

    // Model List
    let model_frame = Frame::builder()
        .label("Model")
        .hexpand(true)
        .vexpand(true)
        .build();

    let model_store = ListStore::new::<ModelObject>();

    let model_factory = create_list_factory();
    model_factory.connect_bind(|_, obj| {
        let list_item = obj
            .downcast_ref::<ListItem>()
            .expect("Needs to be a ListItem");

        let item = list_item
            .item()
            .and_downcast::<ModelObject>()
            .expect("Item should be ModelObject");

        let label = list_item
            .child()
            .and_downcast::<Label>()
            .expect("Child should be Label");

        label.set_text(&item.name());
    });

    let model_selection = SingleSelection::new(Some(model_store.clone().upcast::<ListModel>()));

    let model_list = ListView::builder()
        .model(&model_selection)
        .factory(&model_factory)
        .build();

    let model_scroll = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Automatic)
        .child(&model_list)
        .build();

    model_frame.set_child(Some(&model_scroll));

    lists_hbox.append(&manufacturer_frame);
    lists_hbox.append(&model_frame);

    // --- Printer Name and Location ---
    let details_frame = Frame::builder()
        .label("Printer Details")
        .build();

    let details_grid = Grid::builder()
        .row_spacing(6)
        .column_spacing(12)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .build();

    let name_label = Label::builder()
        .label("Printer Name:")
        .halign(Align::Start)
        .build();

    let name_entry = Entry::builder()
        .placeholder_text("Enter printer name")
        .text(printer_name.as_deref().unwrap_or(""))
        .hexpand(true)
        .build();

    let description_label = Label::builder()
        .label("Description:")
        .halign(Align::Start)
        .build();

    let description_entry = Entry::builder()
        .placeholder_text("Short description of the printer (optional)")
        .hexpand(true)
        .build();

    let location_label = Label::builder()
        .label("Location:")
        .halign(Align::Start)
        .build();

    let location_entry = Entry::builder()
        .placeholder_text("Enter printer location (e.g., Office Room 101)")
        .hexpand(true)
        .build();

    details_grid.attach(&name_label, 0, 0, 1, 1);
    details_grid.attach(&name_entry, 1, 0, 1, 1);
    details_grid.attach(&description_label, 0, 1, 1, 1);
    details_grid.attach(&description_entry, 1, 1, 1, 1);
    details_grid.attach(&location_label, 0, 2, 1, 1);
    details_grid.attach(&location_entry, 1, 2, 1, 1);

    details_frame.set_child(Some(&details_grid));

    // --- Buttons ---
    let buttons_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::End)
        .spacing(6)
        .build();

    let cancel_button = Button::with_label("Cancel");
    let confirm_button = Button::with_label("Confirm");
    confirm_button.add_css_class("suggested-action");

    buttons_box.append(&cancel_button);
    buttons_box.append(&confirm_button);

    // Assemble main layout
    main_vbox.append(&lists_hbox);
    main_vbox.append(&details_frame);
    main_vbox.append(&buttons_box);

    dialog.set_child(Some(&main_vbox));

    // --- State for selected values ---
    let selected_manufacturer: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let selected_model: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));

    // Update models when manufacturer selection changes
    let manufacturers_clone = manufacturers.clone();
    let model_store_clone = model_store.clone();
    let selected_manufacturer_clone = selected_manufacturer.clone();

    manufacturer_selection.connect_selection_changed(move |selection, _, _| {
        if let Some(selected) = selection.selected_item() {
            if let Some(mfr_obj) = selected.downcast_ref::<ManufacturerObject>() {
                let mfr_name = mfr_obj.name();
                *selected_manufacturer_clone.borrow_mut() = Some(mfr_name.clone());

                // Find the manufacturer and populate models
                if let Some(mfr) = manufacturers_clone.iter().find(|m| m.make == mfr_name) {
                    model_store_clone.remove_all();
                    for model in &manufacturers_clone {
                        if model.make.eq(&mfr.make) {
                            model_store_clone.append(&ModelObject::new(&model.make_and_model));
                        }
                    }
                }
            }
        }
    });

    // Track model selection
    let selected_model_clone = selected_model.clone();
    model_selection.connect_selection_changed(move |selection, _, _| {
        if let Some(selected) = selection.selected_item() {
            if let Some(model_obj) = selected.downcast_ref::<ModelObject>() {
                *selected_model_clone.borrow_mut() = Some(model_obj.name());
            }
        }
    });

    // Select first manufacturer by default
    if manufacturer_store.n_items() > 0 {
        manufacturer_selection.set_selected(0);
    }

    dialog.set_visible(true);

    // --- Async result handling ---
    let (tx, rx) = channel::<Option<PrinterSetupResult>>();
    let tx = Rc::new(RefCell::new(Some(tx)));

    cancel_button.connect_clicked(clone!(
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

    let selected_manufacturer_final = selected_manufacturer.clone();
    let selected_model_final = selected_model.clone();

    confirm_button.connect_clicked(clone!(
        #[weak]
        dialog,
        #[strong]
        tx,
        move |_| {
            if let Some(sender) = tx.borrow_mut().take() {
                let manufacturer = selected_manufacturer_final
                    .borrow()
                    .clone()
                    .unwrap_or_default();
                let model = selected_model_final.borrow().clone().unwrap_or_default();
                let printer_name = name_entry.text().to_string();
                let description = description_entry.text().to_string();
                let location = location_entry.text().to_string();

                let result = PrinterSetupResult {
                    manufacturer,
                    model,
                    printer_name,
                    description,
                    location,
                };

                let _ = sender.send(Some(result));
            }
            dialog.close();
        }
    ));

    // Wait for result
    rx.await.unwrap_or_else(|_| None)
}
