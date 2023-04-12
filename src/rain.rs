use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};
use rand_distr::{Binomial, Distribution};
use std::cmp::max;

const DIGIT_START: u32 = 0x30;
const DIGIT_END: u32 = 0x39;
const LATIN_START: u32 = 0x41;
const LATIN_END: u32 = 0x5A;
const KANA_START: u32 = 0xFF66;
const KANA_END: u32 = 0xFF9D;

const DROP_RATE: f32 = 0.3;
const MUTATE_RATE: f32 = 0.025;
const DIM_RATE: f32 = 0.5;

pub const BRIGHTEST: i8 = 15;
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
    flip_counter: usize,
    dim_counter: usize,
}

impl Cell {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let c = get_random_char(rng);
        let b = INVISIBLE;
        let flip_counter = Self::get_flip_counter(rng);
        let dim_counter = Self::get_dim_counter(rng);
        Self {
            c,
            b,
            flip_counter,
            dim_counter,
        }
    }

    pub fn tick(&mut self, rng: &mut ThreadRng) {
        match self.b {
            x if x >= BRIGHTEST => self.b -= 1,
            x if x > 0 => {
                if self.dim_counter == 0 {
                    self.dim_counter = Self::get_dim_counter(rng);
                    self.b -= 1
                } else {
                    self.dim_counter -= 1
                }
            }
            x if x <= 0 => self.b = INVISIBLE,
            _ => panic!("Wrong brightness index!"),
        }

        if self.flip_counter > 0 {
            self.flip_counter -= 1;
            return;
        }
        self.flip_counter = Self::get_flip_counter(rng);
        self.c = get_random_char(rng);
    }

    fn get_flip_counter(rng: &mut ThreadRng) -> usize {
        let max_count = (MUTATE_RATE * 1200.).floor() as usize;
        rng.gen_range(10..max_count)
    }
    fn get_dim_counter(rng: &mut ThreadRng) -> usize {
        let bin = Binomial::new(2, DIM_RATE as f64).unwrap();
        bin.sample(rng) as usize
    }
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
                    cell.b = BRIGHTEST;
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
        for cell in self
            .s
            .iter_mut()
            .rev()
            .flat_map(|row| row.iter_mut())
            .filter(|c| c.b > INVISIBLE)
        {
            cell.tick(&mut self.rng)
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
                            Cell::new(&mut self.rng)
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
    (DIGIT_START..DIGIT_END)
        .chain(LATIN_START..LATIN_END)
        .chain(KANA_START..KANA_END)
        .choose(rng)
        .unwrap()
}

fn new_cell_vec(rng: &mut ThreadRng, width: usize, height: usize) -> Vec<Vec<Cell>> {
    (0..=height)
        .map(|_| (0..=width).map(|_| Cell::new(rng)).collect())
        .collect()
}
