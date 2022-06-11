use crate::rain::{Screen, MAX_INTENSITY_INDEX};
use itertools::Itertools;
use ncurses::*;

const MUL: f32 = 0.65;

const COLOR_MAX: i16 = 1000;

const INTENSITY: [i16; MAX_INTENSITY_INDEX as usize + 1] =
    [1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 4, 7];

pub fn init_ui(color: Option<(i16, i16, i16)>) -> (usize, usize) {
    setlocale(LcCategory::all, "en_US.UTF-8");
    let w = initscr();
    noecho();
    nodelay(w, true);
    raw();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    ncurses::use_default_colors();

    let (r, g, b) = match color {
        Some(rgb) => (
            (MUL * rgb.0 as f32) as i16,
            (MUL * rgb.1 as f32) as i16,
            (MUL * rgb.2 as f32) as i16,
        ),
        None => (0, 640 / 6, 0),
    };

    init_pair(1, -1, -1);

    for i in 1..8 {
        init_pair(i, i + 1, -1);
    }

    for i in 1..7 {
        init_color(i, i * r, i * g, i * b);
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
           
            if cell.b < 0 {
                continue
            }
            
            let b = cell.b as usize;
            let c = if b == 0 { ' ' as u32 } else { cell.c };
            let pair = *INTENSITY.get_unchecked(b);

            attron(COLOR_PAIR(pair));
            mv(j as i32, i as i32);
            addstr(format!("{}", char::from_u32(c).expect("Invalid char")).as_ref());
            attroff(COLOR_PAIR(pair));
        }
    }
    refresh();
}

pub fn term() -> bool {
    getch() == 3
}

pub fn finish() {
    endwin();
}
