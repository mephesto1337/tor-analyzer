use std::cell::Cell;
use std::collections::HashSet;
use std::rc::Rc;

use gtk::prelude::*;

use tor_analyzer_lib::country;
use tor_analyzer_lib::error::Error;
use tor_analyzer_lib::prelude::*;
use tor_analyzer_lib::tor::circuit::CircuitStatus;
use tor_analyzer_lib::tor::stream::StreamStatus;

use crate::NotebookTab;

#[repr(i32)]
enum Columns {
    StreamID,
    StreamEndpoint,
    CircuitIds,
    AttachButton,
}

struct Circuit {
    circuit: tor_analyzer_lib::prelude::Circuit,
    out_country: Option<&'static country::Country>,
}

pub(crate) struct StreamTab {
    grid: Rc<gtk::Grid>,
    row_count: Cell<i32>,
    widget: Rc<gtk::Widget>,
}

impl StreamTab {
    pub(crate) fn new() -> Rc<Self> {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
        vbox.set_homogeneous(false);
        let widget: gtk::Widget = vbox.upcast();

        let grid = gtk::Grid::new();

        let me = Rc::new(Self {
            grid: Rc::new(grid),
            row_count: Cell::new(0),
            widget: Rc::new(widget),
        });
        Rc::clone(&me).create_ui();
        me
    }

    pub(crate) fn create_ui(self: Rc<Self>) {
        let vbox = self.widget.downcast_ref::<gtk::Box>().unwrap();
        vbox.add(&*Rc::clone(&self.grid));

        self.grid.set_hexpand(true);
        self.grid.set_vexpand(false);
        self.grid.set_row_homogeneous(true);
        self.grid.set_column_homogeneous(false);

        let label = gtk::Label::new(Some("ID"));
        label.set_vexpand(true);
        label.set_hexpand(true);
        label.set_halign(gtk::Align::Center);
        self.grid.attach(&label, Columns::StreamID as i32, 0, 1, 1);

        let label = gtk::Label::new(Some("End point"));
        label.set_vexpand(true);
        label.set_hexpand(true);
        label.set_halign(gtk::Align::Center);
        self.grid
            .attach(&label, Columns::StreamEndpoint as i32, 0, 1, 1);

        let label = gtk::Label::new(Some("Circuit"));
        label.set_vexpand(true);
        label.set_hexpand(true);
        label.set_halign(gtk::Align::Center);
        self.grid
            .attach(&label, Columns::CircuitIds as i32, 0, 1, 1);

        let label = gtk::Label::new(Some("Button"));
        label.set_vexpand(true);
        label.set_hexpand(true);
        label.set_halign(gtk::Align::Center);
        self.grid
            .attach(&label, Columns::AttachButton as i32, 0, 1, 1);
        self.row_count.set(1);

        let update_btn = gtk::Button::with_label("Update streams");
        let me = Rc::clone(&self);
        update_btn.connect_clicked(move |_| {
            let row_count = me.row_count.get();
            for _ in 1..row_count {
                me.grid.remove_row(1);
            }
            me.row_count.set(1);

            let (circuits, streams) = match me.refresh_data() {
                Ok(v) => v,
                Err(e) => {
                    log::warn!("Could not refresh data: {}", e);
                    return;
                }
            };

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
                let row_count = me.row_count.get();
                me.grid.insert_row(row_count);
                me.row_count.set(row_count + 1);

                let label = gtk::Label::new(Some(format!("{}", stream.id).as_str()));
                me.grid
                    .attach(&label, Columns::StreamID as i32, row_count, 1, 1);

                let label = gtk::Label::new(Some(format!("{}", stream.target).as_str()));
                me.grid
                    .attach(&label, Columns::StreamEndpoint as i32, row_count, 1, 1);

                let combobox = gtk::ComboBoxText::new();
                for circuit in circuits.iter().filter(|c| {
                    c.circuit.status == CircuitStatus::Built
                        && !occupied_circuits.contains(&&c.circuit.id)
                }) {
                    if let Some(ref out_country) = circuit.out_country {
                        combobox.append(
                            Some(circuit.circuit.id.0.as_str()),
                            format!(
                                "{} {} ({})",
                                out_country.flag, out_country.name, circuit.circuit.id
                            )
                            .as_str(),
                        );
                    } else {
                        combobox.append_text(format!("{}", circuit.circuit.id).as_str());
                    }
                }
                me.grid
                    .attach(&combobox, Columns::CircuitIds as i32, row_count, 1, 1);

                let assign_btn = gtk::Button::with_label("Assign");
                me.grid
                    .attach(&assign_btn, Columns::AttachButton as i32, row_count, 1, 1);

                let me_btn = Rc::clone(&me);
                assign_btn.connect_clicked(move |b| {
                    let idx = me_btn
                        .grid
                        .children()
                        .iter()
                        .enumerate()
                        .find_map(|(idx, widget)| if widget == b { Some(idx + 1) } else { None })
                        .unwrap();

                    let idx = idx as i32;
                    let stream_id = me_btn
                        .grid
                        .child_at(Columns::StreamID as i32, idx)
                        .unwrap()
                        .downcast_ref::<gtk::Label>()
                        .unwrap()
                        .text();
                    let opt_circuit_id = me_btn
                        .grid
                        .child_at(Columns::CircuitIds as i32, idx)
                        .unwrap()
                        .downcast_ref::<gtk::ComboBoxText>()
                        .unwrap()
                        .active_id();

                    if let Some(circuit_id) = opt_circuit_id {
                        let mutex = crate::get_tor_controller();
                        let mut ctrl = mutex.lock().unwrap();
                        match ctrl.attach_stream(
                            StreamID(stream_id.as_str().into()),
                            CircuitID(circuit_id.as_str().into()),
                        ) {
                            Ok(text) => log::debug!("OK: {}", text),
                            Err(e) => log::warn!("Error: {}", e),
                        }
                    } else {
                        log::info!("No circuit selected");
                    }
                });
            }

            me.grid.show_all();
        });
        vbox.add(&*Rc::clone(&self.grid));
        vbox.add(&update_btn);

        // Fill table
        update_btn.clicked();
    }

    fn refresh_data(&self) -> Result<(Vec<Circuit>, Vec<Stream>), Error> {
        let gi = GeoIP::new();
        let mutex = crate::get_tor_controller();
        let mut ctrl = mutex.lock().unwrap();
        let mut circuits = ctrl.get_circuits()?;
        let mut circuits_with_country = Vec::with_capacity(circuits.len());
        for c in circuits.drain(..) {
            let last_node = c.path.iter().last().unwrap();
            let or = ctrl.get_onion_router(hex_encode(last_node.fingerprint))?;
            let out_country = match or.target.addr {
                HostOrAddr::Host(_) => None,
                HostOrAddr::Addr(ref addr) => {
                    gi.lookup_ip(addr.clone()).and_then(country::get_country)
                }
            };
            circuits_with_country.push(Circuit {
                circuit: c,
                out_country,
            });
        }
        let streams = ctrl.get_streams()?;
        drop(ctrl);

        Ok((circuits_with_country, streams))
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
