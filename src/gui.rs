pub use {
    comfy_table::{Table, modifiers, presets},
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
    rand::Rng,
    std::time::Instant,
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
        let mut wgt = Window::default().with_size(w, h).center_screen();
        wgt.set_xclass(self.xclass.unwrap_or("FLTK"));
        wgt.size_range(w, h, 0, 0);
        wgt.fullscreen(self.fullscreen);
        wgt.make_resizable(true);
        wgt.set_icon(self.icon.clone());
        wgt.end();
        wgt.show();
        wgt
    }
}

pub trait Console
where
    Self: Default + 'static,
{
    fn load(&mut self, path: &str);
    fn exit(&self, path: &str);
    fn handle(&mut self, window: &mut Window, event: Event) -> bool;
    fn draw(&self, window: &mut Window);
    fn update(&mut self, dt: f32);
    fn connect(window: &mut Window) {
        let path = format!(
            "{}/.config/{}",
            std::env::var("HOME").unwrap(),
            window.xclass().unwrap(),
        );
        let state = std::rc::Rc::new(std::cell::RefCell::new(Self::default()));
        state.borrow_mut().load(&path);
        let mut time = Instant::now();
        window.draw({
            let state = state.clone();
            move |window| {
                state.borrow_mut().update(time.elapsed().as_secs_f32());
                state.borrow().draw(window);
                time = Instant::now();
            }
        });
        window.handle({
            let state = state.clone();
            move |window, event| state.borrow_mut().handle(window, event)
        });
        window.handle_event(Event::Resize);
        window.set_callback(move |window| {
            if app::event() == Event::Close {
                state.borrow().exit(&path);
                window.hide();
            }
        });
    }
    fn run(settings: Settings) -> Result<(), FltkError> {
        let mut window = settings.config();
        Self::connect(&mut window);
        const TICK: f64 = 0.02;
        app::add_timeout3(TICK, move |handle| {
            window.redraw();
            app::repeat_timeout3(TICK, handle);
        });
        app::App::default().run()
    }
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

pub trait Painter {
    fn draw_rect(&self, x: i32, y: i32, w: i32, h: i32, r: i32, color: Color);
    fn draw_text(&self, line: &str, x: i32, y: i32, color: Color, align: Align, size: i32);
    fn draw_welcome(&self, title: &str, menu: &[&[&str]]);
    fn draw_background(&self, color: Color);
    fn draw_overlay(&self, title: &str, subtitle: &str, color: Color);
}

impl Painter for Window {
    fn draw_rect(&self, x: i32, y: i32, w: i32, h: i32, r: i32, color: Color) {
        draw::set_draw_color(color);
        draw::draw_rounded_rectf(x, y, w, h, r);
    }
    fn draw_text(&self, line: &str, x: i32, y: i32, color: Color, align: Align, size: i32) {
        draw::set_font(Font::CourierBold, size);
        draw::set_draw_color(color);
        let (w, h) = draw::measure(line, false);
        draw::draw_text2(line, x, y, w, h, align);
    }
    fn draw_overlay(&self, title: &str, subtitle: &str, color: Color) {
        draw::set_draw_color(color);
        draw::set_font(Font::CourierBold, 42);
        let (mut w, mut h) = draw::measure(title, false);
        draw::draw_text2(
            title,
            self.w() / 2 - w / 2,
            self.h() / 3 - h,
            w,
            h,
            Align::Left,
        );
        draw::set_font(Font::CourierBold, 24);
        (w, h) = draw::measure(subtitle, false);
        draw::draw_text2(
            subtitle,
            self.w() / 2 - w / 2,
            self.h() / 2 - h,
            w,
            h,
            Align::Left,
        );
    }
    fn draw_background(&self, color: Color) {
        draw::draw_rect_fill(0, 0, self.w(), self.h(), color);
    }
    fn draw_welcome(&self, title: &str, menu: &[&[&str]]) {
        draw::set_font(Font::CourierBold, 20);
        draw::set_draw_color(Color::Green);
        draw::draw_text2(
            &figleter::FIGfont::standard()
                .unwrap()
                .convert(title)
                .unwrap()
                .to_string(),
            0,
            self.h() / 4,
            self.w(),
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
            self.h() / 3 * 2,
            self.w(),
            HEIGHT,
            Align::Center,
        );
    }
}
