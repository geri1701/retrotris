use crate::gui::*;
use crate::models::pong::*;

#[derive(Default)]
pub struct Model {
    field: Field,
    paddle: Paddle,
    ball: Ball,
    score: Score,
    play: bool,
}

impl Console for Model {
    fn handle(&mut self, window: &mut Window, event: Event) -> bool {
        match event {
            Event::Focus => true,
            Event::Resize => {
                self.field.update((window.w(), window.h()));
                self.paddle.set_y(window.h());
                self.play = false;
                true
            }
            Event::KeyDown => {
                const LEFT: Key = Key::from_char('a');
                const RIGHT: Key = Key::from_char('d');
                match event_key() {
                    Key::Escape => match self.play {
                        true => self.play = false,
                        false => crate::Model::connect(window),
                    },
                    Key::Enter => {
                        if !self.play {
                            self.score = Score::default();
                            self.ball = Ball::default();
                            self.paddle.set_x(PAD);
                            self.play = true;
                        }
                    }
                    Key::Left | LEFT => {
                        if self.play {
                            self.paddle.left();
                        }
                    }
                    Key::Right | RIGHT => {
                        if self.play {
                            self.paddle.right();
                        }
                    }
                    _ => return false,
                }
                true
            }
            Event::Move => {
                draw::set_cursor(Cursor::None);
                if self.play
                    && (PAD + WIDTH..window.width() - PAD - WIDTH).contains(&event_coords().0)
                {
                    self.paddle.set_x(event_coords().0 - WIDTH);
                };
                true
            }
            Event::Leave => {
                draw::set_cursor(Cursor::Arrow);
                true
            }
            _ => false,
        }
    }
    fn update(&mut self, _dt: f32) {
        if !self.play {
            return;
        }
        for _ in 0..(PAD + 5 * self.score.1) {
            self.ball.step();
            if self.ball.check_field(&self.field) {
                self.score.dec();
            }
            if self.ball.check_paddle(&self.paddle) {
                self.score.inc();
            }
            if self.score.0 < 0 {
                self.play = false;
            };
        }
        self.score.update();
    }
    fn draw(&self, window: &mut Window) {
        let paddle = self.paddle.draw();
        let ball = self.ball.draw();
        let score = self.score.draw();
        draw::draw_rect_fill(0, 0, window.width(), window.height(), Color::Background);
        if self.play {
            draw::draw_circle_fill(ball.0, ball.1, ball.2, Color::Selection);
            draw::draw_rect_fill(paddle.0, paddle.1, paddle.2, paddle.3, Color::Inactive);
            draw::set_draw_color(Color::Background2);
            draw::draw_text2(
                &score.1.to_string(),
                ball.0,
                ball.1,
                ball.2,
                ball.2,
                Align::Center,
            );
            draw::draw_text2(
                &score.0.to_string(),
                paddle.0,
                paddle.1,
                paddle.2,
                paddle.3,
                Align::Center,
            );
        } else {
            window.draw_welcome(
                "Pong",
                &[&["PRESS ENTER", "for play"], &["PRESS ESC", "for exit"]],
            );
        }
    }
}
