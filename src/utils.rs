use uuid::Uuid;
use gtk4::prelude::{AppInfoExt, WidgetExt, BoxExt};
use gtk4 as gtk;
use gtk::{gio, Image, IconLookupFlags, IconTheme, Box, TextDirection};
use std::process::Command;
use regex::Regex;
#[derive(Debug, Clone)]
pub struct AppField {
    pub app_name: String,
    pub exec: String,
    pub id: Uuid,
}

impl AppField {
    pub fn new() -> Self {
        Self {
            app_name: String::new(),
            exec: String::new(),
            id: Uuid::new_v4(),
        }
    }

    pub fn update_fields(&mut self, app_name: String, exec: String) {
        self.app_name = app_name;
        self.exec = exec;
        self.id = Uuid::new_v4();
    }
}

pub fn string_to_command(input: &str) {
    // this will take the string, to lowercase, and then remove any spaces with split(' ')
    let fms_str = &input.trim().to_lowercase();

    println!("the string that is formatted= {:?}", &fms_str);
    let mut echo_command = Command::new(&fms_str)
        .spawn()
        .expect("something went wrong trying to read the command");
    let hello = echo_command.stdout;
}

// this shouldn't be used!!!! but im stashing it c:
pub fn hash_match_and_launch_app(
    widget: gtk4::Widget, 
    hash: &std::collections::HashMap<gtk4::Box, gio::AppInfo>) {
     let query_child = &widget;
     let hashed_child = hash.contains_key(query_child);
     let captured_app = hash.get(query_child).unwrap();
     let launch_app = gio::AppInfo::launch(
         &captured_app, 
         &[], 
         gio::AppLaunchContext::NONE);
}

// fix this, this isn't working all the time
pub fn prepend_box_if_matches(
    user_text: String, 
    app_name: String,
    rbox: &gtk::Box,
    lbox: &gtk::ListBox
    ) {
    let app_label = gtk::Label::new(Some(&app_name));
    // don't be scared, this is regex.
    // It looks for any matching characters 
    // between user_text and app_name
    let pattern = Regex::new(r"/\D/gm").unwrap(); 
    for item in pattern.find_iter(&user_text) {
        println!("YOU GOT A MATCH ***********{}", &app_name);
        rbox.prepend(&app_label);
        lbox.prepend(rbox);
    }

}

