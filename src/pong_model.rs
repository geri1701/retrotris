use crate::gui::{HEIGHT, WIDTH};

#[derive(Default, Clone)]
enum Direction {
    #[default]
    Positive = 1,
    Negative = -1,
}

#[derive(Default)]
pub struct Paddle(i32, i32);

impl Paddle {
    const SIZE: (i32, i32) = (WIDTH * 2, HEIGHT);
    pub fn set_x(&mut self, value: i32) {
        self.0 = value;
    }
    pub fn set_y(&mut self, value: i32) {
        self.1 = value - HEIGHT * 2;
    }
    fn place(&self, pos: i32) -> bool {
        (self.0..=self.0 + Self::SIZE.0).contains(&pos)
    }
    pub(crate) fn draw(&self) -> (i32, i32, i32, i32) {
        (self.0, self.1, Self::SIZE.0, Self::SIZE.1)
    }
}

#[derive(Default)]
pub struct Ball {
    pos: (i32, i32),
    dir: (Direction, Direction),
}

impl Ball {
    const SIZE: i32 = HEIGHT * 2;
    pub fn draw(&self) -> (i32, i32, i32) {
        (self.pos.0, self.pos.1, Self::SIZE)
    }
    pub fn step(&mut self) {
        self.pos.0 += self.dir.0.clone() as i32;
        self.pos.1 += self.dir.1.clone() as i32;
    }
    pub fn check_paddle(&mut self, paddle: &Paddle) -> bool {
        if paddle.1 == (self.pos.1 + Self::SIZE) && paddle.place(self.pos.0 + Self::SIZE / 2) {
            self.dir.1 = Direction::Negative;
            return true;
        }
        false
    }
    pub fn check_field(&mut self, field: &Field) -> bool {
        if self.pos.0 + Self::SIZE == field.width.1 {
            self.dir.0 = Direction::Negative;
        }
        if self.pos.0 == field.width.0 {
            self.dir.0 = Direction::Positive;
        }
        if self.pos.1 == field.height.0 {
            self.dir.1 = Direction::Positive;
        }
        if self.pos.1 + Self::SIZE == field.height.1 {
            self.pos.1 = 0;
            return true;
        }
        false
    }
}

#[derive(Default)]
pub struct Field {
    width: (i32, i32),
    height: (i32, i32),
}

impl Field {
    pub fn update(&mut self, size: (i32, i32)) {
        self.width.1 = size.0;
        self.height.1 = size.1;
    }
}

#[derive(Default)]
pub struct Score(pub i32, pub i32);

impl Score {
    pub fn dec(&mut self) {
        self.0 -= 1;
    }
    pub fn inc(&mut self) {
        self.0 += 1;
    }
    pub fn update(&mut self) {
        self.1 = match self.0 {
            0..50 => 0,
            50..150 => 1,
            150..300 => 2,
            _ => 3,
        }
    }
    pub fn draw(&self) -> (i32, i32) {
        (self.0, self.1)
    }
}
