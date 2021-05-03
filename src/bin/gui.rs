use std::env;
use std::io;
use std::net::IpAddr;
use std::path::Path;
use std::rc::Rc;

use gio::prelude::*;
use gtk::prelude::*;

use tokio::net::{TcpStream, UnixStream};

use torut::control::{AsyncEvent, ConnError, UnauthenticatedConn};

use tor_analyzer::error::Error;
use tor_analyzer::socket::Socket;
use tor_analyzer::tor::circuit::Circuit;
use tor_analyzer::tor::ns::OnionRouter;
use tor_analyzer::tor::NomParse;

const FIELD_COUNT: usize = 4;

#[derive(Debug)]
struct SimpleCircuit {
    id: tor_analyzer::tor::circuit::CircuitID,
    state: tor_analyzer::tor::circuit::CircuitStatus,
    path: Vec<OnionRouter>,
    endpoint: (IpAddr, u16),
}

#[derive(Debug)]
#[repr(i32)]
enum Columns {
    Id,
    Status,
    Path,
    EndPoint,
}

macro_rules! add_column {
    ($treeview:expr, $variant:expr, $name:expr) => {{
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title($name);
        column.add_attribute(&renderer, "text", $variant as i32);
        column.set_sort_column_id($variant as i32);
        $treeview.append_column(&column);
    }};
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Tor Analyzer");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
    window.add(&vbox);

    let sw = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    sw.set_shadow_type(gtk::ShadowType::EtchedIn);
    sw.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    vbox.add(&sw);

    let col_types = [glib::Type::String; FIELD_COUNT];
    let store = Rc::new(gtk::ListStore::new(&col_types));

    let treeview = gtk::TreeView::with_model(&*store.clone());
    treeview.set_vexpand(true);
    treeview.set_search_column(Columns::Id as i32);

    sw.add(&treeview);
    add_column!(treeview, Columns::Id, "Id");
    add_column!(treeview, Columns::Status, "Status");
    add_column!(treeview, Columns::Path, "Path");
    add_column!(treeview, Columns::EndPoint, "End point");

    let update_btn = gtk::Button::with_label("Update");
    update_btn.connect_clicked(move |_| {
        let store = store.clone();
        update_model(&*store);
    });
    vbox.add(&update_btn);

    // Fill table
    update_btn.clicked();

    window.show_all();
}

async fn event_handler(_event: AsyncEvent<'static>) -> Result<(), ConnError> {
    Ok(())
}

async fn async_get_circuits() -> Result<Vec<SimpleCircuit>, Error> {
    let socket: Socket = match env::args().skip(1).next() {
        Some(a) => {
            let path = Path::new(a.as_str());
            if path.exists() {
                UnixStream::connect(a).await?.into()
            } else {
                TcpStream::connect(a).await?.into()
            }
        }
        None => TcpStream::connect("127.0.0.1:9051").await?.into(),
    };
    let mut anon_conn = UnauthenticatedConn::new(socket);

    let infos = anon_conn.load_protocol_info().await?;
    let auth_data = match infos.make_auth_data()? {
        Some(data) => data,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Cannot authenticate, maybe HashPassword is missing?",
            )
            .into());
        }
    };

    anon_conn.authenticate(&auth_data).await?;
    let mut conn = anon_conn.into_authenticated().await;
    conn.set_async_event_handler(Some(event_handler));

    let circuits_string = conn.get_info("circuit-status").await?;
    let (_rest, mut circuits) = nom::multi::many1(
        Circuit::parse::<nom::error::VerboseError<&str>>,
    )(circuits_string.as_str())?;

    let mut simple_circuits = Vec::with_capacity(circuits.len());
    for mut c in circuits.drain(..) {
        let mut path = Vec::with_capacity(c.path.len());

        for mut step in c.path.drain(..) {
            step.nickname = None;
            let or_str = conn.get_info(&format!("ns/id/${}", step)).await?;
            let (_, or) = OnionRouter::parse::<nom::error::VerboseError<&str>>(or_str.as_str())?;
            path.push(or);
        }

        simple_circuits.push(dbg!(SimpleCircuit {
            id: c.id,
            state: c.status,
            path,
            endpoint: (IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)), 0),
        }))
    }

    Ok(simple_circuits)
}

fn get_circuits() -> Vec<SimpleCircuit> {
    let mut circuits = Vec::new();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            match async_get_circuits().await {
                Ok(mut c) => circuits.extend(c.drain(..)),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        });

    circuits
}

macro_rules! to_gtk_value {
    ($displayable:expr) => {
        format!("{}", $displayable)
    };
    (Pair $displayable:expr, $sep:expr) => {{
        let (a, b) = $displayable;
        format!("{}{}{}", a, $sep, b)
    }};
    (Option $displayable:expr) => {
        if let Some(ref disp) = $displayable {
            format!("{}", disp)
        } else {
            String::new()
        }
    };
    (Vec $displayable:expr) => {{
        let mut s = String::from("[");
        let mut first = true;
        for item in $displayable.iter() {
            if first {
                s.push_str(&format!("{}", item));
            } else {
                s.push_str(&format!(", {}", item));
            }
            first = false;
        }
        s.push(']');
        s
    }};
}

fn update_model(store: &gtk::ListStore) {
    let data = get_circuits();

    store.clear();

    let mut indexes = [0u32; FIELD_COUNT];
    for (i, idx) in indexes.iter_mut().enumerate() {
        *idx = i as u32;
    }
    for d in data.iter() {
        // #[cfg(debug_assertions)]
        eprintln!("Got circuit: {:?}", d);
        let values: [&dyn ToValue; FIELD_COUNT] = [
            &to_gtk_value!(d.id),
            &to_gtk_value!(d.state),
            &to_gtk_value!(Vec d.path),
            &to_gtk_value!(Pair d.endpoint, ':'),
        ];
        store.set(&store.append(), &indexes[..], &values);
    }
}

fn main() {
    let application =
        gtk::Application::new(Some("local.dev.tor-analyzer-gui"), Default::default()).unwrap();
    application.connect_activate(build_ui);

    application.run(&[]);
}
