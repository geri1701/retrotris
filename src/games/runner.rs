use crate::gui::*;
use crate::models::runner::*;
const PADDING: i32 = 34;
#[derive(Default)]
pub struct Model {
    scene: Scene,
    difficulty: Difficulty,
    dino: Dino,
    size: (i32, i32),
    run_speed: f32,
    distance_x: f32,
    ground_scroll: f32,
    obstacles: Vec<Obstacle>,
    next_gap: f32,
    score: u32,
    high_score: u32,
}
impl Console for Model {
    fn load(&mut self, _path: &str) {}
    fn exit(&self, _path: &str) {}
    fn handle(&mut self, window: &mut Window, event: Event) -> bool {
        match event {
            Event::Focus => true,
            Event::Resize => {
                self.size = (window.w(), window.h());
                self.scene = Scene::Welcome;
                true
            }
            Event::KeyDown => {
                match event_key() {
                    Key::Escape => match self.scene {
                        Scene::Welcome => crate::Model::connect(window),
                        _ => self.scene = Scene::Welcome,
                    },
                    Key::Enter => match self.scene {
                        Scene::Playing => self.scene = Scene::Paused,
                        Scene::Paused => self.scene = Scene::Playing,
                        Scene::GameOver => self.scene = Scene::Welcome,
                        Scene::Welcome => {
                            self.setup();
                            self.scene = Scene::Playing;
                        }
                    },
                    Key::Up => {
                        if let Scene::Playing = self.scene
                            && !self.dino.jumping
                        {
                            self.dino.jumping = true;
                            self.dino.vy = -900.0;
                        }
                    }
                    Key::Down => {
                        if let Scene::Playing = self.scene {
                            self.dino.ducking = true
                        }
                    }
                    Key::Tab => {
                        if let Scene::Welcome = self.scene {
                            self.difficulty = self.difficulty.switch();
                        }
                    }
                    _ => return false,
                };
                true
            }
            Event::KeyUp => {
                match event_key() {
                    Key::Down => {
                        if let Scene::Playing = self.scene {
                            self.dino.ducking = false
                        }
                    }
                    _ => return false,
                };
                true
            }
            _ => false,
        }
    }
    fn update(&mut self, dt: f32) {
        if let Scene::Playing = self.scene {
            let ground_top = self.ground_y() - self.dino.height();
            if self.dino.jumping {
                self.dino.vy += 2400.0 * dt;
                self.dino.y += self.dino.vy * dt;
                self.dino.jumping = self.dino.y < ground_top;
            } else {
                self.dino.y = ground_top;
                self.dino.vy = 0.0;
                self.dino.leg_timer += dt;
                if self.dino.leg_timer >= 0.12 {
                    self.dino.leg_timer = 0.0;
                    self.dino.leg_state = !self.dino.leg_state;
                }
            }
            self.run_speed *= (1.0 + (0.06 * dt)).min(1.02);
            let dx = self.run_speed * dt;
            self.distance_x += dx;
            self.ground_scroll = (self.ground_scroll + dx) % 32.0;
            self.score = (self.score as f32 + 120.0 * dt) as u32;
            self.next_gap -= dx;
            if self.next_gap <= 0.0 {
                let kind = match self.difficulty.ptero_enabled()
                    && self.score > 300
                    && rand::rng().random_bool(0.35)
                {
                    true => ObstacleKind::Pterodactyl(rand::rng().random_bool(0.5)),
                    false => ObstacleKind::Cactus(rand::rng().random_bool(0.4)),
                };
                let (y, w, h) = match kind {
                    ObstacleKind::Cactus(tall) => match tall {
                        true => (self.ground_y() - 46.0, 24.0, 46.0),
                        false => (self.ground_y() - 60.0, 36.0, 60.0),
                    },
                    ObstacleKind::Pterodactyl(high) => match high {
                        true => (self.ground_y() - 48.0 - 22.0 - 24.0, 46.0, 24.0),
                        false => (self.ground_y() - 30.0 - 10.0 - 24.0, 46.0, 24.0),
                    },
                };
                let range = if let ObstacleKind::Pterodactyl(..) = kind {
                    const PTERO_MIN_GAP: f32 = 520.0;
                    const PTERO_MAX_GAP: f32 = 760.0;
                    PTERO_MIN_GAP * self.difficulty.gap_scale()
                        ..PTERO_MAX_GAP * self.difficulty.gap_scale()
                } else {
                    let (min, max) = self.difficulty.limit();
                    min..max
                };
                self.next_gap = rand::rng().random_range(range);
                self.obstacles.push(Obstacle {
                    rect: ((self.size.0 - PADDING) as f32 - w, y, w, h),
                    kind,
                });
            }
            for ob in &mut self.obstacles {
                ob.rect.0 -= dx;
            }
            self.obstacles.retain(|o| o.rect.0 >= PADDING as f32);
            if self.obstacles.iter().any(|obstacle| {
                rect_intersect(
                    draw::Rect::new(
                        self.dino.x() as i32,
                        self.dino.y as i32,
                        self.dino.w() as i32,
                        self.dino.height() as i32,
                    ),
                    draw::Rect::new(
                        obstacle.rect.0 as i32,
                        obstacle.rect.1 as i32,
                        obstacle.rect.2 as i32,
                        obstacle.rect.3 as i32,
                    ),
                )
            }) {
                self.high_score = self.high_score.max(self.score);
                self.scene = Scene::GameOver;
            }
        }
    }
    fn draw(&self, window: &mut Window) {
        window.draw_background(Color::Background);
        window.draw_rect(
            PADDING,
            PADDING,
            window.w() - PADDING * 2,
            window.h() - PADDING * 2,
            15,
            Color::Background2,
        );
        window.draw_text(
            &format!("Score: {:05}   High: {:05}", self.score, self.high_score),
            PADDING,
            2,
            Color::Foreground,
            Align::Left,
            22,
        );
        window.draw_rect(
            PADDING,
            self.ground_y() as i32,
            window.w() - PADDING * 2,
            4,
            5,
            Color::Foreground,
        );
        match self.scene {
            Scene::Playing => self.draw_game(),
            Scene::Welcome => window.draw_welcome(
                "T-Rex Runner",
                &[
                    &["PRESS <ENTER>", "for play"],
                    &["PRESS <ESC>", "for exit"],
                    &[
                        "PRESS <TAB>",
                        &format!("for level: {}", self.difficulty.label()),
                    ],
                ],
            ),
            Scene::Paused => {
                self.draw_game();
                window.draw_overlay("Paused", "<ENTER> or <ESC>: Menu", Color::Foreground);
            }
            Scene::GameOver => {
                self.draw_game();
                window.draw_overlay("Game Over", "<ENTER> or <ESC>: Menu", Color::Foreground);
            }
        }
    }
}
impl Model {
    fn ground_y(&self) -> f32 {
        (self.size.1 - PADDING) as f32 - 64.0
    }
    fn setup(&mut self) {
        self.dino = Dino::default();
        self.dino.y = self.ground_y() - self.dino.height();
        self.distance_x = 0.0;
        self.ground_scroll = 0.0;
        self.score = 0;
        const BASE_RUN_SPEED: f32 = 360.0;
        self.run_speed = BASE_RUN_SPEED * self.difficulty.speed_mul();
        self.obstacles.clear();
        self.next_gap = self.difficulty.scaled_gap(true);
    }
    fn draw_game(&self) {
        draw::draw_rect_fill(
            self.dino.x() as i32,
            self.dino.y as i32,
            self.dino.w() as i32,
            self.dino.height() as i32,
            match self.dino.jumping {
                true => Color::Blue,
                false => Color::Green,
            },
        );
        if !self.dino.jumping {
            let foot_w = 10;
            let foot_h = 4;
            let step = 6.0;
            draw::draw_rect_fill(
                match self.dino.leg_state {
                    true => (self.dino.x() + step) as i32,
                    false => (self.dino.x() + self.dino.w() - step - foot_w as f32) as i32,
                },
                (self.dino.y + self.dino.height() - foot_h as f32) as i32,
                foot_w,
                foot_h,
                Color::Blue,
            );
        }
        for obstacle in &self.obstacles {
            draw::draw_rect_fill(
                obstacle.rect.0 as i32,
                obstacle.rect.1 as i32,
                obstacle.rect.2 as i32,
                obstacle.rect.3 as i32,
                match obstacle.kind {
                    ObstacleKind::Cactus(..) => Color::Selection,
                    ObstacleKind::Pterodactyl(..) => Color::Magenta,
                },
            );
        }
    }
}
fn rect_intersect(a: draw::Rect, b: draw::Rect) -> bool {
    a.x < (b.x + b.w) && (a.x + a.w) > b.x && a.y < (b.y + b.h) && (a.y + a.h) > b.y
}
