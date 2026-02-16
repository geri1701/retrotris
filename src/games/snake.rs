use crate::gui::*;
use crate::models::snake::*;

#[derive(Default)]
pub struct Model {
    field: (i32, i32),
    snake: Snake,
    score: Score,
    apple: Apple,
    play: bool,
    timer: f32,
}

impl Model {
    fn set_field(&mut self) {
        self.field = (COLS, ROWS);
    }
    fn set_apple(&mut self, field: (i32, i32)) {
        let Apple(x, y) = Apple::new(field);
        match self.snake.body.contains(&(x, y)) {
            true => self.set_apple(self.field),
            false => self.apple = Apple(x, y),
        };
    }
}

impl Console for Model {
    fn load(&mut self, _path: &str) {}
    fn exit(&self, _path: &str) {}
    fn handle(&mut self, window: &mut Window, event: Event) -> bool {
        match event {
            Event::Focus => true,
            Event::Resize => {
                self.snake.set((window.w(), window.h()));
                self.play = false;
                true
            }
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
                            self.set_field();
                            self.snake.set(self.field);
                            self.score = Score::default();
                            self.set_apple(self.field);
                            self.play = true;
                        }
                    }
                    Key::Up | UP => self.snake.direction(0, -1),
                    Key::Down | DOWN => self.snake.direction(0, 1),
                    Key::Left | LEFT => self.snake.direction(-1, 0),
                    Key::Right | RIGHT => self.snake.direction(1, 0),
                    _ => return false,
                };
                true
            }
            _ => false,
        }
    }
    fn update(&mut self, dt: f32) {
        if !self.play {
            return;
        }
        self.timer += dt;
        if self.timer < 1.0 / (3.0 + self.score.level() as f32) {
            return;
        }
        self.timer = 0.0;
        self.play = if let Some((x, y)) = self.snake.update() {
            let Apple(k, l) = self.apple;
            if (x, y) == (k, l) {
                self.set_apple(self.field);
                self.score.inc();
            } else {
                self.snake.body.pop();
            }
            self.score.update();
            true
        } else {
            false
        };
    }
    fn draw(&self, window: &mut Window) {
        // BACKGROUND
        draw::draw_rect_fill(0, 0, window.width(), window.height(), Color::Background);
        if self.play {
            // GRID
            let cell = window.width() / COLS;
            for x in 0..COLS {
                for y in 0..ROWS {
                    if (x + y) % 2 == 0 {
                        draw::draw_rect_fill(x * cell, y * cell, cell, cell, Color::Background2);
                    }
                }
            }
            // SNAKE BODY
            for &seg in &self.snake.body {
                draw::draw_rect_fill(seg.0 * cell, seg.1 * cell, cell, cell, Color::Cyan);
            }
            // SNAKE HEAD
            draw::draw_rect_fill(
                self.snake.body[0].0 * cell,
                self.snake.body[0].1 * cell,
                cell,
                cell,
                Color::Green,
            );
            // APPLE
            draw::draw_circle_fill(self.apple.0 * cell, self.apple.1 * cell, cell, Color::Red);
        } else {
            window.draw_welcome(
                "Snake",
                &[&["PRESS ENTER", "for play"], &["PRESS ESC", "for exit"]],
            );
        }
    }
}
