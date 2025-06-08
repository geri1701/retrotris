pub use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::*,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub trait Sandbox {
    fn view(&self, frame: &mut Frame);
    fn update(&mut self) -> bool {
        false
    }
    fn handle(&mut self, event: Event) -> Option<bool>;
    fn run(&mut self) {
        let mut container = ratatui::init();
        if let Ok(size) = container.size() {
            self.handle(Event::Resize(size.width, size.height));
        };
        let _ = container.draw(|frame| self.view(frame));
        loop {
            let mut update = self.update();
            if event::poll(std::time::Duration::from_millis(20)).unwrap() {
                if let Some(value) = self.handle(event::read().unwrap()) {
                    update = update || value;
                } else {
                    ratatui::restore();
                    break;
                }
            }
            if update {
                let _ = container.draw(|frame| self.view(frame));
            }
        }
    }
}
