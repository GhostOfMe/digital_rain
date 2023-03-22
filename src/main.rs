#![feature(test)]
mod rain;
mod ui;
extern crate clap;
extern crate css_color_parser;
extern crate ncurses;
extern crate rand;
extern crate test;

use clap::{App, Arg};
use css_color_parser::Color as CssColor;
use rain::*;
use std::{thread, time};
use ui::*;

const TIMEOUT: u64 = 50;

fn main() {
    let app = App::new("Digital Rain")
        .version("0.2.2")
        .arg(
            Arg::with_name("color")
                .short("c")
                .long("color")
                .takes_value(true)
                .help("Use the chosen color. Ex.: <--color=#98c396>"),
        )
        .get_matches();
    let color: Option<(i16, i16, i16)> = match app.value_of("color") {
        None => None,
        Some(color_string) => {
            let color = color_string
                .parse::<CssColor>()
                .expect("Wrong color format");
            Some((color.r as i16, color.g as i16, color.b as i16))
        }
    };

    let (height, width) = init_ui(color);
    let mut s = Screen::new(width, height);
    loop {
        if term() {
            break;
        }

        let (y, x) = get_xy();
        s.update(x, y);
        show(&s);
        thread::sleep(time::Duration::from_millis(TIMEOUT));
    }
    finish();
}
