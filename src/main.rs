#![feature(assoc_char_funcs, test)]
mod rain;
mod ui;
extern crate ncurses;
extern crate rand;
extern crate test;

use rain::*;
use std::{thread, time};
use ui::*;

const TIMEOUT: u64 = 50;


#[cfg(test)]
mod tests{
    use super::rain::*;
    use test::Bencher;

    #[bench]
    fn bench_loop(b: &mut Bencher){
        let mut s = Screen::new(200, 200);
        b.iter(|| s.mutate_screen_loop());
    }

    #[bench]
    fn bench_iter(b: &mut Bencher){
        let mut s = Screen::new(200, 200);
        b.iter(|| s.mutate_screen());
    }
}



fn main() {
    let (height, width) = init_ui();
    let mut s = Screen::new(width, height);
    loop {
        if term() {
            break
        }

        let (y, x) = get_xy();
        s.update(x, y);
        show(&s);
        thread::sleep(time::Duration::from_millis(TIMEOUT));
    }

    finish();
}
