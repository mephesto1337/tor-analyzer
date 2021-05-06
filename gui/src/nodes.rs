use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use gtk::prelude::*;

use tor_analyzer_lib::country;
use tor_analyzer_lib::error::Error;
use tor_analyzer_lib::prelude::*;
use tor_analyzer_lib::tor::ns::OnionRouterFlag;

use crate::notebook::NotebookTab;

#[derive(Debug)]
#[repr(i32)]
enum Columns {
    EndPoint,
    Country,
    Identity,
    Nickname,
    Guard,
    Exit,
    MaxColumns,
}
const FIELD_COUNT: usize = Columns::MaxColumns as usize;

struct Node {
    country: Option<&'static country::Country>,
    or: OnionRouter,
}

impl Node {
    fn target(&self) -> &Target {
        &self.or.target
    }
    fn hex_identity(&self) -> String {
        hex_encode(self.or.identity)
    }
    fn nickname(&self) -> &String {
        &self.or.nickname
    }
    fn is_guard(&self) -> bool {
        self.or.flags.is_set(OnionRouterFlag::Guard)
    }
    fn is_exit(&self) -> bool {
        self.or.flags.is_set(OnionRouterFlag::Exit)
    }
}

pub(super) struct NodeTab {
    nodes: Cell<Option<HashMap<String, Node>>>,
    entry: Cell<Option<Rc<gtk::Entry>>>,
    widget: Cell<Option<Rc<gtk::Widget>>>,
    store: gtk::ListStore,
}

impl NodeTab {
    pub(crate) fn new() -> Rc<Self> {
        let col_types: [glib::Type; FIELD_COUNT] = [
            glib::Type::String,
            glib::Type::String,
            glib::Type::String,
            glib::Type::String,
            glib::Type::Bool,
            glib::Type::Bool,
        ];

        let me = Self {
            nodes: Cell::new(None),
            entry: Cell::new(None),
            widget: Cell::new(None),
            store: gtk::ListStore::new(&col_types),
        };
        me.create()
    }

    pub(crate) fn set_entry(&self, entry: Rc<gtk::Entry>) {
        let cur_entry = self.entry.take();
        if cur_entry.is_some() {
            eprintln!("Overwrite previous entry");
        }
        self.entry.set(Some(entry));
    }

    fn create(self) -> Rc<Self> {
        let me = Rc::new(self);
        Rc::clone(&me).create_ui();
        me
    }

    fn get_nodes(&self) -> HashMap<String, Node> {
        match self.nodes.take() {
            Some(hm) => hm,
            None => HashMap::with_capacity(4096),
        }
    }

    fn set_nodes(&self, hm: HashMap<String, Node>) {
        self.nodes.set(Some(hm));
    }

    fn refresh_data(&self) -> Result<(), Error> {
        #[cfg(debug_assertions)]
        eprintln!("Updating nodes (could take a while, your are in debug mode)");
        let mut nodes = self.get_nodes();
        nodes.clear();
        let mutex = crate::get_tor_controller();
        let mut ctrl = mutex.lock().unwrap();
        let mut ors = ctrl.get_all_onion_router()?;
        drop(ctrl);

        let gi = GeoIP::new();
        for or in ors.drain(..) {
            let country = gi
                .lookup_ip(or.target.addr.clone())
                .and_then(|c| country::get_country(c));

            let node = Node { country, or };
            nodes.insert(node.hex_identity(), node);
        }

        self.set_nodes(nodes);

        Ok(())
    }

    fn refresh_view(&self) {
        self.store.clear();

        let mut indexes = [0u32; FIELD_COUNT];
        for (i, idx) in indexes.iter_mut().enumerate() {
            *idx = i as u32;
        }
        let nodes = self.get_nodes();
        for d in nodes.values() {
            let endpoint = format!("{}", d.target());
            let country = if let Some(c) = d.country {
                format!("{} {}", c.flag, c.name)
            } else {
                String::new()
            };
            let values: [&dyn ToValue; FIELD_COUNT] = [
                &endpoint,
                &country,
                &d.hex_identity(),
                &d.nickname(),
                &d.is_guard(),
                &d.is_exit(),
            ];
            self.store.set(&self.store.append(), &indexes[..], &values);
        }
        self.set_nodes(nodes);
    }

    fn create_ui(self: Rc<Self>) {
        let widget = self.widget.take();
        if widget.is_some() {
            self.widget.set(widget);
            return;
        }

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
            let filter = search_entry_copy.get_text().as_str().to_lowercase();

            crate::filter_func(filter, model, iter)
        });
        let treeview = gtk::TreeView::with_model(&*Rc::clone(&treefilter));
        treeview.set_vexpand(true);

        let me = Rc::clone(&self);
        treeview.connect_row_activated(move |treeview, treepath, _treeviewcolumn| {
            let model = treeview.get_model().unwrap();
            let iter = model.get_iter(treepath).unwrap();
            let data = model.get_value(&iter, Columns::Identity as i32);
            if let Some(text) = data.get::<String>().unwrap() {
                if let Some(entry) = me.entry.take() {
                    entry.set_text(text.as_str());
                    me.entry.set(Some(entry));
                }
            }
        });
        sw.add(&treeview);
        search_entry.connect_changed(move |_| treefilter.refilter());

        add_column!(treeview, Columns::EndPoint, "End point");
        add_column!(treeview, Columns::Country, "Country");
        add_column!(treeview, Columns::Identity, "Identity");
        add_column!(treeview, Columns::Nickname, "Nickname");
        add_column!(bool treeview, Columns::Guard, "Guard?");
        add_column!(bool treeview, Columns::Exit, "Exit?");

        let update_btn = gtk::Button::with_label("Update");
        let me = Rc::clone(&self);
        update_btn.connect_clicked(move |_| match me.refresh_data() {
            Ok(_) => me.refresh_view(),
            Err(e) => eprintln!("Could not refresh data: {}", e),
        });
        vbox.add(&update_btn);

        // Fill table
        update_btn.clicked();

        let widget = Some(Rc::new(vbox.upcast()));
        self.widget.set(widget);
    }
}

impl NotebookTab for NodeTab {
    fn get_widget(&self) -> Rc<gtk::Widget> {
        let widget = self.widget.take().unwrap();
        let copy = Rc::clone(&widget);
        self.widget.set(Some(widget));
        copy
    }

    fn label(&self) -> &'static str {
        "Nodes"
    }
}
