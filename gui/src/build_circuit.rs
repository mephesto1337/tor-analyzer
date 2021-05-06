use std::cell::Cell;
use std::rc::Rc;

use gtk::prelude::*;

use tor_analyzer_lib::error::Error;
use tor_analyzer_lib::prelude::*;

use crate::NotebookTab;

struct Circuit {
    endpoint: Target,
    entry: OnionRouter,
    middle: Vec<OnionRouter>,
    exit: OnionRouter,
}

pub(crate) struct CircuitTab {
    entry: Rc<gtk::Entry>,
    widget: Rc<gtk::Widget>,
    circuits: gtk::ComboBoxText,
}

impl CircuitTab {
    pub(crate) fn new() -> Rc<Self> {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
        vbox.set_homogeneous(false);

        let entry = Rc::new(gtk::Entry::new());
        entry.set_placeholder_text(Some("Enter OR long name"));

        let widget: gtk::Widget = vbox.upcast();

        let me = Rc::new(Self {
            entry,
            widget: Rc::new(widget),
            circuits: gtk::ComboBoxText::new(),
        });
        Rc::clone(&me).create_ui();
        me
    }

    pub(crate) fn get_entry(&self) -> Rc<gtk::Entry> {
        Rc::clone(&self.entry)
    }

    pub(crate) fn create_ui(self: Rc<Self>) {
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 8);

        let update_btn = gtk::Button::with_label("Update circuits");
        let me = Rc::clone(&self);
        update_btn.connect_clicked(move |_| match me.refresh_data() {
            Ok(_) => {}
            Err(e) => eprintln!("Could not refresh data: {}", e),
        });
        hbox.add(&self.circuits);
        hbox.add(&update_btn);
        hbox.set_homogeneous(true);

        // Fill table
        update_btn.clicked();

        let vbox = self.widget.downcast_ref::<gtk::Box>().unwrap();
        vbox.add(&hbox);
        vbox.add(&*Rc::clone(&self.entry));
    }

    fn refresh_data(&self) -> Result<(), Error> {
        let mutex = crate::get_tor_controller();
        let mut ctrl = mutex.lock().unwrap();
        let circuits = ctrl.get_circuits()?;
        drop(ctrl);

        self.circuits.remove_all();

        eprintln!("Adding 0");
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
        let widget = Rc::clone(&self.widget);
        widget
    }

    fn label(&self) -> &'static str {
        "Circuit Builder"
    }
}
