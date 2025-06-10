use fltk::{app, misc::Tooltip, window::Window};
use std::sync::{Arc, RwLock};

pub use {
    cascade::cascade,
    comfy_table::{modifiers, presets, Table},
    fltk::{
        app::event_key,
        draw,
        enums::{Align, Color, Event, Font, Key},
        image::SvgImage,
        prelude::*,
    },
    glib::clone,
};

pub const PAD: i32 = 10;
pub const HEIGHT: i32 = 3 * PAD;
pub const WIDTH: i32 = 3 * HEIGHT;

#[derive(Default)]
pub struct Settings {
    pub size: (i32, i32),
    pub fullscreen: bool,
    pub font_size: u8,
    pub font: Option<Font>,
    pub xclass: Option<&'static str>,
    pub icon: Option<SvgImage>,
}

impl Settings {
    pub fn config(&self) -> Window {
        set_theme(0);
        app::set_visible_focus(false);
        app::set_font(self.font.unwrap_or(Font::CourierBold));
        Tooltip::set_color(Color::Background2);
        Tooltip::set_text_color(Color::Foreground);
        app::set_font_size(match self.font_size {
            0 => 14,
            _ => self.font_size,
        });
        let (w, h) = match self.size {
            (0, 0) => (360, 640),
            _ => self.size,
        };
        cascade!(
            Window::default().with_size(w, h).center_screen();
            ..set_xclass(&self.xclass.unwrap_or("FLTK"));
            ..size_range(w, h, 0, 0);
            ..fullscreen(self.fullscreen);
            ..make_resizable(true);
            ..set_icon(self.icon.clone());
            ..set_callback(move |window| {
                if let Some(mut child) = window.child(0) {
                    child.do_callback();
                    window.set_label(&child.label());
                };
            });
            ..end();
        )
    }
}

pub trait Game {
    fn handle(&mut self, widget: &mut impl WidgetExt, event: Event) -> bool;
    fn draw(&self, widget: &impl WidgetExt);
    fn update(&mut self) -> Option<bool>;
    fn run(settings: Settings) -> Result<(), FltkError>
    where
        Self: Sized + Default + 'static,
    {
        let state = Arc::new(RwLock::new(Self::default()));
        let mut container = cascade!(
            settings.config();
            ..show();
            ..set_callback(move |_widget| {
                if is_close() {
                    app::quit();
                }
            });
            ..handle(clone!(#[strong] state, move |widget, event| state.write().unwrap().handle(widget, event)));
            ..draw(clone!(#[strong] state, move |widget| state.read().unwrap().draw(widget)));
        );
        app::add_idle3(clone!(
            #[strong]
            state,
            move |_| {
                if let Some(value) = state.write().unwrap().update() {
                    if value {
                        container.redraw();
                    }
                    std::thread::sleep(std::time::Duration::from_millis(20));
                } else {
                    std::process::exit(0);
                }
            }
        ));
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
