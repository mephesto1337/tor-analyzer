use std::rc::Rc;

use gio::prelude::*;
use gtk::prelude::*;

use tor_analyzer_lib::country;
use tor_analyzer_lib::error::Error;
use tor_analyzer_lib::prelude::*;

#[derive(Debug)]
struct SimpleCircuit {
    id: CircuitID,
    state: tor_analyzer_lib::tor::circuit::CircuitStatus,
    path: Vec<OnionRouter>,
    endpoint: Option<Target>,
}

#[derive(Debug)]
#[repr(i32)]
enum Columns {
    Id,
    Status,
    Ips,
    Countries,
    Path,
    EndPoint,
    MaxColumns,
}
const FIELD_COUNT: usize = Columns::MaxColumns as usize;

fn get_circuits() -> Result<Vec<SimpleCircuit>, Error> {
    let mutex = crate::get_tor_controller();
    let mut ctrl = mutex.lock().unwrap();

    let mut circuits = ctrl.get_circuits()?;
    let streams = ctrl.get_streams()?;
    drop(ctrl);

    let mut simple_circuits = Vec::with_capacity(circuits.len());
    for mut c in circuits.drain(..) {
        let mut path = Vec::with_capacity(c.path.len());

        for mut step in c.path.drain(..) {
            step.nickname = None;
            let mut ctrl = mutex.lock().unwrap();
            let or = ctrl.get_onion_router(&step)?;
            drop(ctrl);
            path.push(or);
        }

        let endpoint = streams.iter().find_map(|s| {
            if s.circuit_id == c.id {
                Some(s.target.clone())
            } else {
                None
            }
        });

        simple_circuits.push(SimpleCircuit {
            id: c.id,
            state: c.status,
            path,
            endpoint,
        })
    }

    Ok(simple_circuits)
}

fn localize_target(writer: &mut String, target: &Target, gi: &GeoIP) {
    if let Some(loc) = gi.lookup_ip(target.addr) {
        if let Some(country) = country::get_country(loc) {
            writer.push_str(country.flag);
            writer.push(' ');
            writer.push_str(country.name);
        } else {
            writer.push_str(loc);
        }
    }
}

fn update_model(store: &gtk::ListStore) {
    let data = match get_circuits() {
        Ok(circuits) => circuits,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    let gi = GeoIP::new();

    store.clear();

    let mut indexes = [0u32; FIELD_COUNT];
    for (i, idx) in indexes.iter_mut().enumerate() {
        *idx = i as u32;
    }
    for d in data.iter() {
        let id = format!("{}", d.id);
        let state = format!("{}", d.state);
        let mut paths = String::new();
        let mut ips = String::new();
        let mut countries = String::new();
        let mut first = true;
        for p in &d.path {
            if !first {
                countries.push('\n');
                paths.push('\n');
                ips.push('\n');
            }
            first = false;
            localize_target(&mut countries, &p.target, &gi);
            ips.push_str(&format!("{}", p.target));
            paths.push_str(&format!("{}", p));
        }
        let endpoint = if let Some(ref ep) = d.endpoint {
            let mut endpoint = format!("{} ", ep);
            localize_target(&mut endpoint, ep, &gi);
            endpoint
        } else {
            String::new()
        };

        let values: [&dyn ToValue; FIELD_COUNT] =
            [&id, &state, &ips, &countries, &paths, &endpoint];
        store.set(&store.append(), &indexes[..], &values);
    }
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

    let col_types = [glib::Type::String; FIELD_COUNT];
    let store = Rc::new(gtk::ListStore::new(&col_types));

    let treefilter = Rc::new(gtk::TreeModelFilter::new(&*Rc::clone(&store), None));
    let search_entry_copy = Rc::clone(&search_entry);
    treefilter.set_visible_func(move |model, iter| -> bool {
        let filter = search_entry_copy.get_text().as_str().to_lowercase();

        crate::filter_func(filter, model, iter)
    });
    let treeview = gtk::TreeView::with_model(&*Rc::clone(&treefilter));
    treeview.set_vexpand(true);
    treeview.set_search_column(Columns::Id as i32);
    sw.add(&treeview);
    search_entry.connect_changed(move |_| treefilter.refilter());

    add_column!(treeview, Columns::Id, "Id");
    add_column!(treeview, Columns::Status, "Status");
    add_column!(treeview, Columns::Ips, "IPs");
    add_column!(treeview, Columns::Countries, "Countries");
    add_column!(treeview, Columns::Path, "Path");
    add_column!(treeview, Columns::EndPoint, "End point");

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
