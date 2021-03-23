use itertools::Itertools;
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};
use std::cmp::max;

const LATIN_START: u32 = 0x2A;
const LATIN_END: u32 = 0x5A;
const KANA_START: u32 = 0xFF66;
const KANA_END: u32 = 0xFF9D;

const DROP_RATE: f32 = 0.4;
const MUTATE_RATE: f32 = 0.025;
const DIM_RATE: f32 = 0.5;

pub const MAX_INTENSITY_INDEX: i8 = 15;
const INVISIBLE: i8 = -1;

pub struct Screen {
    pub s: Vec<Vec<Cell>>,
    drops: Vec<Drop>,
    pub max_x: usize,
    pub max_y: usize,
    drop_rate: f32,
    mutate_rate: f32,
    dim_rate: f32,
    rng: ThreadRng,
}

#[derive(Clone)]
pub struct Drop {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
pub struct Cell {
    // brightness
    pub b: i8,
    // character
    pub c: u32,
}

impl Screen {
    pub fn new(x: usize, y: usize) -> Self {
        let mut rng = thread_rng();

        let s = new_cell_vec(&mut rng, x, y);

        Screen {
            s: s,
            drops: Vec::new(),
            max_x: x,
            max_y: y,
            drop_rate: DROP_RATE,
            mutate_rate: MUTATE_RATE,
            dim_rate: DIM_RATE,
            rng: rng,
        }
    }

    pub fn update(&mut self, new_x: usize, new_y: usize) {
        if new_x != self.max_x || new_y != self.max_y {
            self.resize(new_x, new_y);
        }

        self.drops = self
            .drops
            .iter()
            .cloned()
            .filter(|x| x.y < self.max_y as i32 && x.x < self.max_x as i32)
            .collect();

        self.mutate_screen();

        for d in self.drops.iter() {
            let (x, y) = (d.x as usize, d.y as usize);
            self.s[y][x].b = MAX_INTENSITY_INDEX;
        }

        for d in self.drops.iter_mut() {
            d.y += 1;
        }

        let mut drop_mul = self.drop_rate * 120. / 80.;

        while drop_mul > 0. {
            if self.rng.gen::<f32>() < drop_mul {
                /*
                    let new_drop = Drop { y: 0, x: self.rng.gen_range(0..self.max_x as i32) };
                    self.drops.push(new_drop);
                */

                if let Some(x) = (0..self.max_x)
                    .clone()
                    .filter(|x| self.s[0][*x].b == -1)
                    .choose(&mut thread_rng())
                {
                    let new_drop = Drop { y: 0, x: x as i32 };
                    self.drops.push(new_drop);
                }else{
                    break
                }

            }

            drop_mul -= 1.;
        }
    }

    fn mutate_screen(&mut self) {
        for (j, i) in (0..self.max_y).cartesian_product(0..self.max_x) {
            if self.s[j][i].b == INVISIBLE {
                continue;
            }

            if self.s[j][i].b == MAX_INTENSITY_INDEX {
                self.s[j][i].b -= 1;
                continue;
            }

            if self.s[j][i].b == 0 {
                self.s[j][i].b = INVISIBLE;
                continue;
            }

            if self.rng.gen::<f32>() < self.mutate_rate {
                self.s[j][i].c = get_random_char(&mut self.rng)
            }

            if self.rng.gen::<f32>() < self.dim_rate {
                self.s[j][i].b -= 1
            }
        }
    }

    fn resize(&mut self, new_x: usize, new_y: usize) {
        let s: Vec<Vec<Cell>> = (0..=max(new_y, self.max_y))
            .map(|tmp_y| {
                (0..=max(new_x, self.max_x))
                    .map(|tmp_x| {
                        if tmp_x < self.max_x && tmp_y < self.max_y {
                            self.s[tmp_y][tmp_x]
                        } else {
                            Cell {
                                c: get_random_char(&mut self.rng),
                                b: 0,
                            }
                        }
                    })
                    .take(new_x)
                    .collect()
            })
            .take(new_y)
            .collect();

        self.s = s;
        self.max_x = new_x;
        self.max_y = new_y;
    }
}

fn get_random_char(rng: &mut ThreadRng) -> u32 {
    (LATIN_START..LATIN_END)
        .chain(KANA_START..KANA_END)
        .choose(rng)
        .unwrap()
}

fn new_cell_vec(rng: &mut ThreadRng, x: usize, y: usize) -> Vec<Vec<Cell>> {
    (0..=y)
        .map(|_| {
            (0..=x)
                .map(|_| Cell {
                    c: get_random_char(rng),
                    b: 0,
                })
                .collect()
        })
        .collect()
}
