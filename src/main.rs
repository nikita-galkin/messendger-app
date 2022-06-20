use glib::clone;
use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Button, Label, Entry, Orientation};

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
    // Create button
    let button = Button::builder()
        .label("Enter")
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    // Create keyboard
    let button_0 = Button::builder()
        .label("0")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_1 = Button::builder()
        .label("1")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_2 = Button::builder()
        .label("2")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_3 = Button::builder()
        .label("3")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_4 = Button::builder()
        .label("4")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_5 = Button::builder()
        .label("5")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_6 = Button::builder()
        .label("6")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_7 = Button::builder()
        .label("7")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_8 = Button::builder()
        .label("8")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_9 = Button::builder()
        .label("9")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_del = Button::builder()
        .label("Del")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    let button_Q = Button::builder()
        .label("Q")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_W = Button::builder()
        .label("W")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_E = Button::builder()
        .label("E")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_R = Button::builder()
        .label("R")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_T = Button::builder()
        .label("T")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_Y = Button::builder()
        .label("Y")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_U = Button::builder()
        .label("U")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_I = Button::builder()
        .label("I")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_O = Button::builder()
        .label("O")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_P = Button::builder()
        .label("P")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    let button_A = Button::builder()
        .label("A")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_S = Button::builder()
        .label("S")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_D = Button::builder()
        .label("D")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_F = Button::builder()
        .label("F")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_G = Button::builder()
        .label("G")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_H = Button::builder()
        .label("H")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_J = Button::builder()
        .label("J")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_K = Button::builder()
        .label("K")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_L = Button::builder()
        .label("L")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_caps = Button::builder()
        .label("caps lock")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    let button_Z = Button::builder()
        .label("Z")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_X = Button::builder()
        .label("X")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_C = Button::builder()
        .label("C")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_V = Button::builder()
        .label("V")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_B = Button::builder()
        .label("B")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_N = Button::builder()
        .label("N")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_M = Button::builder()
        .label("M")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_shift = Button::builder()
        .label("shift")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_excl = Button::builder()
        .label("!")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_quest = Button::builder()
        .label("?")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_coma = Button::builder()
        .label(",")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_dot = Button::builder()
        .label(".")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_space = Button::builder()
        .label("Space")
        .width_request(400)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_alt = Button::builder()
        .label("alt")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    let button_ctrl = Button::builder()
        .label("ctrl")
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
    // ANCHOR: box_append
    // Add widgets to `hor_container_1`
    let hor_container_1 = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    hor_container_1.append(&button_1);
    hor_container_1.append(&button_2);
    hor_container_1.append(&button_3);
    hor_container_1.append(&button_4);
    hor_container_1.append(&button_5);
    hor_container_1.append(&button_6);
    hor_container_1.append(&button_7);
    hor_container_1.append(&button_8);
    hor_container_1.append(&button_9);
    hor_container_1.append(&button_0);
    hor_container_1.append(&button_del);
    // ANCHOR_END: box_append

    // ANCHOR: box_append
    // Add widgets to `hor_container_2`
    let hor_container_2 = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    hor_container_2.append(&button_Q);
    hor_container_2.append(&button_W);
    hor_container_2.append(&button_E);
    hor_container_2.append(&button_R);
    hor_container_2.append(&button_T);
    hor_container_2.append(&button_Y);
    hor_container_2.append(&button_U);
    hor_container_2.append(&button_I);
    hor_container_2.append(&button_O);
    hor_container_2.append(&button_P);
    // ANCHOR_END: box_append

    // ANCHOR: box_append
    // Add widgets to `hor_container_3`
    let hor_container_3 = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    hor_container_3.append(&button_caps);
    hor_container_3.append(&button_A);
    hor_container_3.append(&button_S);
    hor_container_3.append(&button_D);
    hor_container_3.append(&button_F);
    hor_container_3.append(&button_G);
    hor_container_3.append(&button_H);
    hor_container_3.append(&button_J);
    hor_container_3.append(&button_K);
    hor_container_3.append(&button_L);
    // ANCHOR_END: box_append

    // ANCHOR: box_append
    // Add widgets to `hor_container_4`
    let hor_container_4 = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    hor_container_4.append(&button_shift);
    hor_container_4.append(&button_Z);
    hor_container_4.append(&button_X);
    hor_container_4.append(&button_C);
    hor_container_4.append(&button_V);
    hor_container_4.append(&button_B);
    hor_container_4.append(&button_N);
    hor_container_4.append(&button_M);
    hor_container_4.append(&button_excl);
    hor_container_4.append(&button_quest);
    // ANCHOR_END: box_append

    // ANCHOR: box_append
    // Add widgets to `hor_container_5`
    let hor_container_5 = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    hor_container_5.append(&button_ctrl);
    hor_container_5.append(&button_alt);
    hor_container_5.append(&button_space);
    hor_container_5.append(&button_dot);
    hor_container_5.append(&button_coma);
    // ANCHOR_END: box_append

    // ANCHOR: box_append
    // Add widgets to `vert_container`
    let vert_container = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    vert_container.append(&login);
    vert_container.append(&login_input);
    vert_container.append(&password);
    vert_container.append(&password_input);
    vert_container.append(&button);
    vert_container.append(&hor_container_1);
    vert_container.append(&hor_container_2);
    vert_container.append(&hor_container_3);
    vert_container.append(&hor_container_4);
    vert_container.append(&hor_container_5);
    // ANCHOR_END: box_append

    // ANCHOR: window_child
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("QPager")
        .default_width(800)
        .default_height(480)
        .child(&vert_container)
        .build();
    // ANCHOR_END: window_child

    // Present the window
    window.present();
}


// ANCHOR: callback
    // Connect callbacks
    // When a button is clicked, `number` and label of the other button will be changed
    /*button_increase.connect_clicked(clone!(@weak number, @weak button_decrease =>
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(&number.get().to_string());
    }));
    button_decrease.connect_clicked(clone!(@weak button_increase =>
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label(&number.get().to_string());
    }));*/
    // ANCHOR_END: callback