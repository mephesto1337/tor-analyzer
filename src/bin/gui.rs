use std::env;
use std::io;
use std::path::Path;
use std::rc::Rc;

use gio::prelude::*;
use gtk::prelude::*;

use tokio::net::{TcpStream, UnixStream};

use torut::control::{AsyncEvent, ConnError, UnauthenticatedConn};

use tor_analyzer::country;
use tor_analyzer::error::Error;
use tor_analyzer::geoip::GeoIP;
use tor_analyzer::socket::Socket;
use tor_analyzer::tor::circuit::Circuit;
use tor_analyzer::tor::common::Target;
use tor_analyzer::tor::ns::OnionRouter;
use tor_analyzer::tor::stream::Stream;
use tor_analyzer::tor::NomParse;

mod notebook;

#[derive(Debug)]
struct SimpleCircuit {
    id: tor_analyzer::tor::common::CircuitID,
    state: tor_analyzer::tor::circuit::CircuitStatus,
    path: Vec<OnionRouter>,
    endpoint: Option<Target>,
}

#[derive(Debug)]
#[repr(i32)]
enum Columns {
    Id,
    Status,
    Ips,
    Countries,
    Path,
    EndPoint,
    MaxColumns,
}
const FIELD_COUNT: usize = Columns::MaxColumns as usize;

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

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Tor Analyzer");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let mut notebook = notebook::Notebook::new();

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
    window.add(&notebook.notebook);

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
    add_column!(treeview, Columns::Ips, "IPs");
    add_column!(treeview, Columns::Countries, "Countries");
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

    notebook.create_tab("Circuits", vbox.upcast::<gtk::Widget>(), false);
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

    let stream_string = conn.get_info("stream-status").await?;
    let circuits_string = conn.get_info("circuit-status").await?;

    let (_rest, mut circuits) = nom::multi::many1(
        Circuit::parse::<nom::error::VerboseError<&str>>,
    )(circuits_string.as_str())?;

    let (_, streams) =
        nom::multi::many0(Stream::parse::<nom::error::VerboseError<&str>>)(stream_string.as_str())?;

    let mut simple_circuits = Vec::with_capacity(circuits.len());
    for mut c in circuits.drain(..) {
        let mut path = Vec::with_capacity(c.path.len());

        for mut step in c.path.drain(..) {
            step.nickname = None;
            let or_str = conn.get_info(&format!("ns/id/${}", step)).await?;
            let (_, or) = OnionRouter::parse::<nom::error::VerboseError<&str>>(or_str.as_str())?;
            path.push(or);
        }

        let endpoint = streams.iter().find_map(|s| {
            if s.circuit_id == c.id {
                Some(s.target.clone())
            } else {
                None
            }
        });

        simple_circuits.push(SimpleCircuit {
            id: c.id,
            state: c.status,
            path,
            endpoint,
        })
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
                Err(e) => eprintln!("Error: {}", e),
            }
        });

    circuits
}

fn localize_target(writer: &mut String, target: &Target, gi: &GeoIP) {
    if let Some(loc) = gi.lookup_ip(target.addr) {
        if let Some(country) = country::get_country(loc) {
            writer.push_str(country.flag);
            writer.push(' ');
            writer.push_str(country.name);
        } else {
            writer.push_str(loc);
        }
    }
}

fn update_model(store: &gtk::ListStore) {
    let data = get_circuits();
    let gi = tor_analyzer::geoip::GeoIP::new();

    store.clear();

    let mut indexes = [0u32; FIELD_COUNT];
    for (i, idx) in indexes.iter_mut().enumerate() {
        *idx = i as u32;
    }
    for d in data.iter() {
        #[cfg(debug_assertions)]
        eprintln!("Got circuit: {:?}", d);
        let id = format!("{}", d.id);
        let state = format!("{}", d.state);
        let mut paths = String::new();
        let mut ips = String::new();
        let mut countries = String::new();
        let mut first = true;
        for p in &d.path {
            if !first {
                countries.push('\n');
                paths.push('\n');
                ips.push('\n');
            }
            first = false;
            localize_target(&mut countries, &p.target, &gi);
            ips.push_str(&format!("{}", p.target));
            paths.push_str(&format!("{}", p));
        }
        let endpoint = if let Some(ref ep) = d.endpoint {
            let mut endpoint = format!("{} ", ep);
            localize_target(&mut endpoint, ep, &gi);
            endpoint
        } else {
            String::new()
        };

        let values: [&dyn ToValue; FIELD_COUNT] =
            [&id, &state, &ips, &countries, &paths, &endpoint];
        store.set(&store.append(), &indexes[..], &values);
    }
}

fn main() {
    let application =
        gtk::Application::new(Some("local.dev.tor-analyzer-gui"), Default::default()).unwrap();
    application.connect_activate(build_ui);

    application.run(&[]);
}
