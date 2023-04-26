use crate::rain::{Screen, BRIGHTEST, INVISIBLE};
use itertools::Itertools;
use ncurses::{
    attroff, attron, curs_set, endwin, getch, getmaxyx, init_color, init_pair, initscr, mvaddstr,
    nodelay, noecho, raw, refresh, setlocale, start_color, stdscr, LcCategory, COLOR_PAIR,
    CURSOR_VISIBILITY,
};

use css_color_parser::Color as CssColor;

const MUL: f32 = 0.65;
const COLOR_MAX: i16 = 1000;
const INTENSITY: [i16; BRIGHTEST as usize + 1] = [1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 4, 7];
const WHITESPACE: u32 = ' ' as u32;

struct Color {
    r: i16,
    g: i16,
    b: i16,
}

impl From<[u8; 3]> for Color {
    fn from(value: [u8; 3]) -> Self {
        Self {
            r: i16::from(value[0]),
            g: i16::from(value[1]),
            b: i16::from(value[2]),
        }
    }
}

pub struct Config {
    color: Color,
    background: Color,
}

impl Config {
    pub fn new() -> Self {
        Self {
            color: [0, 255, 0].into(),
            background: [0, 0, 0].into(),
        }
    }

    fn parse_color(string: &str) -> Result<Color, Box<dyn std::error::Error>> {
        let c = string.parse::<CssColor>()?;
        Ok([c.r, c.g, c.b].into())
    }

    pub fn set_foreground(&mut self, string: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.color = Self::parse_color(string)?;
        Ok(())
    }
    pub fn set_background(&mut self, string: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.background = Self::parse_color(string)?;
        Ok(())
    }
}

pub fn init(config: &Config) -> (usize, usize) {
    setlocale(LcCategory::all, "en_US.UTF-8");
    let w = initscr();
    noecho();
    nodelay(w, true);
    raw();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    ncurses::use_default_colors();

    let (rf, gf, bf) = (
        (MUL * f32::from(config.color.r)) as i16,
        (MUL * f32::from(config.color.g)) as i16,
        (MUL * f32::from(config.color.b)) as i16,
    );

    let (rb, gb, bb) = (
        (MUL * f32::from(config.background.r)) as i16,
        (MUL * f32::from(config.background.g)) as i16,
        (MUL * f32::from(config.background.b)) as i16,
    );
    init_pair(1, -1, -1);

    for i in 1..8 {
        init_pair(i, i + 1, -1);
    }

    for i in 1..7 {
        init_color(
            i,
            i * rf + (7 - i) * rb,
            i * gf + (7 - i) * gb,
            i * bf + (7 - i) * bb,
        );
    }

    init_color(8, COLOR_MAX, COLOR_MAX, COLOR_MAX);

    get_xy()
}

pub fn get_xy() -> (usize, usize) {
    let (mut height, mut width) = (0, 0);
    getmaxyx(stdscr(), &mut height, &mut width);

    (height as usize, width as usize)
}

pub fn show(s: &Screen) {
    for (j, i) in (0..s.max_y).cartesian_product(0..s.max_x) {
        unsafe {
            let cell = *s.s.get_unchecked(j).get_unchecked(i);

            if cell.b <= INVISIBLE {
                continue;
            }

            let b = cell.b as usize;
            let ch_idx = if b == 0 { WHITESPACE } else { cell.c };
            let ch = char::from_u32(ch_idx).unwrap_or('□');

            let pair = *INTENSITY.get_unchecked(b);

            attron(COLOR_PAIR(pair));
            mvaddstr(j as i32, i as i32, format!("{ch}").as_ref());

            attroff(COLOR_PAIR(pair));
        }
    }
    refresh();
}

pub fn term() -> bool {
    getch() == 3
}

pub fn finish() -> Result<(), Box<dyn std::error::Error>> {
    match endwin() {
        0 => Ok(()),
        1 => Err("1".into()),
        _ => panic!("Wrong return code"),
    }
}
