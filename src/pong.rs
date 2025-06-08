use crate::gui::*;
use crate::pong_model::*;

fn draw_field(flex: &impl WidgetExt) {
    draw::draw_rect_fill(0, 0, flex.width(), flex.height(), Color::Foreground);
    draw::draw_rect_fill(
        PAD,
        PAD,
        flex.width() - 2 * PAD,
        flex.height() - 2 * PAD,
        Color::Background,
    );
    draw::set_font(Font::CourierBold, 22);
}

fn draw_welcome(flex: &impl WidgetExt, title: &str) {
    draw::set_draw_color(Color::Green);
    draw::draw_text2(
        &figleter::FIGfont::standard()
            .unwrap()
            .convert(title)
            .unwrap()
            .to_string(),
        0,
        flex.height() / 2,
        flex.width(),
        HEIGHT,
        Align::Center,
    );
    draw::set_draw_color(Color::Red);
    draw::draw_text2(
        &cascade!(
            Table::new();
            ..load_preset(presets::UTF8_FULL);
            ..apply_modifier(modifiers::UTF8_ROUND_CORNERS);
            ..add_row(["PRESS ENTER"]);
        )
        .to_string(),
        0,
        flex.height() / 4 * 3,
        flex.width(),
        HEIGHT,
        Align::Center,
    );
}

#[derive(Default)]
struct Model {
    field: Field,
    paddle: Paddle,
    ball: Ball,
    score: Score,
    play: bool,
    exit: bool,
}

impl Game for Model {
    fn handle(&mut self, widget: &mut impl WidgetExt, event: Event) -> bool {
        match event {
            Event::Focus => true,
            Event::Resize => {
                self.field.update((widget.w(), widget.h()));
                self.paddle.set_y(widget.h());
                true
            }
            Event::KeyDown => {
                match event_key() {
                    Key::Escape => {
                        if !self.play {
                            self.exit = true;
                        }
                        self.play = false;
                    }
                    Key::Enter => {
                        self.score = Score::default();
                        self.ball = Ball::default();
                        self.paddle.set_x(PAD);
                        self.play = true;
                    }
                    _ => return false,
                }
                true
            }
            Event::Move => {
                draw::set_cursor(Cursor::None);
                if self.play
                    && (PAD + WIDTH..widget.width() - PAD - WIDTH).contains(&event_coords().0)
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
    fn draw(&self, widget: &impl WidgetExt) {
        let paddle = self.paddle.draw();
        let ball = self.ball.draw();
        let score = self.score.draw();
        draw_field(widget);
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
            draw_welcome(widget, NAME);
        }
    }
    fn update(&mut self) -> Option<bool> {
        if self.exit {
            return None;
        }
        if self.play {
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
                    return Some(true);
                };
            }
            self.score.update();
        };
        Some(self.play)
    }
}

const NAME: &str = "Game::Pong";
pub fn play() -> Result<(), FltkError> {
    Model::run(Settings {
        size: (960, 540),
        fullscreen: true,
        icon: Some(SvgImage::from_data(include_str!("../assets/logo.svg")).unwrap()),
        ..Default::default()
    })
}
