mod elmish;
mod model;

use {elmish::*, std::time};

const COLORS: [Color; 7] = [
    Color::Green,
    Color::Cyan,
    Color::Blue,
    Color::from_hex(0x6C71C4), //violet
    Color::Magenta,
    Color::Yellow,
    Color::from_hex(0xCB4B16), //orange
];

enum Message {
    Play,
    Down,
    Rotate,
    Shift(i32, usize),
    Quit,
}

struct Model {
    game: model::Model,
    time: time::Instant,
    play: bool,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            game: model::Model::default(),
            time: time::Instant::now(),
            play: false,
        }
    }
}
impl Sandbox for Model {
    type Msg = Message;

    fn subscription(&mut self) -> bool {
        if self.play
            && self.time.elapsed() >= time::Duration::from_millis(500 - 50 * self.game.value.1)
        {
            self.time = time::Instant::now();
            if !self.game.play() {
                self.play = false;
                return true;
            };
        };
        self.play
    }

    fn update(&mut self, _sender: Sender<Self::Msg>, message: Self::Msg) -> Option<bool> {
        match message {
            Self::Msg::Quit => None,
            Self::Msg::Play => {
                self.game.set_curr();
                self.game.set_grid();
                self.time = time::Instant::now();
                self.play = !self.play;
                Some(false)
            }
            Self::Msg::Down => {
                if self.play {
                    self.game.down();
                }
                Some(self.play)
            }
            Self::Msg::Rotate => {
                if self.play {
                    self.game.rotate();
                }
                Some(self.play)
            }
            Self::Msg::Shift(x, y) => {
                if self.play {
                    self.game.shift((x, y));
                }
                Some(self.play)
            }
        }
    }

    fn view(&self, sender: Sender<Self::Msg>) -> Flex {
        const NAME: &str = "Game::Tetris";
        let play = self.play;
        let value = self.game.value;
        let field = self.game.field();
        let next = self.game.next();
        cascade!(
            Flex::default_fill();
            ..set_label(NAME);
            ..set_frame(FrameType::FlatBox);
            ..set_margin(0);
            ..set_pad(0);
            ..end();
            ..set_callback(clone!(#[strong] sender, move |flex| {
                flex.take_focus().unwrap();
                if is_close() {
                    sender.send(Self::Msg::Quit).unwrap();
                }
            }));
            ..handle(clone!(#[strong] sender, move |_, event| match event {
                Event::Focus => true,
                Event::KeyDown => {
                    const LEFT: Key = Key::from_char('a');
                    const RIGHT: Key = Key::from_char('d');
                    const UP: Key = Key::from_char('w');
                    const DOWN: Key = Key::from_char('s');
                    match app::event_key() {
                        Key::Escape => sender.send(Self::Msg::Quit).unwrap(),
                        Key::Enter => sender.send(Self::Msg::Play).unwrap(),
                        Key::Up | UP => sender.send(Self::Msg::Rotate).unwrap(),
                        Key::Down | DOWN => sender.send(Self::Msg::Down).unwrap(),
                        Key::Left | LEFT => sender.send(Self::Msg::Shift(-1, 0)).unwrap(),
                        Key::Right | RIGHT => sender.send(Self::Msg::Shift(1, 0)).unwrap(),
                        _ => return false,
                    }
                    true
                }
                _ => false,
            }));
            ..draw(move |flex| {
                draw::draw_rect_fill(0, 0, flex.width(), flex.height(), Color::Foreground);
                if play {
                    draw::draw_rect_fill(0, 0, flex.width(), flex.height(), Color::Foreground);
                    let (x, y, h) = draw_field(flex, &field);
                    let (x, y) = draw_next(x, y, h, next);
                    draw_value(x, y, h, value.0);
                } else {
                    draw_welcome(flex, NAME);
                }
            });
        )
    }
}

fn draw_welcome(flex: &Flex, title: &str) {
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

fn draw_field(flex: &Flex, table: &Vec<Vec<Option<usize>>>) -> (i32, i32, i32) {
    let pad: i32 = 1;
    let height: i32 =
        (flex.height() - 2 * PAD - pad * (table.len() as i32 + 1)) / table.len() as i32;
    let ww = height * table[0].len() as i32 + pad * (table[0].len() as i32 - 1) + 2 * PAD;
    let hh = height * table.len() as i32 + pad * (table.len() as i32 - 1) + 2 * PAD;
    let x = (flex.width() - ww) / 2;
    let y = (flex.height() - hh) / 2;
    let mut xx = x;
    let mut yy = y;
    let mut xxx = 0;
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
                    Some(idx) => COLORS[*idx],
                },
            );
            xx += pad + height;
            xxx = xx
        }
        yy += pad + height;
        xx = x + PAD;
    }
    (xxx + PAD, y, height)
}

fn draw_next(x: i32, y: i32, height: i32, table: [[Option<usize>; 4]; 4]) -> (i32, i32) {
    let pad: i32 = 1;
    let mut xx = x;
    let mut yy = y;
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
                    None => Color::Foreground,
                    Some(idx) => COLORS[idx],
                },
            );
            xx += pad + height;
        }
        yy += pad + height;
        xx = x + PAD;
    }
    (xx, yy)
}

fn draw_value(x: i32, y: i32, h: i32, v: i32) {
    let mut yy = y + 1 + h;
    draw::set_draw_color(Color::Background2);
    draw::set_font(Font::CourierBold, h);
    draw::draw_text2(&format!("Score:\t{v}"), x, yy, WIDTH, h, Align::Left);
    for line in [
        "PRESS:",
        "\t<UP>\trotate",
        "\t<DOWN>\tfast down",
        "\t<LEFT>\tmove left",
        "\t<RIGHT>\tmove right",
        "\t<ENTER>\texit to menu",
        "\t<ESC>\texit from game",
    ] {
        yy += 2 * h + 2;
        draw::draw_text2(line, x, yy, line.len() as i32 * h, h, Align::Left);
    }
}

fn main() -> Result<(), FltkError> {
    Model::run(Settings {
        size: (960, 540),
        fullscreen: true,
        icon: Some(SvgImage::from_data(include_str!("../assets/logo.svg")).unwrap()),
        ..Default::default()
    })
}
