use super::*;
extern crate test;
use test::Bencher;

#[bench]
fn bench_screen_update(b: &mut Bencher) {
    let (h, w) = (100, 100);
    let mut s = Screen::new(h, w);

    b.iter(|| s.update(h, w));
}
