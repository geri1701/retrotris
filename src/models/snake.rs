use crate::*;
use rand::Rng;

#[derive(Default)]
pub struct Score(i32, u64);

impl Score {
    pub fn inc(&mut self) {
        self.0 += 1;
    }
    pub fn level(&self) -> u64 {
        self.1
    }
    pub fn update(&mut self) {
        self.1 = match self.0 {
            0..5 => 0,
            5..15 => 1,
            15..30 => 2,
            _ => 3,
        }
    }
}

#[derive(Default)]
pub struct Apple(pub i32, pub i32);

impl Apple {
    pub fn new(field: (i32, i32)) -> Self {
        Self(
            rand::rng().random_range(0..field.0),
            rand::rng().random_range(0..field.1),
        )
    }
}

#[derive(Default)]
pub struct Snake {
    pub body: Vec<(i32, i32)>,
    pub direction: (i32, i32),
}

impl Snake {
    pub fn set(&mut self, field: (i32, i32)) {
        *self = Self {
            body: vec![(
                rand::rng().random_range(0..field.0),
                rand::rng().random_range(0..field.1),
            )],
            direction: (1, 0),
        }
    }
    pub fn update(&mut self) -> Option<(i32, i32)> {
        let (mut x, mut y) = self.body[0];
        x = check_x(x + self.direction.0);
        y = check_y(y + self.direction.1);
        if self.body.contains(&(x, y)) {
            None
        } else {
            self.body.insert(0, (x, y));
            Some((x, y))
        }
    }
    pub fn direction(&mut self, x: i32, y: i32) {
        if self.direction != (x, y) && self.direction != (-x, -y) {
            self.direction = (x, y);
        }
    }
}

fn check_y(coord: i32) -> i32 {
    match coord {
        -1..0 => ROWS - 1,
        0..ROWS => coord,
        _ => 0,
    }
}

fn check_x(coord: i32) -> i32 {
    match coord {
        -1..0 => COLS - 1,
        0..COLS => coord,
        _ => 0,
    }
}
