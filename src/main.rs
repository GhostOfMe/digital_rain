#![feature(assoc_char_funcs)]
mod rain;
mod ui;
extern crate ncurses;
extern crate rand;

use rain::*;
use std::{thread, time};
use ui::*;

const TIMEOUT: u64 = 50;

fn main() {
    let (height, width) = init_ui();
    let mut s = Screen::new(width, height);

    loop {
        s.update();
        show(&s);
        thread::sleep(time::Duration::from_millis(TIMEOUT));
    }
}
