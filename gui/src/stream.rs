use std::collections::HashSet;
use std::rc::Rc;

use gtk::prelude::*;

use tor_analyzer_lib::error::Error;
use tor_analyzer_lib::prelude::*;
use tor_analyzer_lib::tor::circuit::CircuitStatus;
use tor_analyzer_lib::tor::stream::StreamStatus;

use crate::NotebookTab;

pub(crate) struct StreamTab {
    vbox: Rc<gtk::Box>,
    widget: Rc<gtk::Widget>,
}

impl StreamTab {
    pub(crate) fn new() -> Rc<Self> {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
        vbox.set_homogeneous(false);
        let widget: gtk::Widget = vbox.upcast();

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
        vbox.set_homogeneous(true);

        let me = Rc::new(Self {
            vbox: Rc::new(vbox),
            widget: Rc::new(widget),
        });
        Rc::clone(&me).create_ui();
        me
    }

    pub(crate) fn create_ui(self: Rc<Self>) {
        let vbox = self.widget.downcast_ref::<gtk::Box>().unwrap();
        vbox.add(&*Rc::clone(&self.vbox));

        // TODO: use gtk::Grid
        let update_btn = gtk::Button::with_label("Update streams");
        let me = Rc::clone(&self);
        update_btn.connect_clicked(move |_| {
            for child in me.vbox.get_children() {
                if child.is::<gtk::Box>() {
                    eprintln!("Kill !");
                    child.hide();
                    me.vbox.remove(&child);
                    break;
                }
            }

            let (circuits, streams) = match me.refresh_data() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Could not refresh data: {}", e);
                    return;
                }
            };

            let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
            let occupied_circuits = streams
                .iter()
                .filter_map(|s| {
                    if s.circuit_id.0 != "0" {
                        Some(&s.circuit_id)
                    } else {
                        None
                    }
                })
                .collect::<HashSet<_>>();

            for stream in streams.iter().filter(|s| s.status == StreamStatus::New) {
                eprintln!("Got stream: {:?}", stream);
                let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 8);
                let label = gtk::Label::new(Some(format!("{}", stream.target).as_str()));
                hbox.add(&label);

                let combobox = gtk::ComboBoxText::new();
                for circuit in circuits.iter().filter(|c| {
                    c.status == CircuitStatus::Built && occupied_circuits.contains(&&c.id)
                }) {
                    combobox.append_text(format!("{}", circuit.id).as_str());
                }
                hbox.add(&combobox);

                let assign_btn = gtk::Button::with_label("Assign");
                hbox.add(&assign_btn);
                assign_btn.connect_clicked(|_| {
                    eprintln!("Click!");
                });

                vbox.add(&hbox);
            }
            vbox.show();
            eprintln!("Add it!");
            me.vbox.add(&vbox);
        });
        vbox.add(&update_btn);

        // Fill table
        update_btn.clicked();
    }

    fn refresh_data(&self) -> Result<(Vec<Circuit>, Vec<Stream>), Error> {
        let mutex = crate::get_tor_controller();
        let mut ctrl = mutex.lock().unwrap();
        let circuits = ctrl.get_circuits()?;
        let streams = ctrl.get_streams()?;
        drop(ctrl);

        Ok((circuits, streams))
    }
}

impl NotebookTab for StreamTab {
    fn get_widget(&self) -> Rc<gtk::Widget> {
        let widget = Rc::clone(&self.widget);
        widget
    }

    fn label(&self) -> &'static str {
        "Streams"
    }
}
