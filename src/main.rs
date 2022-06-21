mod keyboard;
use keyboard::create_keyboard;
use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, 
    Button, Label, Entry, 
    Orientation, Widget, glib, Align
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
        .build();
    vert_container.append(&login);
    vert_container.append(&login_input);
    vert_container.append(&password);
    vert_container.append(&password_input);
    vert_container.append(&enter);
    vert_container.append(&keyboard);
    // ANCHOR_END: box_append

    // Create main window
    let main_window = create_main_window(app, &vert_container);
    enter.connect_clicked(
        glib::clone!(@weak main_window, @weak app => move |_| {
            // When the button is clicked, let's close the main window
            main_window.close();
            create_chat_window(&app);
    }));    
    
}

fn create_main_window(application: &Application, child: &impl IsA<Widget>) -> ApplicationWindow {
    let window = ApplicationWindow::new(application);
    window.set_title(Some("QPager"));
    window.fullscreen();
    //window.set_default_size(800, 480);
    window.set_child(Some(child));
    window.present();
    window
}

fn create_chat_window(application: &Application) {
    let chat = gtk::Window::new();
    application.add_window(&chat);
    chat.set_title(Some("QPager"));
    chat.fullscreen();
    //chat.set_default_size(800, 480);
    let dialog_place = gtk::ScrolledWindow::builder()
        .height_request(240)
        .width_request(400)
        .can_focus(false)
        .can_target(false)
        .build();
    let message_input = Entry::builder()
        .placeholder_text("Type your message")
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    let vert_container = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .height_request(480)
        .width_request(400)
        .build();
    let keyb = create_keyboard();
    vert_container.append(&dialog_place);
    vert_container.append(&message_input);
    vert_container.append(&keyb);
    
    chat.set_child(Some(&vert_container));
    chat.present();
}