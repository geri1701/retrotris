mod games;
mod gui;
mod models;

use gui::*;

#[derive(Default)]
pub enum Game {
    #[default]
    Tetris,
    Snake,
}

impl Game {
    fn switch(&self) -> Self {
        match self {
            Self::Snake => Self::Tetris,
            Self::Tetris => Self::Snake,
        }
    }
    fn to_str(&self) -> &str {
        match self {
            Self::Snake => "Snake",
            Self::Tetris => "Tetris",
        }
    }
    fn to_play(&self, window: &mut Window) {
        match self {
            Self::Snake => games::snake::Model::connect(window),
            Self::Tetris => games::tetris::Model::connect(window),
        }
    }
}

#[derive(Default)]
pub struct Model(Game);

impl Console for Model {
    fn handle(&mut self, window: &mut Window, event: Event) -> bool {
        match event {
            Event::Focus => true,
            Event::KeyDown => {
                match event_key() {
                    Key::Tab => self.0 = self.0.switch(),
                    Key::Enter => self.0.to_play(window),
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
        window.background();
        window.welcome(
            self.0.to_str(),
            &[
                &["PRESS TAB", "for switch"],
                &["PRESS ENTER", "for play"],
                &["PRESS ESC", "for exit"],
            ],
        );
    }
}

fn main() -> Result<(), FltkError> {
    Model::run(Settings {
        size: Some((960, 540)),
        fullscreen: true,
        icon: Some(SvgImage::from_data(include_str!("../assets/logo.svg")).unwrap()),
        ..Default::default()
    })
}
