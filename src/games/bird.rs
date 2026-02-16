use crate::gui::*;
use rand::Rng;

const DOWN: i32 = 1;
const UP: i32 = -8;
const BIRD: i32 = 60;
const PIPE_WIDTH: i32 = BIRD * 2;
const PIPE_GAP: i32 = BIRD * 5;
const PIPE_SPEED: i32 = BIRD / 15;

#[derive(Default)]
struct Bird {
    position: (i32, i32),
    velocity: i32,
}

impl Bird {
    fn new(height: i32) -> Self {
        Self {
            position: (100, height / 2),
            velocity: 0,
        }
    }
}

struct Pipe {
    x: i32,
    gap_center_y: i32,
    scored: bool,
}

impl Pipe {
    fn top_rect(&self) -> (i32, i32, i32, i32) {
        let top_height = self.gap_center_y - (PIPE_GAP / 2);
        (self.x, 0, PIPE_WIDTH, top_height)
    }

    fn bottom_rect(&self, height: i32) -> (i32, i32, i32, i32) {
        let bottom_y = self.gap_center_y + (PIPE_GAP / 2);
        let bottom_height = height - bottom_y;
        (self.x, bottom_y, PIPE_WIDTH, bottom_height)
    }
}

#[derive(Default)]
pub struct Model {
    field: (i32, i32),
    bird: Bird,
    pipes: Vec<Pipe>,
    score: i32,
    play: bool,
    timer: f32,
}

impl Model {
    fn reset(&mut self) {
        self.bird = Bird::new(self.field.1);
        self.pipes = Vec::new();
        self.score = 0;
    }
}

impl Console for Model {
    fn load(&mut self, _path: &str) {}
    fn exit(&self, _path: &str) {}
    fn handle(&mut self, window: &mut Window, event: Event) -> bool {
        match event {
            Event::Focus => true,
            Event::Resize => {
                self.play = false;
                self.field = (window.w(), window.h());
                true
            }
            Event::KeyDown => {
                match event_key() {
                    Key::Escape => match self.play {
                        true => self.play = false,
                        false => crate::Model::connect(window),
                    },
                    Key::Enter => {
                        if !self.play {
                            self.reset();
                            self.play = true;
                        }
                    }
                    Key::Up => self.bird.velocity = UP,
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
        self.bird.velocity += DOWN;
        self.bird.position.1 += self.bird.velocity;

        for pipe in &mut self.pipes {
            pipe.x -= PIPE_SPEED;
        }

        self.timer += dt;
        if self.timer > 1.5 {
            self.pipes.push(Pipe {
                x: self.field.0,
                gap_center_y: rand::rng()
                    .random_range((PIPE_GAP / 2)..(self.field.1 - (PIPE_GAP / 2))),
                scored: false,
            });
            self.timer = 0.0;
        }

        self.pipes.retain(|pipe| pipe.x > -PIPE_WIDTH);

        for pipe in &mut self.pipes {
            if !pipe.scored && self.bird.position.0 > pipe.x + PIPE_WIDTH {
                self.score += 1;
                pipe.scored = true;
            }
        }

        if self.bird.position.1 - BIRD < 0 || self.bird.position.1 + BIRD > self.field.1 {
            self.play = false;
        }

        for pipe in &self.pipes {
            let bird_collides =
                check_collision_circle_rec_manual(self.bird.position, BIRD, pipe.top_rect())
                    || check_collision_circle_rec_manual(
                        self.bird.position,
                        BIRD,
                        pipe.bottom_rect(self.field.1),
                    );

            if bird_collides {
                self.play = false;
                break;
            }
        }
    }
    fn draw(&self, window: &mut Window) {
        if self.play {
            draw::draw_rect_fill(0, 0, window.width(), window.height(), Color::Background);
            for pipe in &self.pipes {
                let top_rect = pipe.top_rect();
                let bottom_rect = pipe.bottom_rect(self.field.1);
                draw::draw_rect_fill(top_rect.0, top_rect.1, top_rect.2, top_rect.3, Color::Green);
                draw::draw_rect_fill(
                    bottom_rect.0,
                    bottom_rect.1,
                    bottom_rect.2,
                    bottom_rect.3,
                    Color::Green,
                );
            }

            draw::draw_circle_fill(
                self.bird.position.0,
                self.bird.position.1,
                BIRD,
                Color::Selection,
            );

            draw::set_draw_color(Color::Background2);
            draw::draw_text2(
                &self.score.to_string(),
                self.bird.position.0,
                self.bird.position.1,
                BIRD,
                BIRD,
                Align::Center,
            );
        } else {
            window.draw_welcome(
                "Bird",
                &[
                    &["PRESS ENTER", "for play"],
                    &["PRESS ESC", "for exit"],
                    &["PRESS UP", "for fly"],
                ],
            );
        }
    }
}

fn check_collision_circle_rec_manual(
    position: (i32, i32),
    size: i32,
    rec: (i32, i32, i32, i32),
) -> bool {
    // Find the closest point on the rectangle to the center of the circle
    let radius = size / 2;
    let center = (position.0 + radius, position.1 + radius);
    let closest_x = center.0.max(rec.0).min(rec.0 + rec.2);
    let closest_y = center.1.max(rec.1).min(rec.1 + rec.3);

    // Calculate the distance between the closest point and the circle's center
    let distance_x = center.0 - closest_x;
    let distance_y = center.1 - closest_y;
    let distance_squared = (distance_x * distance_x) + (distance_y * distance_y);

    // If the distance squared is less than the circle's radius squared, there is a collision
    distance_squared < (radius * radius)
}
