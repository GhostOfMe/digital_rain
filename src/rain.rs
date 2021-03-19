use itertools::Itertools;
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};
use std::cmp::max;

const LATIN_START: u32 = 0x2A;
const LATIN_END: u32 = 0x5A;
const JAPAN_START: u32 = 0xFF67;
const JAPAN_END: u32 = 0xFF9D;

const DROP_RATE: f32 = 0.20;
const MUTATE_RATE: f32 = 0.05;
const DIM_RATE: f32 = 0.5;

const MAX_INTENSITY_INDEX: i8 = 11;
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
    passed: bool,
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

        let s = get_cell_vec(&mut rng, x, y);

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
            self.max_x = new_x;
            self.max_y = new_y;
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

        if self.rng.gen::<f32>() < self.drop_rate * self.max_x as f32 / 80. {
            let new_drop = Drop {
                y: 0,
                x: self.rng.gen_range(0..self.max_x as i32),
                passed: false,
            };
            self.drops.push(new_drop);
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

    fn resize(&mut self, x: usize, y: usize) {
        let s: Vec<Vec<Cell>> = (0..=max(y, self.max_y))
            .map(|y_tmp| {
                (0..=max(x, self.max_x))
                    .map(|x_tmp| {
                        if x_tmp < self.max_x && y_tmp < self.max_y {
                            self.s[y_tmp][x_tmp]
                        } else {
                            Cell {
                                c: get_random_char(&mut self.rng),
                                b: 0,
                            }
                        }
                    })
                    .take(x)
                    .collect()
            })
            .take(y)
            .collect();

        self.s = s;
    }
}

fn get_random_char(rng: &mut ThreadRng) -> u32 {
    (LATIN_START..LATIN_END)
        .chain(JAPAN_START..JAPAN_END)
        .choose(rng)
        .unwrap()
}

fn get_cell_vec(rng: &mut ThreadRng, x: usize, y: usize) -> Vec<Vec<Cell>> {
    let s: Vec<Vec<Cell>> = (0..=y)
        .map(|_| {
            (0..=x)
                .map(|_| Cell {
                    c: get_random_char(rng),
                    b: 0,
                })
                .collect()
        })
        .collect();

    s
}
