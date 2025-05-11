mod models;

use {
    ::image::GenericImageView,
    models::*,
    piston::{
        input::GenericEvent, Button, EventLoop, EventSettings, Key, PressEvent, ReleaseEvent,
        WindowSettings,
    },
    piston_window::{
        graphics::context::Context, Flip, G2d, G2dTexture, ImageSize, OpenGL, PistonWindow,
        Texture, TextureSettings, Transformed,
    },
};

pub struct Timer {
    pub interval: f64,
    pub time: f64,
    pub next: f64,
}

impl Timer {
    pub fn new(interval: f64) -> Timer {
        Timer {
            interval,
            time: 0.0,
            next: 0.0,
        }
    }

    pub fn event<E: GenericEvent, F: FnMut()>(&mut self, e: &E, mut f: F) {
        if let Some(args) = e.update_args() {
            self.time += args.dt;
            while self.next <= self.time {
                self.next += self.interval;
                f();
            }
        }
    }
}

fn main() {
    let mut game_timers: [Timer; 10] = [
        Timer::new(1.0),
        Timer::new(0.9),
        Timer::new(0.8),
        Timer::new(0.7),
        Timer::new(0.6),
        Timer::new(0.5),
        Timer::new(0.4),
        Timer::new(0.3),
        Timer::new(0.2),
        Timer::new(0.1),
    ];
    let (width, height) = ::image::open("assets/backg.png").unwrap().dimensions();
    let mut window: PistonWindow = WindowSettings::new("retrotris", [width, height])
        .graphics_api(OpenGL::V4_0)
        .exit_on_esc(true)
        .resizable(true)
        .build()
        .unwrap();
    let textures = [
        "assets/dblue-block.png",
        "assets/green-block.png",
        "assets/lblue-block.png",
        "assets/orange-block.png",
        "assets/purple-block.png",
        "assets/red-block.png",
        "assets/yellow-block.png",
        "assets/inactive-block.png",
        "assets/backg.png",
    ]
    .iter()
    .map(|path| {
        Texture::from_path(
            &mut window.create_texture_context(),
            path,
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap()
    })
    .collect::<Vec<G2dTexture>>();
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window.load_font(assets.join("FiraMono-Bold.ttf")).unwrap();
    let mut game_state = Model::default();
    game_state.create_quadshape();
    let mut events = piston_window::Events::new(EventSettings::new().ups(60));
    window.set_lazy(true);
    while let Some(event) = events.next(&mut window) {
        if !game_state.game_over {
            game_state.game_over = game_state.game_over_condition();
        }
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => game_state.rotate_quadshape(),
                Key::Left => game_state.move_quadshape_left(),
                Key::Right => game_state.move_quadshape_right(),
                Key::Down => {
                    game_state.down_held = true;
                }
                _ => {}
            }
        }
        if let Some(Button::Keyboard(key)) = event.release_args() {
            if key == Key::Down {
                game_state.down_held = false;
            }
        }
        if game_state.down_held {
            game_state.move_quadshape_down();
        }
        game_timers[game_state.level].event(&event, || {
            game_state.update_game_state();
        });
        window.draw_2d(&event, |c, g, device| {
            let mut transform = c.transform.trans(50.0, 200.0);
            piston_window::clear([0.0, 0.0, 0.0, 1.0], g);
            draw_game(&mut game_state, &textures, c, g);
            if game_state.game_over {
                piston_window::Text::new_color([1.0, 0.0, 0.0, 1.0], 22)
                    .draw(
                        "Game Over!",
                        &mut glyphs,
                        &c.draw_state,
                        c.transform.trans(580.0, 200.0),
                        g,
                    )
                    .unwrap();
            }
            piston_window::Text::new_color([0.0, 0.0, 0.0, 1.0], 22)
                .draw(
                    &format!("{}: {}", "Score", &game_state.score.to_string()),
                    &mut glyphs,
                    &c.draw_state,
                    transform,
                    g,
                )
                .unwrap();
            transform = c.transform.trans(1000.0, 300.0);
            piston_window::Text::new_color([0.0, 0.0, 0.0, 1.0], 22)
                .draw(
                    &format!("{}: {}", "Level ", &game_state.level.to_string()),
                    &mut glyphs,
                    &c.draw_state,
                    transform,
                    g,
                )
                .unwrap();
            transform = c.transform.trans(1000.0, 326.0);
            piston_window::Text::new_color([0.0, 0.0, 0.0, 1.0], 22)
                .draw(
                    &format!("{}: {}", "Rows ", &game_state.rows.to_string()),
                    &mut glyphs,
                    &c.draw_state,
                    transform,
                    g,
                )
                .unwrap();
            glyphs.factory.encoder.flush(device);
        });
    }
}

fn draw_game(game_state: &mut Model, textures: &[G2dTexture], c: Context, g: &mut G2d) {
    let bg_image = textures.last().unwrap();
    let grid_width_px = GRID_WIDTH as f64 * CELL_SIZE;
    let grid_height_px = GRID_HEIGHT as f64 * CELL_SIZE;
    let horizontal_offset = (bg_image.get_width() as f64 - grid_width_px) / 2.0;
    let vertical_offset = (bg_image.get_height() as f64 - grid_height_px) / 2.0;
    piston_window::image(bg_image, c.transform, g);
    draw_locked_blocks(
        game_state,
        textures,
        c.trans(horizontal_offset, vertical_offset),
        g,
    );
}

fn draw_locked_blocks(game_state: &mut Model, textures: &[G2dTexture], c: Context, g: &mut G2d) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if let Some(texture) = &game_state.grid[y][x] {
                piston_window::image(
                    &textures[*texture],
                    c.transform
                        .trans((x as f64) * CELL_SIZE, (y as f64) * CELL_SIZE),
                    g,
                );
            }
        }
    }
    if game_state.active_quadshape.is_none() {
        game_state.active_quadshape = game_state.upcoming_quadshape.clone();
        game_state.create_quadshape();
    }
    if let Some(ref quadshape) = game_state.active_quadshape {
        let quadshape_x = quadshape.x as f64;
        let quadshape_y = quadshape.y as f64;

        for &(dx, dy) in QUAD_SHAPES[quadshape.shape][quadshape.rotation] {
            let block_x = (quadshape_x + dx as f64) * CELL_SIZE;
            let block_y = (quadshape_y + dy as f64) * CELL_SIZE;
            piston_window::image(
                &textures[quadshape.active_block_texture],
                c.transform.trans(block_x, block_y),
                g,
            );
        }
    }
    if let Some(ref quadshape) = game_state.upcoming_quadshape {
        let quadshape_x = 20.0;
        let quadshape_y = 0.0;
        for &(dx, dy) in QUAD_SHAPES[quadshape.shape][quadshape.rotation] {
            let block_x = (quadshape_x + dx as f64) * CELL_SIZE;
            let block_y = (quadshape_y + dy as f64) * CELL_SIZE;
            piston_window::image(
                &textures[quadshape.active_block_texture],
                c.transform.trans(block_x, block_y),
                g,
            );
        }
    }
}
