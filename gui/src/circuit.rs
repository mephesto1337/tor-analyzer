use std::cell::Cell;
use std::rc::Rc;

use gio::prelude::*;
use gtk::prelude::*;

use tor_analyzer_lib::country;
use tor_analyzer_lib::error::Error;
use tor_analyzer_lib::prelude::*;

use crate::notebook::NotebookTab;

struct Circuit {
    circuit: tor_analyzer_lib::prelude::Circuit,
    path: Vec<OnionRouter>,
    countries: Vec<Option<&'static country::Country>>,
    endpoint: Option<Target>,
}

impl Circuit {
    fn id(&self) -> String {
        format!("{}", self.circuit.id)
    }
    fn status(&self) -> String {
        format!("{}", self.circuit.status)
    }
    fn ips(&self) -> String {
        let mut ips = String::new();
        let mut first = true;
        for or in self.path.iter() {
            if !first {
                ips.push('\n');
            }
            first = false;
            ips.push_str(&format!("{}", or.target));
        }
        ips
    }

    fn countries(&self) -> String {
        let mut countries = String::new();
        let mut first = true;
        for c in self.countries.iter() {
            if !first {
                countries.push('\n');
            }
            first = false;
            if let Some(c) = c {
                countries.push_str(c.flag);
                countries.push(' ');
                countries.push_str(c.name);
            } else {
                countries.push_str("???");
            }
        }
        countries
    }

    fn path(&self) -> String {
        let mut path = String::new();
        let mut first = false;
        for p in self.circuit.path.iter() {
            if !first {
                path.push('\n');
            }
            first = false;
            path.push_str(&format!("{p}"));
        }
        path
    }

    fn endpoint(&self) -> String {
        match self.endpoint {
            Some(ref ep) => format!("{ep}"),
            None => String::new(),
        }
    }
}

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

pub(crate) struct CircuitTab {
    circuits: Cell<Option<Vec<Circuit>>>,
    widget: Cell<Option<Rc<gtk::Widget>>>,
    store: gtk::ListStore,
}

impl CircuitTab {
    pub(crate) fn new() -> Rc<Self> {
        let col_types = [glib::Type::STRING; FIELD_COUNT];
        let me = Self {
            circuits: Cell::new(None),
            widget: Cell::new(None),
            store: gtk::ListStore::new(&col_types),
        };
        me.create()
    }

    fn create(self) -> Rc<Self> {
        let me = Rc::new(self);
        Rc::clone(&me).create_ui();
        me
    }

    fn get_circuits(&self) -> Vec<Circuit> {
        match self.circuits.take() {
            Some(c) => c,
            None => Vec::new(),
        }
    }

    fn set_circuits(&self, v: Vec<Circuit>) {
        self.circuits.set(Some(v));
    }

    fn create_ui(self: Rc<Self>) {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
        vbox.set_homogeneous(false);

        let search_entry = Rc::new(gtk::Entry::new());
        search_entry.set_placeholder_text(Some("Enter text here to filter results"));
        vbox.add(&*Rc::clone(&search_entry));

        let sw = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        sw.set_shadow_type(gtk::ShadowType::EtchedIn);
        sw.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        vbox.add(&sw);

        let treefilter = Rc::new(gtk::TreeModelFilter::new(&self.store, None));
        let search_entry_copy = Rc::clone(&search_entry);
        treefilter.set_visible_func(move |model, iter| -> bool {
            let filter = search_entry_copy.text().as_str().to_lowercase();

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
        let me = Rc::clone(&self);
        update_btn.connect_clicked(move |_| match me.refresh_data() {
            Ok(_) => me.refresh_view(),
            Err(e) => log::warn!("Could not refresh data: {}", e),
        });
        vbox.add(&update_btn);

        // Fill table
        update_btn.clicked();

        let widget = Some(Rc::new(vbox.upcast()));
        self.widget.set(widget);
    }

    fn refresh_data(&self) -> Result<(), Error> {
        let mut circuits = self.get_circuits();
        circuits.clear();
        let mutex = crate::get_tor_controller();
        let mut ctrl = mutex.lock().unwrap();

        let mut ctrl_circuits = ctrl.get_circuits()?;
        let streams = ctrl.get_streams()?;
        drop(ctrl);

        let gi = GeoIP::new();

        for c in ctrl_circuits.drain(..) {
            let mut path = Vec::with_capacity(c.path.len());

            for step in c.path.iter() {
                let mut ctrl = mutex.lock().unwrap();
                let or = ctrl.get_onion_router(hex_encode(step.fingerprint))?;
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

            let countries = path
                .iter()
                .map(|p| match p.target.addr {
                    HostOrAddr::Addr(ref addr) => gi.lookup_ip(*addr),
                    _ => None,
                })
                .map(|loc| {
                    if let Some(loc) = loc {
                        country::get_country(loc)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            circuits.push(Circuit {
                circuit: c,
                path,
                countries,
                endpoint,
            })
        }

        self.set_circuits(circuits);

        Ok(())
    }

    fn refresh_view(&self) {
        self.store.clear();

        let mut indexes = [0u32; FIELD_COUNT];
        for (i, idx) in indexes.iter_mut().enumerate() {
            *idx = i as u32;
        }
        let circuits = self.get_circuits();
        for c in circuits.iter() {
            let values: [(u32, &dyn ToValue); FIELD_COUNT] = [
                (0, &c.id()),
                (1, &c.status()),
                (2, &c.ips()),
                (3, &c.countries()),
                (4, &c.path()),
                (5, &c.endpoint()),
            ];
            self.store.set(&self.store.append(), &values);
        }
        self.set_circuits(circuits);
    }
}

impl NotebookTab for CircuitTab {
    fn get_widget(&self) -> Rc<gtk::Widget> {
        let widget = self.widget.take().unwrap();
        let copy = Rc::clone(&widget);
        self.widget.set(Some(widget));
        copy
    }

    fn label(&self) -> &'static str {
        "Circuits"
    }
}
