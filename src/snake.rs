use crate::gui::*;
use crate::snake_model::*;
use std::time;

struct Model {
    snake: Snake,
    score: Score,
    apple: Apple,
    time: time::Instant,
    play: bool,
    exit: bool,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            snake: Snake::default(),
            score: Score::default(),
            apple: Apple::default(),
            time: time::Instant::now(),
            play: false,
            exit: false,
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

impl Game for Model {
    fn update(&mut self) -> Option<bool> {
        if self.exit {
            return None;
        }
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
            }
        };
        Some(self.play)
    }
    fn draw(&self, widget: &impl WidgetExt) {
        draw::draw_rect_fill(0, 0, widget.width(), widget.height(), Color::Foreground);
        if self.play {
            draw_field(widget, self.map());
        } else {
            draw_welcome(widget, NAME);
        }
    }
    fn handle(&mut self, _widget: &mut impl WidgetExt, event: Event) -> bool {
        match event {
            Event::Focus => true,
            Event::KeyDown => {
                const LEFT: Key = Key::from_char('a');
                const RIGHT: Key = Key::from_char('d');
                const UP: Key = Key::from_char('w');
                const DOWN: Key = Key::from_char('s');
                match event_key() {
                    Key::Escape => {
                        if !self.play {
                            self.exit = true;
                        }
                        self.play = false;
                    }
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
                }
                true
            }
            _ => false,
        }
    }
}

fn draw_welcome(flex: &impl WidgetExt, title: &str) {
    draw::draw_rect_fill(
        PAD,
        PAD,
        flex.width() - 2 * PAD,
        flex.height() - 2 * PAD,
        Color::Background,
    );
    draw::set_font(Font::CourierBold, 22);
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

fn draw_field(
    flex: &impl WidgetExt,
    table: [[Option<usize>; FIELD_WIDTH]; FIELD_HEIGHT],
) -> (i32, i32) {
    draw::draw_rect_fill(0, 0, flex.width(), flex.height(), Color::Foreground);
    let pad: i32 = 1;
    let height: i32 =
        (flex.height() - 2 * PAD - pad * (table.len() as i32 + 1)) / table.len() as i32;
    let ww = height * table[0].len() as i32 + pad * (table[0].len() as i32 - 1) + 2 * PAD;
    let hh = height * table.len() as i32 + pad * (table.len() as i32 - 1) + 2 * PAD;
    let x = (flex.width() - ww) / 2;
    let y = (flex.height() - hh) / 2;
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
    (height, pad)
}

const NAME: &str = "Game::Snake";
pub fn play() -> Result<(), FltkError> {
    Model::run(Settings {
        size: (960, 540),
        fullscreen: true,
        icon: Some(SvgImage::from_data(include_str!("../assets/logo.svg")).unwrap()),
        ..Default::default()
    })
}
