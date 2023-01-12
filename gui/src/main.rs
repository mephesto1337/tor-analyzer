use std::rc::Rc;
use std::sync::{Arc, Mutex};

use tor_analyzer_lib::prelude::*;

use gtk::prelude::*;

macro_rules! add_column {
    (bool $treeview:expr, $variant:expr, $name:expr) => {{
        let renderer = gtk::CellRendererToggle::new();
        let column = gtk::TreeViewColumn::new();
        gtk::prelude::TreeViewColumnExt::pack_start(&column, &renderer, true);
        gtk::prelude::TreeViewColumnExt::set_title(&column, $name);
        gtk::prelude::TreeViewColumnExt::add_attribute(
            &column,
            &renderer,
            "active",
            $variant as i32,
        );
        gtk::prelude::TreeViewColumnExt::set_sort_column_id(&column, $variant as i32);
        gtk::prelude::TreeViewColumnExt::set_sort_indicator(&column, true);
        gtk::prelude::TreeViewColumnExt::set_clickable(&column, true);
        gtk::prelude::TreeViewColumnExt::set_fixed_width(&column, 70);
        gtk::prelude::TreeViewColumnExt::set_sizing(&column, gtk::TreeViewColumnSizing::Fixed);
        $treeview.append_column(&column);
        column
    }};
    ($treeview:expr, $variant:expr, $name:expr) => {{
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        gtk::prelude::TreeViewColumnExt::pack_start(&column, &renderer, true);
        gtk::prelude::TreeViewColumnExt::set_title(&column, $name);
        gtk::prelude::TreeViewColumnExt::add_attribute(&column, &renderer, "text", $variant as i32);
        gtk::prelude::TreeViewColumnExt::set_sort_column_id(&column, $variant as i32);
        gtk::prelude::TreeViewColumnExt::set_sort_indicator(&column, true);
        gtk::prelude::TreeViewColumnExt::set_clickable(&column, true);
        $treeview.append_column(&column);
        column
    }};
}
macro_rules! popup_error {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        log::error!($($arg)*);
        let window = gtk::MessageDialog::new(
            None::<&gtk::ApplicationWindow>,
            gtk::DialogFlags::empty(),
            gtk::MessageType::Error,
            gtk::ButtonsType::Ok,
            message.as_str(),
        );
        window.connect_response(|message_dialog, _response_type| unsafe {
            message_dialog.destroy();
        });
        window.show();
    }};
}

mod build_circuit;
mod circuit;
mod nodes;
mod notebook;
mod stream;

use notebook::NotebookTab;

static mut TOR_CONTROLLER: Option<Arc<Mutex<TorController>>> = None;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Tor Analyzer");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let provider = gtk::CssProvider::new();
    // Load the CSS file
    let style = include_bytes!("style.css");
    provider.load_from_data(style).expect("Failed to load CSS");
    // We give the CssProvided to the default screen so the CSS rules we added
    // can be applied to our window.
    gtk::StyleContext::add_provider_for_screen(
        &gtk::prelude::WidgetExt::screen(&window).unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let mut notebook = notebook::Notebook::new();

    window.add(&notebook.notebook);

    // notebook.create_tab("Circuits", circuit::create_tab(), false);
    let circuits = circuit::CircuitTab::new();
    notebook.create_tab(&*circuits);

    let nodes = nodes::NodeTab::new();
    notebook.create_tab(&*nodes);

    let builder = build_circuit::CircuitTab::new();
    notebook.create_tab(&*builder);

    let streams = stream::StreamTab::new();
    notebook.create_tab(&*streams);

    nodes.set_circuit_tab(Rc::clone(&builder));

    window.show_all();
}

fn get_tor_controller() -> Arc<Mutex<tor_analyzer_lib::TorController>> {
    Arc::clone(unsafe { TOR_CONTROLLER.as_ref() }.unwrap())
}

fn filter_func(filter: String, model: &gtk::TreeModel, iter: &gtk::TreeIter) -> bool {
    if filter.is_empty() {
        return true;
    }

    let mut text = String::new();
    for i in 0..model.n_columns() {
        let value = model.value(iter, i);
        if value.is::<String>() {
            if let Ok(v) = value.get::<String>() {
                text.push_str(v.to_lowercase().as_str());
                text.push(' ');
            }
        }
    }
    let show = text.contains(filter.as_str());
    show
}

fn main() -> Result<(), tor_analyzer_lib::error::Error> {
    env_logger::init();
    let first_arg = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9051".into());

    let mut ctrl = TorController::new(first_arg)?;
    ctrl.set_conf("__LeaveStreamsUnattached", Some(1))
        .expect("Cannot change config");
    unsafe {
        TOR_CONTROLLER = Some(Arc::new(Mutex::new(ctrl)));
    }

    let application = gtk::Application::new(
        Some("local.dev.tor-analyzer-gui"),
        gio::ApplicationFlags::FLAGS_NONE,
    );
    application.connect_activate(build_ui);

    // popup_error!("hello world");
    application.run_with_args(&[""][..]);
    let _ = get_tor_controller()
        .lock()
        .unwrap()
        .set_conf("__LeaveStreamsUnattached", Some(0));
    Ok(())
}
