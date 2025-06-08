use {crate::gui::*, crate::tetris_model::*, std::time};

const COLORS: [Color; 7] = [
    Color::Green,
    Color::Cyan,
    Color::Blue,
    Color::from_hex(0x6C71C4), //violet
    Color::Magenta,
    Color::Yellow,
    Color::from_hex(0xCB4B16), //orange
];

struct Model {
    score: Score,
    grid: Grid,
    curr: Figure,
    next: Next,
    time: time::Instant,
    play: bool,
    exit: bool,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            score: Score::default(),
            grid: Grid::default(),
            curr: Figure::default(),
            next: Next::default(),
            time: time::Instant::now(),
            play: false,
            exit: false,
        }
    }
}

impl Model {
    pub fn next(&mut self) {
        self.curr = Figure::new(10, 0, self.next.get(), 0);
        self.next = Next::default();
    }
    pub fn shift(&mut self, direction: (i32, usize)) {
        if let Some(temp) = self.grid.check(self.curr.shift(direction)) {
            self.curr = temp;
        }
    }
    pub fn rotate(&mut self) {
        if let Some(temp) = self.grid.check(self.curr.rotate()) {
            self.curr = temp;
        }
    }
    pub fn down(&mut self) -> bool {
        if let Some(temp) = self.grid.check(self.curr.shift((0, 1))) {
            self.curr = temp;
        } else {
            for (x, y) in self.curr.coor {
                if y == 0 {
                    return false;
                } else {
                    self.grid.0[y][x as usize] = Some(self.curr.shape.0);
                }
            }
            self.next();
        };
        true
    }
}

impl Game for Model {
    fn update(&mut self) -> Option<bool> {
        if self.exit {
            return None;
        }
        if self.play
            && self.time.elapsed() >= time::Duration::from_millis(500 - 50 * self.score.get().1)
        {
            self.time = time::Instant::now();
            for line in self.grid.find_full_line() {
                self.grid.0.remove(line);
                self.grid.0.insert(0, [None; GRID_WIDTH]);
                self.score.inc();
            }
            if !self.down() {
                self.play = false;
                return Some(true);
            }
            self.score.update();
        };
        Some(self.play)
    }
    fn draw(&self, widget: &impl WidgetExt) {
        draw::draw_rect_fill(0, 0, widget.width(), widget.height(), Color::Foreground);
        if self.play {
            draw::draw_rect_fill(0, 0, widget.width(), widget.height(), Color::Foreground);
            let (x, y, h) = draw_field(widget, &self.grid.draw(&self.curr));
            let (x, y) = draw_next(x, y, h, self.next.draw());
            draw_score(x, y, h, self.score.get().0);
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
                        self.next();
                        self.grid = Grid::default();
                        self.time = time::Instant::now();
                        self.play = !self.play;
                    }
                    Key::Up | UP => self.rotate(),
                    Key::Down | DOWN => {
                        self.down();
                    }
                    Key::Left | LEFT => self.shift((-1, 0)),
                    Key::Right | RIGHT => self.shift((1, 0)),
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

fn draw_field(flex: &impl WidgetExt, table: &Vec<Vec<Option<usize>>>) -> (i32, i32, i32) {
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

fn draw_score(x: i32, y: i32, h: i32, v: i32) {
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

const NAME: &str = "Game::Tetris";
pub fn play() -> Result<(), FltkError> {
    Model::run(Settings {
        size: (960, 540),
        fullscreen: true,
        icon: Some(SvgImage::from_data(include_str!("../assets/logo.svg")).unwrap()),
        ..Default::default()
    })
}
