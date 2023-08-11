use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, SearchBar, Image };

use crate::input::{self, compare_inputs};

pub fn draw_ui(app: &Application) {

    let apps = gio::AppInfo::all();

    let hbox = gtk::ListBox::new();

    let draw_window = ApplicationWindow::new(app);

    let search_bar = SearchBar::new();  //TODO! Need to add input box. Styling is broken.
    search_bar.set_margin(10);

    // handle search event. Searchbar.handle_event(); takes 1 arg

    let text = gtk::Label::new(None); // TODO! This needs to be figured out. Maybe not use a label
    // at all.
    
    for app in apps {
       let icon_with_name = gtk::ListBoxRow::new();
       let name = app.display_name();
       let icon = app.icon(); // TODO!
       let image_container = Image::from_icon_name(Some(&name), gtk::IconSize::SmallToolbar);
       let title = gtk::Label::new(Some(&name));

       icon_with_name.add(&image_container);
       icon_with_name.add(&title);

       
       hbox.add(&icon_with_name);
       hbox.add(&title);    
    }

    hbox.add(&text);
    hbox.add(&search_bar); 

    draw_window.add(&hbox);
    draw_window.set_size_request(300, 300);
    draw_window.set_keep_above(true);
    draw_window.show_all();
}

pub fn run_rs() {
    let _applications = gio::AppInfo::all();
}
