use crate::tui::*;

#[derive(Default)]
pub enum Game {
    #[default]
    Pong,
    Snake,
    Tetris,
}

impl Game {
    fn switch(&self) -> Self {
        match self {
            Self::Pong => Self::Snake,
            Self::Snake => Self::Tetris,
            Self::Tetris => Self::Pong,
        }
    }
    fn to_str(&self) -> &str {
        match self {
            Self::Pong => "Pong",
            Self::Snake => "Snake",
            Self::Tetris => "Tetris",
        }
    }
}

#[derive(Default)]
struct Model {
    game: Game,
}

impl Sandbox for Model {
    fn handle(&mut self, event: Event) -> Option<bool> {
        match event {
            Event::Key(value) => match value.code {
                KeyCode::Enter => None,
                KeyCode::Tab => {
                    self.game = self.game.switch();
                    Some(true)
                }
                _ => Some(false),
            },
            _ => Some(false),
        }
    }
    fn view(&self, frame: &mut Frame) {
        const NAME: &str = "Retro::Games";
        let outer = Block::bordered()
            .border_style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .title(Line::from("[ WELCOME ]".bold()).centered());
        let [title, body] = Layout::vertical([Constraint::Length(6), Constraint::Min(0)])
            .areas(outer.inner(frame.area()));
        let [_left, center, _right] = Layout::horizontal([
            Constraint::Min(0),
            Constraint::Length(45),
            Constraint::Min(0),
        ])
        .areas(body);
        frame.render_widget(outer, frame.area());
        frame.render_widget(
            Paragraph::new(
                figleter::FIGfont::standard()
                    .unwrap()
                    .convert(NAME)
                    .unwrap()
                    .to_string(),
            )
            .style(Style::default().fg(Color::Red))
            .centered(),
            title,
        );
        frame.render_widget(
            Paragraph::new(format!(
                r#"
Press <TAB>    to choice game: {}
      <ENTER>  to start
"#,
                self.game.to_str(),
            ))
            .style(Style::default().fg(Color::Green))
            .block(Block::default().borders(Borders::NONE)),
            center,
        );
    }
}

pub fn choice() {
    let mut steam = Model::default();
    steam.run();
    play(steam.game);
}

pub fn play(game: Game) {
    match game {
        Game::Pong => crate::pong::play(),
        Game::Snake => crate::snake::play(),
        Game::Tetris => crate::tetris::play(),
    }
    .unwrap();
}
