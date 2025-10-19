pub use {
    cascade::cascade,
    comfy_table::{modifiers, presets, Table},
    fltk::{
        app,
        app::event_coords,
        app::event_key,
        draw,
        enums::{Align, Color, Cursor, Event, Font, Key},
        image::SvgImage,
        misc::Tooltip,
        prelude::*,
        window::Window,
    },
    std::{
        sync::{Arc, RwLock},
        time::{Duration, Instant},
    },
};

pub const SCREEN_WIDTH: i32 = 960;
pub const SCREEN_HEIGHT: i32 = 540;
pub const COLS: i32 = SCREEN_WIDTH / 20;
pub const ROWS: i32 = SCREEN_HEIGHT / 20;

pub const PAD: i32 = 10;
pub const HEIGHT: i32 = 3 * PAD;
pub const WIDTH: i32 = 3 * HEIGHT;

#[derive(Default)]
pub struct Settings {
    pub fullscreen: bool,
    pub size: Option<(i32, i32)>,
    pub font_size: Option<u8>,
    pub font: Option<Font>,
    pub xclass: Option<&'static str>,
    pub icon: Option<SvgImage>,
}

impl Settings {
    pub fn config(&self) -> Window {
        set_theme(0);
        app::set_font(self.font.unwrap_or(Font::CourierBold));
        app::set_font_size(self.font_size.unwrap_or(14));
        let (w, h) = self.size.unwrap_or((360, 640));
        cascade!(
            Window::default().with_size(w, h).center_screen();
            ..set_xclass(&self.xclass.unwrap_or("FLTK"));
            ..size_range(w, h, 0, 0);
            ..fullscreen(self.fullscreen);
            ..make_resizable(true);
            ..set_icon(self.icon.clone());
            ..end();
            ..show();
        )
    }
}

pub trait Console
where
    Self: Default + 'static,
{
    fn handle(&mut self, window: &mut Window, event: Event) -> bool;
    fn draw(&self, window: &mut Window);
    fn update(&mut self, dt: f32);
    fn connect(window: &mut Window) {
        let state = Arc::new(RwLock::new(Self::default()));
        let mut time = Instant::now();
        window.draw({
            let state = state.clone();
            move |window| {
                state.write().unwrap().update(time.elapsed().as_secs_f32());
                state.read().unwrap().draw(window);
                time = Instant::now();
            }
        });
        window.handle({
            let state = state.clone();
            move |window, event| state.write().unwrap().handle(window, event)
        });
        window.handle_event(Event::Resize);
        window.set_callback(move |window| {
            if is_close() {
                window.hide();
            }
        });
    }
    fn run(settings: Settings) -> Result<(), FltkError> {
        let mut window = settings.config();
        Self::connect(&mut window);
        app::add_idle3(move |_| {
            window.redraw();
            std::thread::sleep(Duration::from_millis(20));
        });
        app::App::default().run()
    }
}

pub fn is_close() -> bool {
    app::event() == Event::Close
}

pub fn set_theme(theme: usize) {
    const COLOR: [[u32; 5]; 4] = [
        [
            //SOLARIZED LIGHT
            0xEEE8D5, //base2
            0xFDF6E3, //base3
            0x586E75, //base01
            0xCB4B16, //orange
            0xB58900, //yellow
        ],
        [
            //SOLARIZED DARK
            0x073642, //base02
            0x002B36, //base03
            0x93A1A1, //base1
            0x268BD2, //blue
            0x6C71C4, //violet
        ],
        [
            //ADWAITA LIGHT
            0xF6F5F4, //set_background_color
            0xFCFCFC, //set_background2_color
            0x323232, //set_foreground_color
            0x6C71C4, //set_selection_color
            0x3584E4, //set_inactive_color
        ],
        [
            //ADWAITA DARK
            0x353535, //set_background_color
            0x303030, //set_background2_color
            0xD6D6D6, //set_foreground_color
            0x268BD2, //set_selection_color
            0x15539E, //set_inactive_color
        ],
    ];
    let color = COLOR[theme];
    Tooltip::set_color(Color::Background2);
    Tooltip::set_text_color(Color::Foreground);
    //app::set_visible_focus(false);
    app::set_scheme(match theme % 2 {
        1 => app::Scheme::Gtk,
        _ => app::Scheme::Oxy,
    });
    let (r, g, b) = Color::from_hex(color[0]).to_rgb();
    app::set_background_color(r, g, b);
    let (r, g, b) = Color::from_hex(color[1]).to_rgb();
    app::set_background2_color(r, g, b);
    let (r, g, b) = Color::from_hex(color[2]).to_rgb();
    app::set_foreground_color(r, g, b);
    let (r, g, b) = Color::from_hex(color[3]).to_rgb();
    app::set_selection_color(r, g, b);
    let (r, g, b) = Color::from_hex(color[4]).to_rgb();
    app::set_inactive_color(r, g, b);
    for (hex, color) in [
        (0xB58900, Color::Yellow),
        (0xDC322F, Color::Red),
        (0xD33682, Color::Magenta),
        (0x268BD2, Color::Blue),
        (0x2AA198, Color::Cyan),
        (0x859900, Color::Green),
    ] {
        let (r, g, b) = Color::from_hex(hex).to_rgb();
        app::set_color(color, r, g, b);
    }
    app::set_visible_focus(false);
    app::redraw();
}

pub trait Paint {
    fn draw_welcome(&self, title: &str, menu: &[&[&str]]);
    fn draw_background(&self, color: Color);
}

impl Paint for Window {
    fn draw_background(&self, color: Color) {
        draw::draw_rect_fill(0, 0, self.width(), self.height(), color);
    }
    fn draw_welcome(&self, title: &str, menu: &[&[&str]]) {
        draw::set_font(Font::CourierBold, 22);
        draw::set_draw_color(Color::Green);
        draw::draw_text2(
            &figleter::FIGfont::standard()
                .unwrap()
                .convert(title)
                .unwrap()
                .to_string(),
            0,
            self.height() / 4,
            self.width(),
            HEIGHT,
            Align::Center,
        );
        draw::set_draw_color(Color::Red);
        draw::draw_text2(
            &{
                let mut table = Table::new();
                table.load_preset(presets::UTF8_FULL);
                table.apply_modifier(modifiers::UTF8_ROUND_CORNERS);
                for row in menu {
                    table.add_row(*row);
                }
                table
            }
            .to_string(),
            0,
            self.height() / 4 * 3,
            self.width(),
            PAD * 2,
            Align::Center,
        );
    }
}
