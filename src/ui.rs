use crate::rain::{Screen, MAX_INTENSITY_INDEX};
use itertools::Itertools;
use ncurses::*;

const COLOR_BASE_R: i16 = 152;      // Default 0
const COLOR_BASE_G: i16 = 195;      // Default 200
const COLOR_BASE_B: i16 = 121;      // Default 0
const COLOR_MAX: i16 = 1000;

const INTENSITY: [i16; MAX_INTENSITY_INDEX as usize + 1] =
    [1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 4, 7];

pub fn init_ui() -> (usize, usize) {
    setlocale(LcCategory::all, "en_US.UTF-8");
    let w = initscr();
    noecho();
    nodelay(w, true);
    raw();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    init_pair(1, COLOR_BLACK, COLOR_BLACK);
    for i in 1..7 {
        init_pair(i + 1, i, COLOR_BLACK);
    }
    for i in 1..6 {
        init_color(i, i * COLOR_BASE_R, i * COLOR_BASE_G, i * COLOR_BASE_B);
    }

    init_color(6, COLOR_MAX, COLOR_MAX, COLOR_MAX);

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
            if cell.b >= 0 {
                let b = cell.b as usize;
                let c = if b == 0 { ' ' as u32 } else { cell.c };
                let pair = *INTENSITY.get_unchecked(b);

                attron(COLOR_PAIR(pair));
                mv(j as i32, i as i32);
                addstr(format!("{}", char::from_u32(c).expect("Invalid char")).as_ref());
                attroff(COLOR_PAIR(pair));
            }
        }
    }
    refresh();
}

pub fn term() -> bool {
    getch() == 3
}

pub fn finish(){
    endwin();
}
