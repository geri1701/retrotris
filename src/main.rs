mod games;
mod gui;
mod models;

use gui::*;

#[derive(Default)]
pub enum Game {
    #[default]
    Tetris,
    Snake,
    Bird,
    Pong,
    Runner,
}

impl Game {
    fn switch(&self) -> Self {
        match self {
            Self::Tetris => Self::Snake,
            Self::Snake => Self::Bird,
            Self::Bird => Self::Pong,
            Self::Pong => Self::Runner,
            Self::Runner => Self::Tetris,
        }
    }
    fn to_str(&self) -> &str {
        match self {
            Self::Tetris => "Tetris",
            Self::Snake => "Snake",
            Self::Bird => "Bird",
            Self::Pong => "Pong",
            Self::Runner => "Runner",
        }
    }
    fn to_play(&self, window: &mut Window) {
        match self {
            Self::Tetris => games::tetris::Model::connect(window),
            Self::Snake => games::snake::Model::connect(window),
            Self::Bird => games::bird::Model::connect(window),
            Self::Pong => games::pong::Model::connect(window),
            Self::Runner => games::runner::Model::connect(window),
        }
    }
}

#[derive(Default)]
pub struct Model(Game);

impl Console for Model {
    fn load(&mut self, _path: &str) {}
    fn exit(&self, _path: &str) {}
    fn update(&mut self, _dt: f32) {}
    fn handle(&mut self, window: &mut Window, event: Event) -> bool {
        match event {
            Event::Focus => true,
            Event::KeyDown => {
                match event_key() {
                    Key::Tab => self.0 = self.0.switch(),
                    Key::Enter => {
                        window.set_xclass(self.0.to_str());
                        self.0.to_play(window);
                    }
                    Key::Escape => std::process::exit(0),
                    _ => return false,
                }
                window.redraw();
                true
            }
            _ => false,
        }
    }
    fn draw(&self, window: &mut Window) {
        window.draw_background(Color::Background);
        window.draw_welcome(
            "Games",
            &[
                &["GAMES:", self.0.to_str()],
                &["PRESS TAB", "for switch"],
                &["PRESS ENTER", "for play"],
                &["PRESS ESC", "for exit"],
            ],
        );
    }
}

fn main() -> Result<(), FltkError> {
    Model::run(Settings {
        fullscreen: true,
        size: Some((SCREEN_WIDTH, SCREEN_HEIGHT)),
        icon: Some(SvgImage::from_data(include_str!("../assets/logo.svg")).unwrap()),
        ..Default::default()
    })
}
