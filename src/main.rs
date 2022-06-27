mod keyboard;

use keyboard::create_keyboard;
use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, 
    Button, Label, Entry, 
    Orientation, Widget, glib, WindowType
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
        .visibility(false)
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
    let keyboard_grid = keyboard.0;
    let keyboard_buttons = keyboard.1;
    let keyboard_special_buttons = keyboard.2;
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
    vert_container.add(&keyboard_grid);
    // ANCHOR_END: box_append

    for (i, btn) in keyboard_buttons.into_iter() {
        if i == '<' {
            btn.connect_clicked(glib::clone!(@strong keyboard_special_buttons, @weak login_input, @weak password_input => move |_| {
                if login_input.is_focus() {
                    let str = login_input.text();
                    let mut string: String = str.into();
                    string.pop();
                    login_input.set_text(&string);
                    login_input.emit_move_cursor(gtk::MovementStep::BufferEnds, 1, false);
                }
                else if password_input.is_focus() {
                    let str = password_input.text();
                    let mut string: String = str.into();
                    string.pop();
                    password_input.set_text(&string);
                    password_input.emit_move_cursor(gtk::MovementStep::BufferEnds, 1, false);
                } 
            }));
        }
        else if i == '>' {
            btn.connect_clicked(glib::clone!(@weak login_input, @weak password_input => move |_| {
                if login_input.is_focus() {
                    password_input.set_is_focus(true);
                }
            }));
        }
        else {
            btn.connect_clicked(glib::clone!(@strong keyboard_special_buttons, @weak login_input, @weak password_input => move |_| {
                if login_input.is_focus() {
                    let str = login_input.text();
                    let mut string: String = str.into();
                    if keyboard_special_buttons.get("caps").unwrap().is_active() {
                        string.push_str(&i.to_string().to_uppercase());
                    }
                    else if keyboard_special_buttons.get("shift").unwrap().is_active() {
                        string.push_str(&i.to_string().to_uppercase());
                        keyboard_special_buttons.get("shift").unwrap().activate();
                    }
                    else {
                        string.push_str(&i.to_string());
                    }
                    login_input.set_text(&string);
                    login_input.emit_move_cursor(gtk::MovementStep::BufferEnds, 1, false);
                }
                else if password_input.is_focus() {
                    let str = password_input.text();
                    let mut string: String = str.into();
                    if keyboard_special_buttons.get("caps").unwrap().is_active() {
                        string.push_str(&i.to_string().to_uppercase());
                    }
                    else if keyboard_special_buttons.get("shift").unwrap().is_active() {
                        string.push_str(&i.to_string().to_uppercase());
                        keyboard_special_buttons.get("shift").unwrap().activate();
                    }
                    else {
                        string.push_str(&i.to_string());
                    }
                    password_input.set_text(&string);
                    password_input.emit_move_cursor(gtk::MovementStep::BufferEnds, 1, false);
                } 
            }));
        }
        
    }
    
    
    
    // Create main window
    let main_window = create_main_window(&app, &vert_container);
    

    enter.connect_clicked(
        glib::clone!(@weak main_window, @weak app => move |_| {
            // When the button is clicked, let's close the main window
            println!("Login: {}", login_input.text().to_string());
            println!("Password: {}", password_input.text().to_string());
            main_window.close();
            create_chat_window(&app, login_input.text().to_string());
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

fn create_chat_window(application: &Application, login: String) {
    let chat = gtk::Window::new(WindowType::Toplevel);
    application.add_window(&chat);
    chat.set_title("QPager");
    chat.set_window_position(gtk::WindowPosition::Center);
    //chat.fullscreen();
    chat.set_default_size(800, 480);
    
    let chat_box = gtk::ListBox::new();
    let chat_place = gtk::ScrolledWindow::builder()
        .height_request(480)
        .width_request(200)
        .can_focus(false)
        .child(&chat_box)
        .build();
    let dialog_box = gtk::ListBox::builder().valign(gtk::Align::End).build();
    let dialog_place = gtk::ScrolledWindow::builder()
        .height_request(300)
        .width_request(600)
        .can_focus(false)
        .child(&dialog_box)
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
    let keyboard_grid = keyb.0;
    let keyboard_buttons = keyb.1;
    let keyboard_special_buttons = keyb.2;

    vert_container.add(&dialog_place);
    vert_container.add(&message_input);
    vert_container.add(&keyboard_grid);
    hor_container.add(&chat_place);
    hor_container.add(&vert_container);
    for (i, btn) in keyboard_buttons.into_iter() {
        if i == '<' {
            btn.connect_clicked(glib::clone!(@strong keyboard_special_buttons, @weak message_input => move |_| {
                if message_input.is_focus() {
                    let str = message_input.text();
                    let mut string: String = str.into();
                    string.pop();
                    message_input.set_text(&string);
                    message_input.emit_move_cursor(gtk::MovementStep::BufferEnds, 1, false);
                }
            }));
        }
        else if i == '>' {
            btn.connect_clicked(glib::clone!(@strong login, @weak dialog_box, @weak dialog_place, @weak message_input => move |_| {
                let label = Label::builder().label(&message_input.text().to_string()).build();
                let login_label = Label::builder().label(&login).margin_end(5).build();
                let cont = gtk::Box::builder().orientation(Orientation::Horizontal).halign(gtk::Align::End).build();
                cont.add(&login_label);
                cont.add(&label);
                dialog_box.add(&cont);
                let len: i32 = message_input.text().len().try_into().unwrap();
                message_input.delete_text(0, len);
                dialog_place.show_all();
            }));
        }
        else {
            btn.connect_clicked(glib::clone!(@strong keyboard_special_buttons, @weak message_input => move |_| {
                if message_input.is_focus() {
                    let str = message_input.text();
                    let mut string: String = str.into();
                    if keyboard_special_buttons.get("caps").unwrap().is_active() {
                        string.push_str(&i.to_string().to_uppercase());
                    }
                    else if keyboard_special_buttons.get("shift").unwrap().is_active() {
                        string.push_str(&i.to_string().to_uppercase());
                        keyboard_special_buttons.get("shift").unwrap().activate();
                    }
                    else {
                        string.push_str(&i.to_string());
                    }
                    message_input.set_text(&string);
                    message_input.emit_move_cursor(gtk::MovementStep::BufferEnds, 1, false);
                }
            }));
        }
    }
    chat.set_child(Some(&hor_container));
    chat.show_all();
}