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
        .version("0.2.4")
        .arg(
            Arg::with_name("color")
                .short("c")
                .long("color")
                .takes_value(true)
                .help("Use the chosen foreground color. Ex.: <--color=#98c396>"))
        .arg( 
            Arg::with_name("background_color")
                .short("b")
                .long("background")
                .takes_value(true)
                .help("Use the chosen background color. Improves color blending. Ex.: <--background=#2f3b35>"))
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

    let background: Option<(i16, i16, i16)> = match app.value_of("background_color") {
        None => None,
        Some(color_string) => {
            let color = color_string
                .parse::<CssColor>()
                .expect("Wrong color format");
            Some((color.r as i16, color.g as i16, color.b as i16))
        }
    };
    let (height, width) = init_ui(color, background);
    let mut s = Screen::new(height-1, width-1);
    loop {
        if term() {
            break;
        }

        let (height, width) = get_xy();
        s.update(width, height);
        show(&s);
        thread::sleep(time::Duration::from_millis(TIMEOUT));
    }
    finish();
    print!("{}", s.max_x)
}
