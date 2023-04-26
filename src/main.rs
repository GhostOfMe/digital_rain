#![feature(test)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
extern crate clap;
extern crate css_color_parser;
extern crate ncurses;
extern crate rand;

mod rain;
#[cfg(test)]
mod test;
mod ui;

use clap::{App, Arg};
use rain::Screen;
use std::{thread, time};
use ui::{finish, get_xy, show, term, Config};

const TIMEOUT: u64 = 50;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut config = Config::new();

    if let Some(s) = app.value_of("color") {
        config.set_foreground(s.into())?;
    }

    if let Some(s) = app.value_of("background_color") {
        config.set_background(s.into())?;
    }

    let (height, width) = ui::init(config);
    let mut s = Screen::new(height - 1, width - 1);
    loop {
        if term() {
            break;
        }

        let (height, width) = get_xy();
        s.update(width, height);
        show(&s);
        thread::sleep(time::Duration::from_millis(TIMEOUT));
    }
    finish()
}
