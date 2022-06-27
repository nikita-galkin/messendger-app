use std::collections::HashMap;

use gtk::prelude::*;
use gtk::{ Button, ToggleButton, Grid, Align };

fn keyboard_buttons(str: &str) -> Button
{
    return Button::builder()
        .label(str)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .can_focus(false)
        .build();
}
fn special_keyboard_buttons(str: &str) -> ToggleButton
{
    return ToggleButton::builder()
        .label(str)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .can_focus(false)
        .build();
}

pub fn create_keyboard() -> (Grid, HashMap<char, Button>, HashMap<&'static str, ToggleButton>)
{
    let mut buttons: HashMap<char, Button> = HashMap::new();
    let mut special_buttons: HashMap<&'static str, ToggleButton> = HashMap::new();
    let button_0 = keyboard_buttons("0");
    buttons.insert('0', button_0.clone());
    let button_1 = keyboard_buttons("1");
    buttons.insert('1', button_1.clone());
    let button_2 = keyboard_buttons("2");
    buttons.insert('2', button_2.clone());
    let button_3 = keyboard_buttons("3");
    buttons.insert('3', button_3.clone());
    let button_4 = keyboard_buttons("4");
    buttons.insert('4', button_4.clone());
    let button_5 = keyboard_buttons("5");
    buttons.insert('5', button_5.clone());
    let button_6 = keyboard_buttons("6");
    buttons.insert('6', button_6.clone());
    let button_7 = keyboard_buttons("7");
    buttons.insert('7', button_7.clone());
    let button_8 = keyboard_buttons("8");
    buttons.insert('8', button_8.clone());
    let button_9 = keyboard_buttons("9");
    buttons.insert('9', button_9.clone());
    

    let button_q = keyboard_buttons("Q");
    buttons.insert('q', button_q.clone());
    let button_w = keyboard_buttons("W");
    buttons.insert('w', button_w.clone());
    let button_e = keyboard_buttons("E");
    buttons.insert('e', button_e.clone());
    let button_r = keyboard_buttons("R");
    buttons.insert('r', button_r.clone());
    let button_t = keyboard_buttons("T");
    buttons.insert('t', button_t.clone());
    let button_y = keyboard_buttons("Y");
    buttons.insert('y', button_y.clone());
    let button_u = keyboard_buttons("U");
    buttons.insert('u', button_u.clone());
    let button_i = keyboard_buttons("I");
    buttons.insert('i', button_i.clone());
    let button_o = keyboard_buttons("O");
    buttons.insert('o', button_o.clone());
    let button_p = keyboard_buttons("P");
    buttons.insert('p', button_p.clone());
    let button_a = keyboard_buttons("A");
    buttons.insert('a', button_a.clone());
    let button_s = keyboard_buttons("S");
    buttons.insert('s', button_s.clone());
    let button_d = keyboard_buttons("D");
    buttons.insert('d', button_d.clone());
    let button_f = keyboard_buttons("F");
    buttons.insert('f', button_f.clone());
    let button_g = keyboard_buttons("G");
    buttons.insert('g', button_g.clone());
    let button_h = keyboard_buttons("H");
    buttons.insert('h', button_h.clone());
    let button_j = keyboard_buttons("J");
    buttons.insert('j', button_j.clone());
    let button_k = keyboard_buttons("K");
    buttons.insert('k', button_k.clone());
    let button_l = keyboard_buttons("L");
    buttons.insert('l', button_l.clone());
    let button_z = keyboard_buttons("Z");
    buttons.insert('z', button_z.clone());
    let button_x = keyboard_buttons("X");
    buttons.insert('x', button_x.clone());
    let button_c = keyboard_buttons("C");
    buttons.insert('c', button_c.clone());
    let button_v = keyboard_buttons("V");
    buttons.insert('v', button_v.clone());
    let button_b = keyboard_buttons("B");
    buttons.insert('b', button_b.clone());
    let button_n = keyboard_buttons("N");
    buttons.insert('n', button_n.clone());
    let button_m = keyboard_buttons("M");
    buttons.insert('m', button_m.clone());
    let button_excl = keyboard_buttons("!");
    buttons.insert('!', button_excl.clone());
    let button_quest = keyboard_buttons("?");
    buttons.insert('?', button_quest.clone());
    let button_coma = keyboard_buttons(",");
    buttons.insert(',', button_coma.clone());
    let button_dot = keyboard_buttons(".");
    buttons.insert('.', button_dot.clone());
    let button_space = keyboard_buttons("space");
    buttons.insert(' ', button_space.clone());
    let button_del = keyboard_buttons("del");
    buttons.insert('<', button_del.clone());
    let button_enter = keyboard_buttons("enter");
    buttons.insert('>', button_enter.clone());

    let button_caps = special_keyboard_buttons("caps");
    special_buttons.insert("caps", button_caps.clone());
    let button_shift = special_keyboard_buttons("shift");
    special_buttons.insert("shift", button_shift.clone());
    let button_alt = special_keyboard_buttons("alt");
    special_buttons.insert("alt", button_alt.clone());
    let button_ctrl = special_keyboard_buttons("ctrl");
    special_buttons.insert("ctrl", button_ctrl.clone());
    

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

    grid.attach(&button_q, 0, 1, 1, 1);
    grid.attach(&button_w, 1, 1, 1, 1);
    grid.attach(&button_e, 2, 1, 1, 1);
    grid.attach(&button_r, 3, 1, 1, 1);
    grid.attach(&button_t, 4, 1, 1, 1);
    grid.attach(&button_y, 5, 1, 1, 1);
    grid.attach(&button_u, 6, 1, 1, 1);
    grid.attach(&button_i, 7, 1, 1, 1);
    grid.attach(&button_o, 8, 1, 1, 1);
    grid.attach(&button_p, 9, 1, 1, 1);
    grid.attach(&button_excl, 10, 1, 1, 1);

    grid.attach(&button_caps, 0, 2, 1, 1);
    grid.attach(&button_a, 1, 2, 1, 1);
    grid.attach(&button_s, 2, 2, 1, 1);
    grid.attach(&button_d, 3, 2, 1, 1);
    grid.attach(&button_f, 4, 2, 1, 1);
    grid.attach(&button_g, 5, 2, 1, 1);
    grid.attach(&button_h, 6, 2, 1, 1);
    grid.attach(&button_j, 7, 2, 1, 1);
    grid.attach(&button_k, 8, 2, 1, 1);
    grid.attach(&button_l, 9, 2, 1, 1);
    grid.attach(&button_quest, 10, 2, 1, 1);

    grid.attach(&button_shift, 0, 3, 2, 1);
    grid.attach(&button_z, 2, 3, 1, 1);
    grid.attach(&button_x, 3, 3, 1, 1);
    grid.attach(&button_c, 4, 3, 1, 1);
    grid.attach(&button_v, 5, 3, 1, 1);
    grid.attach(&button_b, 6, 3, 1, 1);
    grid.attach(&button_n, 7, 3, 1, 1);
    grid.attach(&button_m, 8, 3, 1, 1);
    grid.attach(&button_enter, 9, 3, 2, 1);

    grid.attach(&button_ctrl, 0, 4, 1, 1);
    grid.attach(&button_alt, 1, 4, 1, 1);
    grid.attach(&button_space, 2, 4, 7, 1);
    grid.attach(&button_dot, 9, 4, 1, 1);
    grid.attach(&button_coma, 10, 4, 1, 1);

    (grid, buttons, special_buttons)
}