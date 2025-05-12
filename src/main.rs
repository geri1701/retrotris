mod models;

use {
    ::image::GenericImageView,
    models::*,
    piston::{
        Button, EventLoop, EventSettings, Key, PressEvent, ReleaseEvent,
        WindowSettings, UpdateEvent,
    },
    piston_window::{
        graphics::context::Context, Flip, G2d, G2dTexture, ImageSize, PistonWindow,
        Texture, TextureSettings, Transformed,
    },
};

const FONT_SIZE: u32 = 22;

impl Model {
    fn draw_game(&mut self, textures: &[G2dTexture], c: Context, g: &mut G2d) {
        let bg_image = textures.last().unwrap();
        let grid_width_px = GRID_WIDTH as f64 * CELL_SIZE;
        let grid_height_px = GRID_HEIGHT as f64 * CELL_SIZE;
        let horizontal_offset = (bg_image.get_width() as f64 - grid_width_px) / 2.0;
        let vertical_offset = (bg_image.get_height() as f64 - grid_height_px) / 2.0;
        piston_window::image(bg_image, c.transform, g);
        self.draw_locked_blocks(
            textures,
            c.trans(horizontal_offset, vertical_offset),
            g,
        );
    }
    fn draw_locked_blocks(&mut self, textures: &[G2dTexture], c: Context, g: &mut G2d) {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if let Some(texture) = &self.grid[y][x] {
                    piston_window::image(
                        &textures[*texture],
                        c.transform
                            .trans((x as f64) * CELL_SIZE, (y as f64) * CELL_SIZE),
                        g,
                    );
                }
            }
        }
        if self.active_quadshape.is_none() {
            self.active_quadshape = self.upcoming_quadshape.clone();
            self.create_quadshape();
        }
        if let Some(ref quadshape) = self.active_quadshape {
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
        if let Some(ref quadshape) = self.upcoming_quadshape {
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
    fn handle(&mut self, event: &piston_window::Event) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => self.rotate_quadshape(),
                Key::Left => self.move_quadshape_left(),
                Key::Right => self.move_quadshape_right(),
                Key::Down => self.down_held = true,
                _ => {}
            }
        }
        if let Some(Button::Keyboard(key)) = event.release_args() {
            if key == Key::Down {
                self.down_held = false;
            }
        }
    }
    fn run(&mut self, width: u32, height: u32) {
        let mut window: PistonWindow = WindowSettings::new("retrotris", [width, height])
            .graphics_api(piston_window::OpenGL::V4_0)
            .exit_on_esc(true)
            .resizable(true)
            .build()
            .unwrap();
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
        self.create_quadshape();
        let mut events = piston_window::Events::new(EventSettings::new().ups(60));
        window.set_lazy(true);
        while let Some(event) = events.next(&mut window) {
            if !self.game_over {
                self.game_over = self.game_over_condition();
            }
            if self.down_held {
                self.move_quadshape_down();
            }
            self.handle(&event);
            if let Some(args) = event.update_args() {
                game_timers[self.level].event(args.dt, || self.update_game_state());
            }
            window.draw_2d(&event, |context, g, device| {
                piston_window::clear(piston_window::color::BLACK, g);
                self.draw_game(&textures, context, g);
                if self.game_over {
                    piston_window::Text::new_color(piston_window::color::RED, FONT_SIZE)
                        .draw(
                            "Game Over!",
                            &mut glyphs,
                            &context.draw_state,
                            context.transform.trans(580.0, 200.0),
                            g,
                        )
                        .unwrap();
                }
                piston_window::Text::new_color(piston_window::color::OLIVE, FONT_SIZE)
                    .draw(
                        &format!("Score: {}", &self.score),
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(1000.0, 274.0),
                        g,
                    )
                    .unwrap();
                piston_window::Text::new_color(piston_window::color::OLIVE, FONT_SIZE)
                    .draw(
                        &format!("Level: {}", &self.level),
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(1000.0, 300.0),
                        g,
                    )
                    .unwrap();
                piston_window::Text::new_color(piston_window::color::OLIVE, FONT_SIZE)
                    .draw(
                        &format!("Rows:  {}", &self.rows),
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(1000.0, 326.0),
                        g,
                    )
                    .unwrap();
                glyphs.factory.encoder.flush(device);
            });
        }
    }
}

fn main() {
    let (width, height) = ::image::open("assets/backg.png").unwrap().dimensions();
    Model::default().run(width, height);
}
