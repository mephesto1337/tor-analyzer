#![allow(unused_attributes)]
#![no_main]

use std::rc::Rc;

use glib;
use gtk::prelude::*;
use gtk::{IconSize, Orientation, ReliefStyle};

pub struct Notebook {
    pub notebook: gtk::Notebook,
    tabs: Vec<gtk::Box>,
    accels: gtk::AccelGroup,
}

pub trait NotebookTab {
    fn get_widget(&self) -> Rc<gtk::Widget>;

    fn label(&self) -> &'static str;

    fn is_closable(&self) -> bool {
        false
    }
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
        notebook.connect_switch_page(|_notebook, page, _idx| {
            if let Some(entry) = find_child::<gtk::Entry>(page.clone()) {
                entry.grab_focus();
            }
        });
        let accels = gtk::AccelGroup::new();
        Self {
            notebook,
            tabs: Vec::new(),
            accels,
        }
    }

    pub fn create_tab(&mut self, nbt: &dyn NotebookTab) -> u32 {
        let close_image = gtk::Image::from_icon_name(Some("window-close"), IconSize::Button);
        // let label = gtk::Button::with_label(title);
        let label = gtk::Label::new(Some(nbt.label()));

        let widget = nbt.get_widget();
        let tab = gtk::Box::new(Orientation::Horizontal, 0);
        let index = self.notebook.append_page(&*widget, Some(&tab));
        // label.connect_clicked(|b| {
        //     let parent = b.get_parent().unwrap().get_parent().unwrap();
        //     let notebook = parent.downcast_ref::<gtk::Notebook>().unwrap();

        //     notebook.set_current_page(Some(0));
        // });

        if nbt.is_closable() {
            let button = gtk::Button::new();
            button.set_relief(ReliefStyle::None);
            button.add(&close_image);
            let (key, mod_) = gtk::accelerator_parse("<ctrl>w");
            button.add_accelerator("clicked", &self.accels, key, mod_, gtk::AccelFlags::VISIBLE);

            button.connect_clicked(glib::clone!(@weak self.notebook as notebook => move |_| {
                let index = notebook
                    .page_num(&*widget)
                    .expect("Couldn't get page_num from notebook");
                notebook.remove_page(Some(index));
            }));
            tab.pack_start(&button, false, false, 0);
        }

        tab.pack_start(&label, false, false, 0);
        tab.show_all();

        self.tabs.push(tab);

        log::debug!("{}: {}", index, nbt.label());
        index
    }
}
