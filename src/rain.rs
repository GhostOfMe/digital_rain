use itertools::Itertools;
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};

const LATIN_START: u32 = 0x2A;
const LATIN_END: u32 = 0x5A;
const JAPAN_START: u32 = 0xFF67;
const JAPAN_END: u32 = 0xFF9D;

const DROP_RATE: f32 = 0.45;
const MUTATE_RATE: f32 = 0.05;
const DIM_RATE: f32 = 0.5;
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

        let s: Vec<Vec<Cell>> = (0..y)
            .map(|_| {
                (0..x)
                    .map(|_| Cell {
                        c: (LATIN_START..LATIN_END)
                            .chain(JAPAN_START..JAPAN_END)
                            .choose(&mut rng)
                            .unwrap(),
                        b: 0,
                    })
                    .collect()
            })
            .collect();

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

    pub fn update(&mut self) {
        let create_drop = self.rng.gen::<f32>() < self.drop_rate;
        self.mutate_screen();

        if create_drop {
            let new_drop = Drop {
                y: 0,
                x: self.rng.gen_range(0..self.max_x as i32),
                passed: false,
            };
            self.drops.push(new_drop);
        }
        for d in self.drops.iter_mut() {
            d.y += 1;
        }

        self.drops = self
            .drops
            .iter()
            .cloned()
            .filter(|x| x.y < self.max_y as i32)
            .collect();

        for d in self.drops.iter() {
            let (x, y) = (d.x as usize, d.y as usize);
            self.s[y][x].b = 7;
        }
    }

    fn mutate_screen(&mut self) {
        for (j, i) in (0..self.max_y).cartesian_product(0..self.max_x) {
            if self.s[j][i].b == -1 {
                continue;
            }

            if self.s[j][i].b == 7 {
                self.s[j][i].b -= 1;
                continue;
            }
            if self.s[j][i].b == 0 {
                self.s[j][i].b -= 1;
                continue;
            }

            let mutate_screen_cell = self.rng.gen::<f32>() < self.mutate_rate;
            let dim_screen_cell = self.rng.gen::<f32>() < self.dim_rate;

            if mutate_screen_cell {
                self.s[j][i].c = (0xFF67..0xFF9D as u32)
                    .chain(0x30..0x5A as u32)
                    .choose(&mut self.rng)
                    .unwrap();
            }
            if dim_screen_cell {
                self.s[j][i].b -= 1
            }
        }
    }
}
