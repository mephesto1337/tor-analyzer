use std::sync::{Arc, Mutex};

use tor_analyzer_lib::prelude::*;

use gio::prelude::*;
use gtk::prelude::*;

macro_rules! add_column {
    (bool $treeview:expr, $variant:expr, $name:expr) => {{
        let renderer = gtk::CellRendererToggle::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title($name);
        column.add_attribute(&renderer, "active", $variant as i32);
        column.set_sort_column_id($variant as i32);
        column.set_sort_indicator(true);
        column.set_clickable(true);
        column.set_fixed_width(70);
        column.set_sizing(gtk::TreeViewColumnSizing::Fixed);
        $treeview.append_column(&column);
        column
    }};
    ($treeview:expr, $variant:expr, $name:expr) => {{
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title($name);
        column.add_attribute(&renderer, "text", $variant as i32);
        column.set_sort_column_id($variant as i32);
        column.set_sort_indicator(true);
        column.set_clickable(true);
        $treeview.append_column(&column);
        column
    }};
}

mod circuit;
mod nodes;
mod notebook;

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
        &window.get_screen().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let mut notebook = notebook::Notebook::new();

    window.add(&notebook.notebook);

    notebook.create_tab("Circuits", circuit::create_tab(), false);
    notebook.create_tab("Entry nodes", nodes::create_tab(), false);
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
    for i in 0..model.get_n_columns() {
        let value = model.get_value(iter, i);
        if value.is::<String>() {
            if let Some(v) = value.get::<String>().unwrap() {
                text.push_str(v.to_lowercase().as_str());
                text.push(' ');
            }
        }
    }
    let show = text.contains(filter.as_str());
    show
}

fn main() {
    let first_arg = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("127.0.0.1:9051".into())
        .clone();

    let ctrl = TorController::new(first_arg).expect("Cannot contact Tor Controller");
    unsafe {
        TOR_CONTROLLER = Some(Arc::new(Mutex::new(ctrl)));
    }

    let application =
        gtk::Application::new(Some("local.dev.tor-analyzer-gui"), Default::default()).unwrap();
    application.connect_activate(build_ui);

    application.run(&[]);
}
