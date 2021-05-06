use std::rc::Rc;

use gtk::prelude::*;

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
}

impl CircuitTab {
    pub(crate) fn new() -> Self {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
        vbox.set_homogeneous(false);

        let search_entry = Rc::new(gtk::Entry::new());
        search_entry.set_placeholder_text(Some("Enter text here to filter results"));
        vbox.add(&*Rc::clone(&search_entry));

        let widget: gtk::Widget = vbox.upcast();

        Self {
            entry: search_entry,
            widget: Rc::new(widget),
        }
    }

    pub(crate) fn get_entry(&self) -> Rc<gtk::Entry> {
        Rc::clone(&self.entry)
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
