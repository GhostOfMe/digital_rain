use crate::rain::Screen;
use itertools::Itertools;
use ncurses::*;

const COLOR_BASE: i16 = 200;
const COLOR_MAX: i16 = 1000;

const INTENSITY: [i16; 8] = [1, 2, 2, 3, 4, 5, 4, 7];

pub fn init_ui() -> (usize, usize) {
    let (mut height, mut width) = (0, 0);
    setlocale(LcCategory::all, "en_US.UTF-8");
    initscr();
    noecho();
    getmaxyx(stdscr(), &mut height, &mut width);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    init_pair(1, COLOR_BLACK, COLOR_BLACK);
    for i in 0..8 {
        init_pair(i + 1, i, COLOR_BLACK);
    }
    for i in 1..5 {
        init_color(i, 0, i * COLOR_BASE, 0);
    }

    init_color(6, COLOR_MAX, COLOR_MAX, COLOR_MAX);

    (height as usize, width as usize)
}

pub fn show(s: &Screen) {
    for (j, i) in (0..s.max_y).cartesian_product(0..s.max_x) {
        if s.s[j][i].b >= 0 {
            let b = s.s[j][i].b as usize;
            let c = s.s[j][i].c;
            attron(COLOR_PAIR(INTENSITY[b]));
            mv(j as i32, i as i32);
            addstr(format!("{}", char::from_u32(c).expect("Invalid char")).as_ref());
            attroff(COLOR_PAIR(INTENSITY[b]));
        }
    }
    refresh();
}
