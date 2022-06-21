use gtk::prelude::*;
use gtk::{ Button, Grid, Align };

fn keyboard_buttons(str: &str) -> Button
{
    return Button::builder()
        .label(str)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();
}

pub fn create_keyboard() -> Grid
{
    let button_0 = keyboard_buttons("0");
    let button_1 = keyboard_buttons("1");
    let button_2 = keyboard_buttons("2");
    let button_3 = keyboard_buttons("3");
    let button_4 = keyboard_buttons("4");
    let button_5 = keyboard_buttons("5");
    let button_6 = keyboard_buttons("6");
    let button_7 = keyboard_buttons("7");
    let button_8 = keyboard_buttons("8");
    let button_9 = keyboard_buttons("9");

    let button_Q = keyboard_buttons("Q");
    let button_W = keyboard_buttons("W");
    let button_E = keyboard_buttons("E");
    let button_R = keyboard_buttons("R");
    let button_T = keyboard_buttons("T");
    let button_Y = keyboard_buttons("Y");
    let button_U = keyboard_buttons("U");
    let button_I = keyboard_buttons("I");
    let button_O = keyboard_buttons("O");
    let button_P = keyboard_buttons("P");
    let button_A = keyboard_buttons("A");
    let button_S = keyboard_buttons("S");
    let button_D = keyboard_buttons("D");
    let button_F = keyboard_buttons("F");
    let button_G = keyboard_buttons("G");
    let button_H = keyboard_buttons("H");
    let button_J = keyboard_buttons("J");
    let button_K = keyboard_buttons("K");
    let button_L = keyboard_buttons("L");
    let button_Z = keyboard_buttons("Z");
    let button_X = keyboard_buttons("X");
    let button_C = keyboard_buttons("C");
    let button_V = keyboard_buttons("V");
    let button_B = keyboard_buttons("B");
    let button_N = keyboard_buttons("N");
    let button_M = keyboard_buttons("M");

    let button_del = keyboard_buttons("del");
    let button_caps = keyboard_buttons("caps");
    let button_shift = keyboard_buttons("shift");
    let button_excl = keyboard_buttons("!");
    let button_quest = keyboard_buttons("?");
    let button_coma = keyboard_buttons(",");
    let button_dot = keyboard_buttons(".");
    let button_space = keyboard_buttons("space");
    let button_alt = keyboard_buttons("alt");
    let button_ctrl = keyboard_buttons("ctrl");
    let button_enter = keyboard_buttons("enter");

    let grid = Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .halign(Align::Center)
        .valign(Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();
    
    grid.attach(&button_1, 0, 0, 1, 1);
    grid.attach(&button_2, 1, 0, 1, 1);
    grid.attach(&button_3, 2, 0, 1, 1);
    grid.attach(&button_4, 3, 0, 1, 1);
    grid.attach(&button_5, 4, 0, 1, 1);
    grid.attach(&button_6, 5, 0, 1, 1);
    grid.attach(&button_7, 6, 0, 1, 1);
    grid.attach(&button_8, 7, 0, 1, 1);
    grid.attach(&button_9, 8, 0, 1, 1);
    grid.attach(&button_0, 9, 0, 1, 1);
    grid.attach(&button_del, 10, 0, 1, 1);

    grid.attach(&button_Q, 0, 1, 1, 1);
    grid.attach(&button_W, 1, 1, 1, 1);
    grid.attach(&button_E, 2, 1, 1, 1);
    grid.attach(&button_R, 3, 1, 1, 1);
    grid.attach(&button_T, 4, 1, 1, 1);
    grid.attach(&button_Y, 5, 1, 1, 1);
    grid.attach(&button_U, 6, 1, 1, 1);
    grid.attach(&button_I, 7, 1, 1, 1);
    grid.attach(&button_O, 8, 1, 1, 1);
    grid.attach(&button_P, 9, 1, 1, 1);
    grid.attach(&button_excl, 10, 1, 1, 1);

    grid.attach(&button_caps, 0, 2, 1, 1);
    grid.attach(&button_A, 1, 2, 1, 1);
    grid.attach(&button_S, 2, 2, 1, 1);
    grid.attach(&button_D, 3, 2, 1, 1);
    grid.attach(&button_F, 4, 2, 1, 1);
    grid.attach(&button_G, 5, 2, 1, 1);
    grid.attach(&button_H, 6, 2, 1, 1);
    grid.attach(&button_J, 7, 2, 1, 1);
    grid.attach(&button_K, 8, 2, 1, 1);
    grid.attach(&button_L, 9, 2, 1, 1);
    grid.attach(&button_quest, 10, 2, 1, 1);

    grid.attach(&button_shift, 0, 3, 2, 1);
    grid.attach(&button_Z, 2, 3, 1, 1);
    grid.attach(&button_X, 3, 3, 1, 1);
    grid.attach(&button_C, 4, 3, 1, 1);
    grid.attach(&button_V, 5, 3, 1, 1);
    grid.attach(&button_B, 6, 3, 1, 1);
    grid.attach(&button_N, 7, 3, 1, 1);
    grid.attach(&button_M, 8, 3, 1, 1);
    grid.attach(&button_enter, 9, 3, 2, 1);

    grid.attach(&button_ctrl, 0, 4, 1, 1);
    grid.attach(&button_alt, 1, 4, 1, 1);
    grid.attach(&button_space, 2, 4, 7, 1);
    grid.attach(&button_dot, 9, 4, 1, 1);
    grid.attach(&button_coma, 10, 4, 1, 1);

    grid
}