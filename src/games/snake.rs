use crate::gui::*;
use crate::models::snake::*;
use std::time;

pub struct Model {
    snake: Snake,
    score: Score,
    apple: Apple,
    time: time::Instant,
    play: bool,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            snake: Snake::default(),
            score: Score::default(),
            apple: Apple::default(),
            time: time::Instant::now(),
            play: false,
        }
    }
}
const FIELD_WIDTH: usize = 54;
const FIELD_HEIGHT: usize = 30;
impl Model {
    pub fn map(&self) -> [[Option<usize>; FIELD_WIDTH]; FIELD_HEIGHT] {
        let mut field = [[None; FIELD_WIDTH]; FIELD_HEIGHT];
        let Apple(x, y) = self.apple;
        field[y as usize][x as usize] = Some(7);
        for (x, y) in &self.snake.body {
            field[*y as usize][*x as usize] = Some(self.score.level() as usize);
        }
        field
    }
    fn set_apple(&mut self) {
        let Apple(x, y) = Apple::default();
        match self.snake.body.contains(&(x, y)) {
            true => self.set_apple(),
            false => self.apple = Apple(x, y),
        };
    }
}

impl Console for Model {
    fn handle(&mut self, window: &mut Window, event: Event) -> bool {
        match event {
            Event::Focus => true,
            Event::KeyDown => {
                const LEFT: Key = Key::from_char('a');
                const RIGHT: Key = Key::from_char('d');
                const UP: Key = Key::from_char('w');
                const DOWN: Key = Key::from_char('s');
                match event_key() {
                    Key::Escape => match self.play {
                        true => self.play = false,
                        false => crate::Model::connect(window),
                    },
                    Key::Enter => {
                        if !self.play {
                            self.score = Score::default();
                            self.snake = Snake::default();
                            self.set_apple();
                            self.time = time::Instant::now();
                            self.play = true;
                        }
                    }
                    Key::Up | UP => self.snake.direction(0, -1),
                    Key::Down | DOWN => self.snake.direction(0, 1),
                    Key::Left | LEFT => self.snake.direction(-1, 0),
                    Key::Right | RIGHT => self.snake.direction(1, 0),
                    _ => return false,
                };
                window.redraw();
                true
            }
            _ => false,
        }
    }
    fn draw(&self, window: &mut Window) {
        window.background();
        match self.play {
            true => draw_field(window, self.map()),
            false => window.welcome(
                NAME,
                &[&["PRESS ENTER", "for play"], &["PRESS ESC", "for exit"]],
            ),
        }
    }
    fn update(&mut self, window: &mut Window) {
        if self.play
            && self.time.elapsed() >= time::Duration::from_millis(300 - 50 * self.score.level())
        {
            self.time = time::Instant::now();
            self.play = if let Some((x, y)) = self.snake.update() {
                let Apple(k, l) = self.apple;
                if (x, y) == (k, l) {
                    self.set_apple();
                    self.score.inc();
                } else {
                    self.snake.body.pop();
                }
                self.score.update();
                true
            } else {
                false
            };
            window.redraw();
        };
    }
}

fn draw_field(window: &mut Window, table: [[Option<usize>; FIELD_WIDTH]; FIELD_HEIGHT]) {
    draw::draw_rect_fill(0, 0, window.width(), window.height(), Color::Foreground);
    let pad: i32 = 1;
    let height: i32 =
        (window.height() - 2 * PAD - pad * (table.len() as i32 + 1)) / table.len() as i32;
    let ww = height * table[0].len() as i32 + pad * (table[0].len() as i32 - 1) + 2 * PAD;
    let hh = height * table.len() as i32 + pad * (table.len() as i32 - 1) + 2 * PAD;
    let x = (window.width() - ww) / 2;
    let y = (window.height() - hh) / 2;
    let mut xx = x;
    let mut yy = y;
    draw::draw_rect_fill(xx, yy, ww, hh, Color::Foreground);
    xx += PAD;
    yy += PAD;
    for line in table {
        for cell in line {
            draw::draw_rect_fill(
                xx,
                yy,
                height,
                height,
                match cell {
                    None => Color::Background2,
                    Some(idx) => [
                        Color::Green,
                        Color::Blue,
                        Color::Cyan,
                        Color::from_hex(0x6C71C4), //violet
                        Color::Magenta,
                        Color::Yellow,
                        Color::from_hex(0xCB4B16), //orange
                        Color::Red,
                    ][idx],
                },
            );
            xx += pad + height;
        }
        yy += pad + height;
        xx = x + PAD;
    }
}

const NAME: &str = "Game::Snake";
