mod gui;
mod launcher;
mod pong;
mod pong_model;
mod snake;
mod snake_model;
mod tetris;
mod tetris_model;
mod tui;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        launcher::choice();
    } else {
        match &args[1][..] {
            "pong" => launcher::play(launcher::Game::Pong),
            "snake" => launcher::play(launcher::Game::Snake),
            "tetris" => launcher::play(launcher::Game::Tetris),
            _ => launcher::choice(),
        }
    }
}
