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

impl Notebook {
    pub fn new() -> Self {
        let notebook = gtk::Notebook::new();
        notebook.connect_change_current_page(|notebook, idx| {
            eprintln!("Event: {}", idx);
            if idx > 0 {
                notebook.next_page();
            } else {
                notebook.prev_page();
            }
            true
        });
        let (key, mod_) = gtk::accelerator_parse("<ctrl>n");
        let accels = gtk::AccelGroup::new();
        notebook.add_accelerator(
            "change-current-page",
            &accels,
            key,
            mod_,
            gtk::AccelFlags::VISIBLE,
        );
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

        index
    }
}
