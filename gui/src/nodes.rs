use std::rc::Rc;

use gtk::prelude::*;

use tor_analyzer_lib::country;
use tor_analyzer_lib::error::Error;
use tor_analyzer_lib::prelude::*;
use tor_analyzer_lib::tor::ns::OnionRouterFlag;

struct Node {
    endpoint: Target,
    country: Option<&'static country::Country>,
    identity: String,
    nickname: String,
    guard: bool,
    middle: bool,
    exit: bool,
}

#[derive(Debug)]
#[repr(i32)]
enum Columns {
    EndPoint,
    Country,
    Identity,
    Nickname,
    Guard,
    Middle,
    Exit,
    MaxColumns,
}
const FIELD_COUNT: usize = Columns::MaxColumns as usize;

fn get_nodes() -> Result<Vec<Node>, Error> {
    let mutex = crate::get_tor_controller();
    let mut ctrl = mutex.lock().unwrap();

    let mut ors = ctrl.get_all_onion_router()?;
    drop(ctrl);
    let mut nodes = Vec::with_capacity(ors.len());

    let gi = GeoIP::new();
    for or in ors.drain(..) {
        let country = gi
            .lookup_ip(or.target.addr.clone())
            .and_then(|c| country::get_country(c));
        let guard = or.flags.is_set(OnionRouterFlag::Guard);
        let middle = or.flags.is_set(OnionRouterFlag::Valid);
        let exit = or.flags.is_set(OnionRouterFlag::Exit);

        nodes.push(Node {
            endpoint: or.target,
            country,
            identity: hex_encode(or.identity),
            nickname: or.nickname,
            guard,
            middle,
            exit,
        });
    }

    Ok(nodes)
}

fn update_model(store: &gtk::ListStore) {
    eprintln!("Updating nodes (could take a while)");
    let data = match get_nodes() {
        Ok(nodes) => nodes,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    store.clear();

    let mut indexes = [0u32; FIELD_COUNT];
    for (i, idx) in indexes.iter_mut().enumerate() {
        *idx = i as u32;
    }
    for d in data.iter() {
        let endpoint = format!("{}", d.endpoint);
        let country = if let Some(c) = d.country {
            format!("{} {}", c.flag, c.name)
        } else {
            String::new()
        };
        let values: [&dyn ToValue; FIELD_COUNT] = [
            &endpoint,
            &country,
            &d.identity,
            &d.nickname,
            &d.guard,
            &d.middle,
            &d.exit,
        ];
        store.set(&store.append(), &indexes[..], &values);
    }
    eprintln!("Got {} nodes", data.len());
}

pub fn create_tab() -> gtk::Widget {
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
    vbox.set_homogeneous(false);

    let search_entry = Rc::new(gtk::Entry::new());
    search_entry.set_placeholder_text(Some("Enter text here to filter results"));
    vbox.add(&*Rc::clone(&search_entry));

    let sw = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    sw.set_shadow_type(gtk::ShadowType::EtchedIn);
    sw.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    vbox.add(&sw);

    let col_types: [glib::Type; FIELD_COUNT] = [
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::Bool,
        glib::Type::Bool,
        glib::Type::Bool,
    ];
    let store = Rc::new(gtk::ListStore::new(&col_types));

    let treefilter = Rc::new(gtk::TreeModelFilter::new(&*Rc::clone(&store), None));
    let search_entry_copy = Rc::clone(&search_entry);
    treefilter.set_visible_func(move |model, iter| -> bool {
        let filter = search_entry_copy.get_text().as_str().to_lowercase();

        crate::filter_func(filter, model, iter)
    });
    let treeview = gtk::TreeView::with_model(&*Rc::clone(&treefilter));
    treeview.set_vexpand(true);
    sw.add(&treeview);
    search_entry.connect_changed(move |_| treefilter.refilter());

    add_column!(treeview, Columns::EndPoint, "End point");
    add_column!(treeview, Columns::Country, "Country");
    add_column!(treeview, Columns::Identity, "Identity");
    add_column!(treeview, Columns::Nickname, "Nickname");
    add_column!(bool treeview, Columns::Guard, "Guard?");
    add_column!(bool treeview, Columns::Middle, "Middle?");
    add_column!(bool treeview, Columns::Exit, "Exit?");

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
