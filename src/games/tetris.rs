use {crate::gui::*, crate::models::tetris::*};

const COLORS: [Color; 7] = [
    Color::Green,
    Color::Cyan,
    Color::Blue,
    Color::from_hex(0x6C71C4), //violet
    Color::Magenta,
    Color::Yellow,
    Color::from_hex(0xCB4B16), //orange
];

#[derive(Default)]
pub struct Model {
    score: Score,
    grid: Grid,
    curr: Figure,
    next: Next,
    play: bool,
    timer: f32,
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
                        self.next();
                        self.grid = Grid::default();
                        self.play = !self.play;
                    }
                    Key::Up | UP => self.rotate(),
                    Key::Down | DOWN => {
                        self.down();
                    }
                    Key::Left | LEFT => self.shift((-1, 0)),
                    Key::Right | RIGHT => self.shift((1, 0)),
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
        if self.timer < 1.0 / (3.0 + self.score.get().1 as f32) {
            return;
        }
        self.timer = 0.0;
        for line in self.grid.find_full_line() {
            self.grid.0.remove(line);
            self.grid.0.insert(0, [None; GRID_WIDTH]);
            self.score.inc();
        }
        if !self.down() {
            self.play = false;
        }
        self.score.update();
    }
    fn draw(&self, window: &mut Window) {
        window.draw_background(Color::Background);
        if self.play {
            draw::draw_rect_fill(0, 0, window.width(), window.height(), Color::Foreground);
            let (x, y, h) = draw_field(window, &self.grid.draw(&self.curr));
            let (x, y) = draw_next(x, y, h, self.next.draw());
            draw_score(x, y, h, self.score.get().0);
        } else {
            window.draw_welcome(
                "Tetris",
                &[
                    &["PRESS ENTER", "for play"],
                    &["PRESS ESC", "for exit"],
                    &["PRESS UP", "for rotate"],
                    &["PRESS DOWN", "for down"],
                    &["PRESS LEFT", "for left"],
                ],
            );
        }
    }
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
