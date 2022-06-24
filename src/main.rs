mod keyboard;
use std::rc::{Rc, self};

use keyboard::create_keyboard;
use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, 
    Button, Label, Entry, 
    Orientation, Widget, glib, WindowType, atk
};

const APP_ID: &str = "org.gtk-rs.GObjectMemoryManagement4";
fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    let login = Label::builder()
        .label("Login:")
        .halign(gtk::Align::Start)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    let login_input = Entry::builder()
        .placeholder_text("Type your login")
        .margin_top(10)
        .can_focus(true)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    let password = Label::builder()
        .label("Password:")
        .halign(gtk::Align::Start)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    let password_input = Entry::builder()
        .placeholder_text("Type your password")
        .can_focus(true)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    // Create 'enter' button
    let enter = Button::builder()
        .label("Enter")
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    // Create keyboard
    let keyboard = create_keyboard();
    
    // ANCHOR: box_append
    // Add widgets to `vert_container`
    let vert_container = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .width_request(800)
        .height_request(480)
        .build();
    vert_container.add(&login);
    vert_container.add(&login_input);
    vert_container.add(&password);
    vert_container.add(&password_input);
    vert_container.add(&enter);
    vert_container.add(&keyboard.0);
    // ANCHOR_END: box_append

    for (i, btn) in keyboard.1.into_iter() {
        btn.connect_clicked(glib::clone!(@weak login_input, @weak password_input => move |_| {
            if login_input.is_focus() {
                let str = login_input.text();
                let mut string: String = str.into();
                if keyboard.2.get("caps").unwrap().is_active() {
                    string.push_str(&i.to_string().to_uppercase());
                }
                else if keyboard.2.get("shift").unwrap().is_active() {
                    string.push_str(&i.to_string().to_uppercase());
                    keyboard.2.get("shift").unwrap().activate();
                }
                else {
                    string.push_str(&i.to_string());
                }
                login_input.set_text(&string);
            }
            else if password_input.is_focus() {
                let str = password_input.text();
                let mut string: String = str.into();
                if keyboard.2.get("caps").unwrap().is_active() {
                    string.push_str(&i.to_string().to_uppercase());
                }
                else if keyboard.2.get("shift").unwrap().is_active() {
                    string.push_str(&i.to_string().to_uppercase());
                    keyboard.2.get("shift").unwrap().activate();
                }
                else {
                    string.push_str(&i.to_string());
                }
                password_input.set_text(&string);
            } 
        }));
    } 
    /*keyboard.1[10].connect_clicked(glib::clone!(@weak login_input, @weak password_input => move |_| {
        if login_input.is_focus() {
            let str = login_input.text();
            let mut string: String = str.into();
            string.pop();
            login_input.set_text(&string);
        }
        else if password_input.is_focus() {
            let str = password_input.text();
            let mut string: String = str.into();
            string.pop();
            password_input.set_text(&string);
        }
    }));*/

    /*keyboard.1[11].connect_clicked(glib::clone!(@weak login_input, @weak password_input => move |_| {
        if login_input.is_focus() {
            let str = login_input.text();
            let mut string: String = str.into();
            
            if keyboard.2[0].is_active() {
                string.push('Q');
            }
            else if keyboard.2[1].is_active() {
                string.push('Q');
                keyboard.2[1].activate();
            }
            else {
                string.push('q');
            }
            login_input.set_text(&string);
            
        }
        else if password_input.is_focus() {
            let str = password_input.text();
            let mut string: String = str.into();
            if keyboard.2[0].is_active() {
                string.push('Q');
            }
            else if keyboard.2[1].is_active() {
                string.push('Q');
                keyboard.2[1].activate();
            }
            else {
                string.push('q');
            }
            password_input.set_text(&string);
        }
    }));*/

    
    // Create main window
    let main_window = create_main_window(&app, &vert_container);
    

    enter.connect_clicked(
        glib::clone!(@weak main_window, @weak app => move |_| {
            // When the button is clicked, let's close the main window
            main_window.close();
            create_chat_window(&app);
    }));
    
}

fn create_main_window(application: &Application, child: &impl IsA<Widget>) -> ApplicationWindow {
    let window = ApplicationWindow::new(application);
    window.set_title("QPager");
    //window.fullscreen();
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(800, 480);
    window.set_child(Some(child));
    window.show_all();
    window
}

fn create_chat_window(application: &Application) {
    let chat = gtk::Window::new(WindowType::Toplevel);
    application.add_window(&chat);
    chat.set_title("QPager");
    chat.set_window_position(gtk::WindowPosition::Center);
    //chat.fullscreen();
    chat.set_default_size(800, 480);
    let chat_place = gtk::ScrolledWindow::builder()
        .height_request(480)
        .width_request(200)
        .can_focus(false)
        .build();
    let dialog_place = gtk::ScrolledWindow::builder()
        .height_request(300)
        .width_request(600)
        .can_focus(false)
        .build();
    let message_input = Entry::builder()
        .placeholder_text("Type your message")
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    let hor_container = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .height_request(480)
        .width_request(800)
        .build();
    let vert_container = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .height_request(480)
        .width_request(600)
        .build();
    let keyb = create_keyboard();

    vert_container.add(&dialog_place);
    vert_container.add(&message_input);
    vert_container.add(&keyb.0);
    hor_container.add(&chat_place);
    hor_container.add(&vert_container);
    
    chat.set_child(Some(&hor_container));
    chat.show_all();
}