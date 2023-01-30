use std::rc::Rc;

use gtk::prelude::*;

use tor_analyzer_lib::error::Error;
use tor_analyzer_lib::prelude::*;

use crate::NotebookTab;

// struct Circuit {
//     endpoint: Target,
//     entry: OnionRouter,
//     middle: Vec<OnionRouter>,
//     exit: OnionRouter,
// }

use crate::nodes::{Columns, COLUMNS_TYPE, FIELD_COUNT};

pub(crate) struct CircuitTab {
    nodes: Rc<gtk::ListStore>,
    widget: Rc<gtk::Widget>,
    circuits: gtk::ComboBoxText,
}

impl CircuitTab {
    pub(crate) fn new() -> Rc<Self> {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
        vbox.set_homogeneous(false);

        // let entry = Rc::new(gtk::Entry::new());
        // entry.set_placeholder_text(Some("Enter OR long name"));

        let widget: gtk::Widget = vbox.upcast();
        let me = Rc::new(Self {
            nodes: Rc::new(gtk::ListStore::new(&COLUMNS_TYPE)),
            widget: Rc::new(widget),
            circuits: gtk::ComboBoxText::new(),
        });
        Rc::clone(&me).create_ui();
        me
    }

    pub(crate) fn add_row(&self, row: &[glib::Value]) {
        let iter = self.nodes.append();
        (0..FIELD_COUNT).for_each(|col| self.nodes.set_value(&iter, col as u32, &row[col]));
    }

    pub(crate) fn create_ui(self: Rc<Self>) {
        let vbox = self.widget.downcast_ref::<gtk::Box>().unwrap();

        // TODO: update self.nodes on self.circuits change

        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        let update_btn = gtk::Button::with_label("Update circuits");
        let me = Rc::clone(&self);
        update_btn.connect_clicked(move |_| match me.refresh_data() {
            Ok(_) => {}
            Err(e) => log::warn!("Could not refresh data: {}", e),
        });
        hbox.add(&self.circuits);
        hbox.add(&update_btn);
        hbox.set_homogeneous(true);
        vbox.add(&hbox);

        let sw = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        sw.set_shadow_type(gtk::ShadowType::EtchedIn);
        sw.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        vbox.add(&sw);

        let treeview = gtk::TreeView::with_model(&*Rc::clone(&self.nodes));
        treeview.set_vexpand(true);
        treeview.selection().set_mode(gtk::SelectionMode::Multiple);

        // Drag and drop
        treeview.set_reorderable(true);

        let me = Rc::clone(&self);
        treeview.connect_key_press_event(move |treeview, keyevent| {
            if keyevent.keyval() != gdk::keys::constants::Delete {
                return gtk::Inhibit(false);
            }
            let treemodel = treeview.model().unwrap();
            let (selection, _treemodel) = treeview.selection().selected_rows();

            if selection.is_empty() {
                return gtk::Inhibit(true);
            }
            let model = Rc::clone(&me.nodes);
            for path in selection.iter() {
                if let Some(iter) = treemodel.iter(path) {
                    model.remove(&iter);
                }
            }
            treeview.show_all();

            gtk::Inhibit(true)
        });
        sw.add(&treeview);
        add_column!(treeview, Columns::EndPoint, "End point");
        add_column!(treeview, Columns::Country, "Country");
        add_column!(treeview, Columns::Identity, "Identity");
        add_column!(treeview, Columns::Nickname, "Nickname");
        add_column!(bool treeview, Columns::Guard, "Guard?");
        add_column!(bool treeview, Columns::Exit, "Exit?");

        let create_circuit_btn = gtk::Button::with_label("Create");
        let me = Rc::clone(&self);
        create_circuit_btn.connect_clicked(move |_| {
            let store = &*me.nodes;
            let mut path = Vec::new();
            store.foreach(|model, _path, iter| {
                let value = model.value(iter, Columns::Identity as i32);
                path.push(value.get::<String>().unwrap());
                false
            });

            if path.len() < 3 {
                popup_error!("Need at least 3 nodes to build a ciruit");
                return;
            }
            let circuit_id = CircuitID(
                me.circuits
                    .active_text()
                    .map(|gs| gs.into())
                    .unwrap_or(String::from("0")),
            );
            let mutex = crate::get_tor_controller();
            let mut ctrl = mutex.lock().unwrap();

            if let Err(e) = ctrl.extend_circuit(circuit_id, path) {
                popup_error!("Could not extend circuit: {}", e);
                return;
            }

            me.nodes.clear();
        });
        vbox.add(&create_circuit_btn);

        let clear_circuit_btn = gtk::Button::with_label("Clear");
        let me = Rc::clone(&self);
        clear_circuit_btn.connect_clicked(move |_| {
            me.nodes.clear();
        });
        vbox.add(&clear_circuit_btn);

        // Fill table
        update_btn.clicked();
    }

    fn refresh_data(&self) -> Result<(), Error> {
        let mutex = crate::get_tor_controller();
        let mut ctrl = mutex.lock().unwrap();
        let circuits = ctrl.get_circuits()?;
        drop(ctrl);

        self.circuits.remove_all();

        self.circuits.append(None, "0");
        for c in circuits.iter() {
            let id = format!("{}", c.id);
            self.circuits.append(None, id.as_str());
        }

        Ok(())
    }
}

impl NotebookTab for CircuitTab {
    fn get_widget(&self) -> Rc<gtk::Widget> {
        Rc::clone(&self.widget)
    }

    fn label(&self) -> &'static str {
        "Circuit Builder"
    }
}
