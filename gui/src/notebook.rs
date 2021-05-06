#![allow(unused_attributes)]
#![no_main]

use glib;
use gtk::prelude::*;
use gtk::{IconSize, Orientation, ReliefStyle, Widget};

pub struct Notebook {
    pub notebook: gtk::Notebook,
    tabs: Vec<gtk::Box>,
    accels: gtk::AccelGroup,
}

fn find_child<T: StaticType>(widget: gtk::Widget) -> Option<gtk::Widget> {
    if widget.is::<T>() {
        return Some(widget);
    }

    if widget.is::<gtk::Container>() {
        let container = widget.downcast_ref::<gtk::Container>().unwrap();
        for c in container.get_children() {
            let t = find_child::<T>(c);
            if t.is_some() {
                return t;
            }
        }
    }
    None
}

impl Notebook {
    pub fn new() -> Self {
        let notebook = gtk::Notebook::new();
        notebook.connect_change_current_page(|notebook, idx| {
            if idx != 0 {
                let count = notebook.get_n_pages() as i32;
                if let Some(current) = notebook.get_current_page() {
                    let new = current as i32 + idx;
                    if new >= 0 && new < count {
                        if idx < 0 {
                            notebook.next_page();
                        } else {
                            notebook.prev_page();
                        }
                    } else {
                        if idx < 0 {
                            notebook.set_current_page(Some(0));
                        } else {
                            notebook.set_current_page(Some(count as u32 - 1));
                        }
                    }
                }
            }
            let new = notebook.get_nth_page(notebook.get_current_page()).unwrap();
            if let Some(entry) = find_child::<gtk::Entry>(new) {
                entry.grab_focus();
            }

            true
        });
        /*
        notebook.connect_grab_focus(|widget| {
            let notebook = widget.upcast_ref::<gtk::Notebook>();
            let page = match notebook.get_nth_page(notebook.get_current_page()) {
                Some(p) => p,
                None => return,
            };
            let tab = page.downcast_ref::<gtk::Box>().expect("Not a GtkBox");
            for c in tab.get_children() {
                if c.is::<gtk::Entry>() {
                    c.grab_focus();
                    let entry = c.downcast_ref::<gtk::Entry>().unwrap();
                    entry.set_position(0);
                    break;
                }
            }
        });
        */
        let accels = gtk::AccelGroup::new();
        Self {
            notebook,
            tabs: Vec::new(),
            accels,
        }
    }

    pub fn create_tab(&mut self, title: &str, widget: Widget, closable: bool) -> u32 {
        let close_image = gtk::Image::from_icon_name(Some("window-close"), IconSize::Button);
        // let label = gtk::Button::with_label(title);
        let label = gtk::Label::new(Some(title));

        let tab = gtk::Box::new(Orientation::Horizontal, 0);
        let index = self.notebook.append_page(&widget, Some(&tab));
        // label.connect_clicked(|b| {
        //     let parent = b.get_parent().unwrap().get_parent().unwrap();
        //     let notebook = parent.downcast_ref::<gtk::Notebook>().unwrap();

        //     notebook.set_current_page(Some(0));
        // });

        if closable {
            let button = gtk::Button::new();
            button.set_relief(ReliefStyle::None);
            button.add(&close_image);
            let (key, mod_) = gtk::accelerator_parse("<ctrl>w");
            button.add_accelerator("clicked", &self.accels, key, mod_, gtk::AccelFlags::VISIBLE);

            button.connect_clicked(glib::clone!(@weak self.notebook as notebook => move |_| {
                let index = notebook
                    .page_num(&widget)
                    .expect("Couldn't get page_num from notebook");
                notebook.remove_page(Some(index));
            }));
            tab.pack_start(&button, false, false, 0);
        }

        tab.pack_start(&label, false, false, 0);
        tab.show_all();

        self.tabs.push(tab);

        eprintln!("{}: {}", index, title);
        index
    }
}
