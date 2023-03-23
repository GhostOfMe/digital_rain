use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};
use std::cmp::{max, min};

const LATIN_START: u32 = 0x2A;
const LATIN_END: u32 = 0x5A;
const KANA_START: u32 = 0xFF66;
const KANA_END: u32 = 0xFF9D;

const DROP_RATE: f32 = 0.3;
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

#[derive(Copy, Clone)]
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
    pub fn new(height: usize, width: usize) -> Self {
        let mut rng = thread_rng();

        let s = new_cell_vec(&mut rng, width, height);

        Self {
            s,
            drops: Vec::new(),
            max_x: width,
            max_y: height,
            drop_rate: DROP_RATE,
            mutate_rate: MUTATE_RATE,
            dim_rate: DIM_RATE,
            rng,
        }
    }

    pub fn update(&mut self, width: usize, height: usize) {
        if width != self.max_x || height != self.max_y {
            self.resize(width, height);
        }

        self.mutate_screen();

        let mut tmp_drops = self.drops.clone();
        let tmp_max_y = self.max_y as i32;
        let tmp_max_x = self.max_x as i32;
        tmp_drops = tmp_drops
            .iter()
            .filter(|x| x.y < tmp_max_y && x.x < tmp_max_x)
            .map(|d| {
                unsafe {
                    let mut cell = self
                        .s
                        .get_unchecked_mut(d.y as usize)
                        .get_unchecked_mut(d.x as usize);
                    cell.b = MAX_INTENSITY_INDEX;
                }
                Drop { x: d.x, y: d.y + 1 }
            })
            .collect();

        let mut drop_mul = self.drop_rate * self.max_x as f32 / 80.;

        while drop_mul > 0. {
            if self.rng.gen::<f32>() < drop_mul {
                if let Some(x) = (0..self.max_x)
                    .filter(|x| unsafe { self.s.get_unchecked(0).get_unchecked(*x).b == -1 })
                    .choose(&mut thread_rng())
                {
                    let new_drop = Drop { y: 0, x: x as i32 };
                    tmp_drops.push(new_drop);
                } else {
                    break;
                }
            }

            drop_mul -= 1.;
        }

        self.drops = tmp_drops;
    }

    fn mutate_screen(&mut self) {
        let brightness_below: Vec<i8> = self
            .s
            .iter()
            .flat_map(|row| row.iter().map(|c| c.b))
            .skip(self.max_x)
            .take((self.max_x) * (self.max_y - 1))
            .chain((0..self.max_x).map(|_| MAX_INTENSITY_INDEX))
            .collect();

        for (cell, brightness_below) in self
            .s
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .zip(brightness_below)
        //     .filter(|(c, _)| c.b != INVISIBLE)
        {
            if cell.b == MAX_INTENSITY_INDEX {
                cell.b -= 1;
                continue;
            }

            if cell.b == 0 {
                cell.b = INVISIBLE;
                continue;
            }

            if self.rng.gen::<f32>() < self.mutate_rate {
                cell.c = get_random_char(&mut self.rng)
            }

            cell.b = min(cell.b, brightness_below);

            if cell.b > 0 && self.rng.gen::<f32>() < self.dim_rate {
                cell.b -= 1
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
                                b: INVISIBLE,
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

fn new_cell_vec(rng: &mut ThreadRng, width: usize, height: usize) -> Vec<Vec<Cell>> {
    (0..=height)
        .map(|_| {
            (0..=width)
                .map(|_| Cell {
                    c: get_random_char(rng),
                    b: INVISIBLE,
                })
                .collect()
        })
        .collect()
}
