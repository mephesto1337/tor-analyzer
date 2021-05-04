use std::rc::Rc;

use gio::prelude::*;
use gtk::prelude::*;

use tor_analyzer_lib::prelude::*;

#[derive(Debug)]
struct SimpleCircuit {
    endpoint: Target,
    or: OnionRouter,
}

#[derive(Debug)]
#[repr(i32)]
enum Columns {
    EndPoint,
    Country,
    Path,
    MaxColumns,
}
const FIELD_COUNT: usize = Columns::MaxColumns as usize;

fn update_model(model: &gtk::ListStore) {
    let _ = model;
}

pub fn create_tab() -> gtk::Widget {
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);

    let sw = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    sw.set_shadow_type(gtk::ShadowType::EtchedIn);
    sw.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    vbox.add(&sw);

    let col_types = [glib::Type::String; FIELD_COUNT];
    let store = Rc::new(gtk::ListStore::new(&col_types));

    let treeview = gtk::TreeView::with_model(&*store.clone());
    treeview.set_vexpand(true);
    treeview.set_search_column(Columns::Country as i32);

    sw.add(&treeview);
    add_column!(treeview, Columns::EndPoint, "End point");
    add_column!(treeview, Columns::Country, "Country");
    add_column!(treeview, Columns::Path, "Name");

    let update_btn = gtk::Button::with_label("Update");
    update_btn.connect_clicked(move |_| {
        let store = store.clone();
        update_model(&*store);
    });
    vbox.add(&update_btn);

    // Fill table
    update_btn.clicked();

    vbox.upcast()
}
