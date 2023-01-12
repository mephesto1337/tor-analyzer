use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use gtk::prelude::*;

use tor_analyzer_lib::country;
use tor_analyzer_lib::error::Error;
use tor_analyzer_lib::prelude::*;
use tor_analyzer_lib::tor::ns::OnionRouterFlag;

use crate::build_circuit::CircuitTab;
use crate::notebook::NotebookTab;

#[repr(i32)]
pub(crate) enum Columns {
    EndPoint,
    Country,
    Identity,
    Nickname,
    Guard,
    Exit,
    BadExit,
    MaxColumns,
}
pub(crate) const FIELD_COUNT: usize = Columns::MaxColumns as usize;
pub(crate) const COLUMNS_TYPE: [glib::Type; FIELD_COUNT] = [
    glib::Type::STRING,
    glib::Type::STRING,
    glib::Type::STRING,
    glib::Type::STRING,
    glib::Type::BOOL,
    glib::Type::BOOL,
    glib::Type::BOOL,
];

pub(crate) struct Node {
    country: Option<&'static country::Country>,
    or: OnionRouter,
}

impl Node {
    pub(crate) fn target(&self) -> &Target {
        &self.or.target
    }
    pub(crate) fn hex_identity(&self) -> String {
        hex_encode(self.or.identity)
    }
    pub(crate) fn nickname(&self) -> &String {
        &self.or.nickname
    }
    pub(crate) fn is_guard(&self) -> bool {
        self.or.flags.is_set(OnionRouterFlag::Guard)
    }
    pub(crate) fn is_exit(&self) -> bool {
        self.or.flags.is_set(OnionRouterFlag::Exit)
    }
    pub(crate) fn is_badexit(&self) -> bool {
        self.or.flags.is_set(OnionRouterFlag::BadExit)
    }
}

pub(super) struct NodeTab {
    nodes: Cell<Option<HashMap<String, Node>>>,
    circuit_tab: Cell<Option<Rc<CircuitTab>>>,
    widget: Cell<Option<Rc<gtk::Widget>>>,
    store: gtk::ListStore,
}

impl NodeTab {
    pub(crate) fn new() -> Rc<Self> {
        let me = Self {
            nodes: Cell::new(None),
            circuit_tab: Cell::new(None),
            widget: Cell::new(None),
            store: gtk::ListStore::new(&COLUMNS_TYPE),
        };
        me.create()
    }

    pub(crate) fn get_circuit_tab(&self) -> Option<Rc<CircuitTab>> {
        if let Some(cur_circuit_tab) = self.circuit_tab.take() {
            let circuit_tab = Rc::clone(&cur_circuit_tab);
            self.circuit_tab.set(Some(cur_circuit_tab));
            Some(circuit_tab)
        } else {
            None
        }
    }

    pub(crate) fn set_circuit_tab(&self, circuit_tab: Rc<CircuitTab>) {
        let cur_circuit_tab = self.circuit_tab.take();
        if cur_circuit_tab.is_some() {
            log::warn!("Overwrite previous entry");
        }
        self.circuit_tab.set(Some(circuit_tab));
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
        log::warn!("Updating nodes (could take a while, your are in debug mode)");
        let mut nodes = self.get_nodes();
        nodes.clear();
        let mutex = crate::get_tor_controller();
        let mut ctrl = mutex.lock().unwrap();
        let mut ors = ctrl.get_all_onion_router()?;
        drop(ctrl);

        let gi = GeoIP::new();
        for or in ors.drain(..) {
            let country = match or.target.addr {
                HostOrAddr::Addr(ref addr) => gi
                    .lookup_ip(addr.clone())
                    .and_then(|c| country::get_country(c)),
                _ => None,
            };

            let node = Node { country, or };
            nodes.insert(node.hex_identity(), node);
        }

        self.set_nodes(nodes);

        Ok(())
    }

    fn refresh_view(&self) {
        self.store.clear();

        let nodes = self.get_nodes();
        for d in nodes.values() {
            let endpoint = format!("{}", d.target());
            let country = if let Some(c) = d.country {
                format!("{} {}", c.flag, c.name)
            } else {
                String::new()
            };
            let values: [(u32, &dyn ToValue); FIELD_COUNT] = [
                (0, &endpoint),
                (1, &country),
                (2, &d.hex_identity()),
                (3, &d.nickname()),
                (4, &d.is_guard()),
                (5, &d.is_exit()),
                (6, &d.is_badexit()),
            ];
            self.store.set(&self.store.append(), &values);
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
            let filter = search_entry_copy.text().as_str().to_lowercase();

            crate::filter_func(filter, model, iter)
        });
        let treeview = gtk::TreeView::with_model(&*Rc::clone(&treefilter));
        treeview.set_vexpand(true);

        let me = Rc::clone(&self);
        treeview.connect_row_activated(move |treeview, treepath, _treeviewcolumn| {
            let model = treeview.model().unwrap();
            let iter = model.iter(treepath).unwrap();
            let row = (0..FIELD_COUNT)
                .map(|col| model.value(&iter, col as i32))
                .collect::<Vec<_>>();

            let circuit_tab = me.get_circuit_tab().unwrap();
            circuit_tab.add_row(&row[..]);
        });
        sw.add(&treeview);
        search_entry.connect_changed(move |_| treefilter.refilter());

        add_column!(treeview, Columns::EndPoint, "End point");
        add_column!(treeview, Columns::Country, "Country");
        add_column!(treeview, Columns::Identity, "Identity");
        add_column!(treeview, Columns::Nickname, "Nickname");
        add_column!(bool treeview, Columns::Guard, "Guard?");
        add_column!(bool treeview, Columns::Exit, "Exit?");
        add_column!(bool treeview, Columns::BadExit, "Bad exit?");

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
