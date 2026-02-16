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
        x = check_limit(x + self.direction.0, COLS);
        y = check_limit(y + self.direction.1, ROWS);
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

fn check_limit(coord: i32, limit: i32) -> i32 {
    if coord < 0 {
        limit - 1
    } else if coord > limit {
        0
    } else {
        coord
    }
}
