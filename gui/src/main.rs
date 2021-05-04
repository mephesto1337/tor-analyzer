use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::{Arc, Mutex};

use tor_analyzer_lib::prelude::*;

use gio::prelude::*;
use gtk::prelude::*;

macro_rules! add_column {
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

static RT: AtomicPtr<tokio::runtime::Runtime> = AtomicPtr::new(std::ptr::null_mut());
static TOR_CONTROLLER: AtomicPtr<Arc<Mutex<TorController<()>>>> =
    AtomicPtr::new(std::ptr::null_mut());

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Tor Analyzer");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let mut notebook = notebook::Notebook::new();

    window.add(&notebook.notebook);

    notebook.create_tab("Circuits", circuit::create_tab(), false);
    notebook.create_tab("Entry nodes", nodes::create_tab(), false);
    window.show_all();
}

fn get_controller<H>() -> Arc<Mutex<TorController<H>>> {
    let ptr = TOR_CONTROLLER.load(Ordering::Acquire);

    // SAFETY: pointer is not null as we just initialize it in main
    let mutex = unsafe { &*(ptr as *const _) };
    Arc::clone(mutex)
}

fn get_runtime() -> &'static tokio::runtime::Runtime {
    // SAFETY: pointer is not null as we just initialize it in main
    unsafe { RT.load(Ordering::Acquire).as_ref() }.unwrap()
}

fn main() {
    let rt = Box::new(
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Cannot create tokio runtime"),
    );
    RT.store(Box::into_raw(rt), Ordering::Release);

    let mut first_arg = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("127.0.0.1:9051".into())
        .clone();

    get_runtime().block_on(async move {
        let ctrl = TorController::new(first_arg)
            .await
            .expect("Cannot contact Tor Controller");
        ctrl.set_async_event_handler(tor_analyzer_lib::void_async_event_handler);
        TOR_CONTROLLER.store(
            Box::into_raw(Box::new(Arc::new(Mutex::new(ctrl)))) as *mut _,
            Ordering::Release,
        );

        let application =
            gtk::Application::new(Some("local.dev.tor-analyzer-gui"), Default::default()).unwrap();
        application.connect_activate(build_ui);

        application.run(&[]);
    });
}
