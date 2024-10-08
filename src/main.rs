use {
    ::image::GenericImageView,
    gfx_device_gl::Resources,
    piston::input::GenericEvent,
    piston_window::graphics::context::Context,
    piston_window::*,
    rand::prelude::*,
    std::{ops::Deref, rc::Rc},
};

const CELL_SIZE: f64 = 24.0;
const GRID_WIDTH: usize = 12;
const GRID_HEIGHT: usize = 24;

type TextureVecIndex = usize;
type QuadshapeShape = ([&'static [(i32, i32)]; 4], TextureVecIndex);

const QUAD_SHAPES: [QuadshapeShape; 7] = [
    // Square (No rotation)
    (
        [
            &[(0, 0), (1, 0), (0, 1), (1, 1)],
            &[(0, 0), (1, 0), (0, 1), (1, 1)],
            &[(0, 0), (1, 0), (0, 1), (1, 1)],
            &[(0, 0), (1, 0), (0, 1), (1, 1)],
        ],
        0,
    ),
    // Straight
    (
        [
            &[(0, 0), (1, 0), (2, 0), (3, 0)],
            &[(0, 0), (0, 1), (0, 2), (0, 3)],
            &[(0, 0), (1, 0), (2, 0), (3, 0)],
            &[(0, 0), (0, 1), (0, 2), (0, 3)],
        ],
        1,
    ),
    // T-Shaped Quadshape
    (
        [
            &[(0, 0), (1, 0), (2, 0), (1, 1)], // Rotation 0
            &[(1, 0), (0, 1), (1, 1), (1, 2)], // Rotation 1
            &[(0, 1), (1, 0), (1, 1), (2, 1)], // Rotation 2
            &[(1, 0), (1, 1), (1, 2), (2, 1)], // Rotation 3
        ],
        2,
    ),
    // L
    (
        [
            &[(0, 0), (1, 0), (2, 0), (2, 1)],
            &[(1, 0), (1, 1), (1, 2), (0, 2)],
            &[(0, 0), (0, 1), (1, 1), (2, 1)],
            &[(1, 0), (2, 0), (1, 1), (1, 2)],
        ],
        3,
    ),
    // J
    (
        [
            &[(0, 0), (1, 0), (2, 0), (0, 1)],
            &[(1, 0), (1, 1), (1, 2), (2, 2)],
            &[(0, 1), (1, 1), (2, 1), (2, 0)],
            &[(1, 0), (1, 1), (1, 2), (0, 0)],
        ],
        4,
    ),
    // S
    (
        [
            &[(1, 0), (2, 0), (0, 1), (1, 1)],
            &[(0, 0), (0, 1), (1, 1), (1, 2)],
            &[(1, 0), (2, 0), (0, 1), (1, 1)],
            &[(0, 0), (0, 1), (1, 1), (1, 2)],
        ],
        5,
    ),
    // Z
    (
        [
            &[(0, 0), (1, 0), (1, 1), (2, 1)],
            &[(1, 0), (0, 1), (1, 1), (0, 2)],
            &[(0, 0), (1, 0), (1, 1), (2, 1)],
            &[(1, 0), (0, 1), (1, 1), (0, 2)],
        ],
        6,
    ),
];
#[derive(Clone)]
struct Quadshape {
    x: i32,
    y: i32,
    shape: usize,
    rotation: usize,
    active_block_texture: Rc<G2dTexture>,
}

#[derive(Clone, Copy, Default, PartialEq)]
enum CellState {
    #[default]
    Free,
    Locked,
}

#[derive(Clone, Default)]
struct GridCell {
    state: CellState,
    texture: Option<G2dTexture>,
}
#[derive(Clone, Default)]
struct Grid {
    data: [[GridCell; GRID_WIDTH]; GRID_HEIGHT],
}

struct GameState {
    active_quadshape: Option<Quadshape>,
    upcoming_quadshape: Option<Quadshape>,
    grid: Grid,
    score: usize,
    level: usize,
    rows: usize,
    game_over: bool,
    down_held: bool,
}

impl GameState {
    fn lock_quadshape(&mut self, textures: &[Rc<G2dTexture>]) {
        if let Some(ref mut quadshape) = self.active_quadshape {
            for &(block_x, block_y) in QUAD_SHAPES[quadshape.shape].0[quadshape.rotation].iter() {
                let abs_x = quadshape.x + block_x;
                let abs_y = quadshape.y + block_y;
                self.grid.data[abs_y as usize][abs_x as usize].state = CellState::Locked;
                self.grid.data[abs_y as usize][abs_x as usize].texture =
                    Some(textures[7].deref().clone());
            }
        }
        self.active_quadshape = None;
    }

    fn clear_completed_rows(&mut self) {
        let mut rows_to_clear: Vec<usize> = Vec::new();
        for y in 0..GRID_HEIGHT {
            let mut rows_completed = true;
            for x in 0..GRID_WIDTH {
                if self.grid.data[y][x].state != CellState::Locked {
                    rows_completed = false;
                    break;
                }
            }
            if rows_completed {
                rows_to_clear.push(y);
            }
        }
        if !rows_to_clear.is_empty() {
            for &row in &rows_to_clear {
                self.rows += 1;
                for y in (1..=row).rev() {
                    for x in 0..GRID_WIDTH {
                        self.grid.data[y][x] = self.grid.data[y - 1][x].clone();
                    }
                }
                for x in 0..GRID_WIDTH {
                    self.grid.data[0][x] = GridCell::default();
                }
            }
            let num_lines_cleared = rows_to_clear.len();
            self.score += 100_usize * num_lines_cleared * num_lines_cleared;
            self.level = self.score / 1000 + 1;
        }
    }

    fn update_game_state(&mut self, textures: Vec<Rc<Texture<Resources>>>) {
        if self.down_held {
            self.move_quadshape_down();
        } else if self.active_quadshape.is_none() {
            self.create_quadshape(&textures);
        } else if !self.quadshape_should_lock() {
            self.move_quadshape_down();
        } else {
            self.lock_quadshape(&textures);
            self.clear_completed_rows();
        }
    }

    fn quadshape_should_lock(&self) -> bool {
        if let Some(ref quadshape) = self.active_quadshape {
            let new_y = quadshape.y + 1;
            if new_y >= GRID_HEIGHT as i32 {
                return true;
            }
            if quadshape_collides(quadshape, quadshape.x, new_y, &self.grid) {
                return true;
            }
        }
        false
    }

    fn move_quadshape_left(&mut self) {
        if let Some(ref mut quadshape) = self.active_quadshape {
            let new_x = quadshape.x - 1;
            if new_x >= -1
                || QUAD_SHAPES[quadshape.shape].0[quadshape.rotation] != [(1, 0), (2, 0), (2, 1)]
            {
                if new_x >= -1 && !quadshape_collides(quadshape, new_x, quadshape.y, &self.grid) {
                    quadshape.x = new_x;
                }
            }
        }
    }

    fn move_quadshape_right(&mut self) {
        if let Some(ref mut quadshape) = self.active_quadshape {
            let new_x = quadshape.x + 1;
            if new_x < GRID_WIDTH as i32
                && !quadshape_collides(quadshape, new_x, quadshape.y, &self.grid)
            {
                quadshape.x = new_x;
            }
        }
    }

    fn rotate_quadshape(&mut self) {
        if let Some(ref mut quadshape) = self.active_quadshape {
            if quadshape_can_rotate(quadshape, &self.grid) {
                let new_rotation = (quadshape.rotation + 1) % 4;
                quadshape.rotation = new_rotation;
            }
        }
    }

    fn move_quadshape_down(&mut self) {
        if let Some(ref mut quadshape) = self.active_quadshape {
            let new_y = quadshape.y + 1;
            if new_y < GRID_HEIGHT as i32
                && !quadshape_collides(quadshape, quadshape.x, new_y, &self.grid)
            {
                quadshape.y = new_y;
            }
        }
    }

    fn game_over_condition(&self) -> bool {
        for &(x, y) in QUAD_SHAPES[0].0[0].iter() {
            let abs_x = (GRID_WIDTH as i32 / 2) - 1 + x;
            let abs_y = y;
            if self.grid.data[abs_y as usize][abs_x as usize].state == CellState::Locked {
                return true;
            }
        }
        false
    }

    fn create_quadshape(&mut self, textures: &[Rc<G2dTexture>]) {
        let mut rng = rand::thread_rng();
        let shape_index = random_quadshape_index(&mut rng);
        let shape = QUAD_SHAPES[shape_index];
        let rotation = 0;
        let active_block_texture = Rc::clone(&textures[shape.1]);
        // initial position
        let x = (GRID_WIDTH as i32 / 2) - 1;
        let y = 0;
        let temp_quadshape = Quadshape {
            x,
            y,
            shape: shape_index,
            rotation,
            active_block_texture: Rc::clone(&active_block_texture),
        };
        if !quadshape_collides(&temp_quadshape, x, y, &self.grid) {
            // If there is no collision, add the new quadshape to the game state
            let new_quadshape = Quadshape {
                x,
                y,
                shape: shape_index,
                rotation,
                active_block_texture,
            };
            self.upcoming_quadshape = Some(new_quadshape);
        }
    }
}

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
    let textures = load_textures(&mut window);
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window.load_font(assets.join("FiraMono-Bold.ttf")).unwrap();
    let mut game_state = GameState {
        active_quadshape: None,
        upcoming_quadshape: None,
        grid: Grid::default(),
        score: 0,
        level: 1,
        rows: 0,
        game_over: false,
        down_held: false,
    };
    game_state.create_quadshape(&textures);
    let mut events = Events::new(EventSettings::new().ups(60));
    window.set_lazy(true);
    while let Some(e) = events.next(&mut window) {
        if !game_state.game_over {
            game_state.game_over = game_state.game_over_condition();
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
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
        if let Some(Button::Keyboard(key)) = e.release_args() {
            if key == Key::Down {
                game_state.down_held = false;
            }
        }
        if game_state.down_held {
            game_state.move_quadshape_down();
        }
        game_timers[game_state.level - 1_usize].event(&e, || {
            game_state.update_game_state(textures.clone());
        });
        window.draw_2d(&e, |c, g, device| {
            let mut transform = c.transform.trans(50.0, 200.0);
            clear([0.0, 0.0, 0.0, 1.0], g);
            draw_game(&mut game_state, &textures, c, g, &mut glyphs);
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 22)
                .draw(
                    &format!("{}: {}", "Score", &game_state.score.to_string()),
                    &mut glyphs,
                    &c.draw_state,
                    transform,
                    g,
                )
                .unwrap();
            transform = c.transform.trans(1000.0, 300.0);
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 22)
                .draw(
                    &format!("{}: {}", "Level ", &game_state.level.to_string()),
                    &mut glyphs,
                    &c.draw_state,
                    transform,
                    g,
                )
                .unwrap();
            transform = c.transform.trans(1000.0, 326.0);
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 22)
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

fn random_quadshape_index(rng: &mut ThreadRng) -> usize {
    rng.gen_range(0..QUAD_SHAPES.len())
}

fn load_textures(window: &mut PistonWindow) -> Vec<Rc<G2dTexture>> {
    let mut textures = Vec::new();
    for path in [
        "assets/dblue-block.png",
        "assets/green-block.png",
        "assets/lblue-block.png",
        "assets/orange-block.png",
        "assets/purple-block.png",
        "assets/red-block.png",
        "assets/yellow-block.png",
        "assets/inactive-block.png",
        "assets/backg.png",
    ] {
        let texture = Texture::from_path(
            &mut window.create_texture_context(),
            path,
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap();
        textures.push(Rc::new(texture).clone());
    }
    textures
}

fn quadshape_can_rotate(quadshape: &Quadshape, grid: &Grid) -> bool {
    let new_rotation = (quadshape.rotation + 1) % 4;
    let temp_quadshape = Quadshape {
        x: quadshape.x,
        y: quadshape.y,
        shape: quadshape.shape,
        rotation: new_rotation,
        active_block_texture: Rc::clone(&quadshape.active_block_texture),
    };
    for &(block_x, block_y) in QUAD_SHAPES[temp_quadshape.shape].0[temp_quadshape.rotation].iter() {
        let new_x = temp_quadshape.x + block_x;
        let new_y = temp_quadshape.y + block_y;
        if new_x < 0
            || new_x >= GRID_WIDTH as i32
            || new_y >= GRID_HEIGHT as i32
            || (new_y >= 0 && grid.data[new_y as usize][new_x as usize].state == CellState::Locked)
        {
            return false;
        }
    }
    true
}

fn quadshape_collides(quadshape: &Quadshape, x: i32, y: i32, grid: &Grid) -> bool {
    for &(block_x, block_y) in QUAD_SHAPES[quadshape.shape].0[quadshape.rotation].iter() {
        let new_x = x + block_x;
        let new_y = y + block_y;
        if new_x < 0 || new_x >= GRID_WIDTH as i32 || new_y >= GRID_HEIGHT as i32 {
            return true;
        }
        if new_y >= 0 && grid.data[new_y as usize][new_x as usize].state == CellState::Locked {
            return true;
        }
    }
    false
}

fn draw_game(
    game_state: &mut GameState,
    textures: &[Rc<G2dTexture>],
    c: Context,
    g: &mut G2d,
    glyphs: &mut Glyphs,
) {
    let bg_image = textures.last().unwrap();
    let grid_width_px = GRID_WIDTH as f64 * CELL_SIZE;
    let grid_height_px = GRID_HEIGHT as f64 * CELL_SIZE;
    let horizontal_offset = (bg_image.get_width() as f64 - grid_width_px) / 2.0;
    let vertical_offset = (bg_image.get_height() as f64 - grid_height_px) / 2.0;
    image(bg_image.deref(), c.transform, g);
    draw_locked_blocks(
        game_state,
        textures,
        c.trans(horizontal_offset, vertical_offset),
        g,
    );
    draw_game_over(game_state, c, g, glyphs);
}

fn draw_locked_blocks(
    game_state: &mut GameState,
    textures: &[Rc<G2dTexture>],
    c: Context,
    g: &mut G2d,
) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if let Some(texture) = &game_state.grid.data[y][x].texture {
                let rect_x = (x as f64) * CELL_SIZE;
                let rect_y = (y as f64) * CELL_SIZE;
                image(texture, c.transform.trans(rect_x, rect_y), g);
            }
        }
    }
    if game_state.active_quadshape.is_none() {
        game_state.active_quadshape = game_state.upcoming_quadshape.clone();
        game_state.create_quadshape(textures);
    }
    if let Some(ref quadshape) = game_state.active_quadshape {
        let quadshape_x = quadshape.x as f64;
        let quadshape_y = quadshape.y as f64;
        let quadshape_shape = &QUAD_SHAPES[quadshape.shape].0[quadshape.rotation];

        for &(dx, dy) in quadshape_shape.iter() {
            let block_x = (quadshape_x + dx as f64) * CELL_SIZE;
            let block_y = (quadshape_y + dy as f64) * CELL_SIZE;
            image(
                &*quadshape.active_block_texture,
                c.transform.trans(block_x, block_y),
                g,
            );
        }
    }
    if let Some(ref quadshape) = game_state.upcoming_quadshape {
        let quadshape_x = 20.0;
        let quadshape_y = 0.0;
        let quadshape_shape = &QUAD_SHAPES[quadshape.shape].0[quadshape.rotation];
        for &(dx, dy) in quadshape_shape.iter() {
            let block_x = (quadshape_x + dx as f64) * CELL_SIZE;
            let block_y = (quadshape_y + dy as f64) * CELL_SIZE;
            image(
                &*quadshape.active_block_texture,
                c.transform.trans(block_x, block_y),
                g,
            );
        }
    }
}

fn draw_game_over(game_state: &GameState, context: Context, g2d: &mut G2d, glyphs: &mut Glyphs) {
    if game_state.game_over {
        text::Text::new_color([1.0, 0.0, 0.0, 1.0], 22)
            .draw(
                "Game Over!",
                glyphs,
                &context.draw_state,
                context.transform.trans(580.0, 200.0),
                g2d,
            )
            .unwrap();
    }
}
