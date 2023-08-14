use ::image::GenericImageView;
use gfx_device_gl::Resources;
use graphics::context::Context;
use piston::input::GenericEvent;
use piston_window::Transformed;
use piston_window::*;
use rand::prelude::*;
use std::ops::Deref;
use std::rc::Rc;

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
    let event_settings = EventSettings::new().ups(60);
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
    let bg_img = ::image::open("img/backg.png").unwrap();
    let (width, height) = bg_img.dimensions();
    let mut window: PistonWindow = WindowSettings::new("retrotris", [width, height])
        .graphics_api(OpenGL::V4_0)
        .exit_on_esc(true)
        .resizable(true)
        .build()
        .unwrap();
    let textures = load_textures(&mut window);
    let textures_ref: &Vec<Rc<G2dTexture>> = &textures; // Reference to the vector
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("img")
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
    game_state.create_quadshape(textures_ref);
    let mut events = Events::new(event_settings);
    window.set_lazy(true);
    while let Some(e) = events.next(&mut window) {
        if !game_state.game_over {
            game_state.game_over = game_over_condition(&game_state);
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up => rotate_quadshape(&mut game_state),
                Key::Left => move_quadshape_left(&mut game_state),
                Key::Right => move_quadshape_right(&mut game_state),
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
            move_quadshape_down(&mut game_state);
        }
        game_timers[game_state.level - 1_usize].event(&e, || {
            update_game_state(&mut game_state, textures.clone());
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
    let texture_paths = [
        "img/dblue-block.png",
        "img/green-block.png",
        "img/lblue-block.png",
        "img/orange-block.png",
        "img/purple-block.png",
        "img/red-block.png",
        "img/yellow-block.png",
        "img/inactive-block.png",
        "img/backg.png",
    ];

    let mut textures = Vec::new();

    for path in &texture_paths {
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

fn move_quadshape_left(game_state: &mut GameState) {
    if let Some(ref mut quadshape) = game_state.active_quadshape {
        let new_x = quadshape.x - 1;
        if new_x >= -1
            || QUAD_SHAPES[quadshape.shape].0[quadshape.rotation] != [(1, 0), (2, 0), (2, 1)]
        {
            if new_x >= -1 {
                if !quadshape_collides(quadshape, new_x, quadshape.y, &game_state.grid) {
                    quadshape.x = new_x;
                }
            }
        }
    }
}

fn move_quadshape_right(game_state: &mut GameState) {
    if let Some(ref mut quadshape) = game_state.active_quadshape {
        let new_x = quadshape.x + 1;
        if new_x < GRID_WIDTH as i32 {
            if !quadshape_collides(quadshape, new_x, quadshape.y, &game_state.grid) {
                quadshape.x = new_x;
            }
        }
    }
}

fn move_quadshape_down(game_state: &mut GameState) {
    if let Some(ref mut quadshape) = game_state.active_quadshape {
        let new_y = quadshape.y + 1;
        if new_y < GRID_HEIGHT as i32 {
            if !quadshape_collides(quadshape, quadshape.x, new_y, &game_state.grid) {
                quadshape.y = new_y;
            }
        }
    }
}

fn rotate_quadshape(game_state: &mut GameState) {
    if let Some(ref mut quadshape) = game_state.active_quadshape {
        if quadshape_can_rotate(quadshape, &game_state.grid) {
            let new_rotation = (quadshape.rotation + 1) % 4;
            quadshape.rotation = new_rotation;
        }
    }
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

fn update_game_state(game_state: &mut GameState, textures: Vec<Rc<Texture<Resources>>>) {
    if game_state.down_held {
        move_quadshape_down(game_state);
    } else {
        if game_state.active_quadshape.is_none() {
            game_state.create_quadshape(&textures);
        } else {
            if !quadshape_should_lock(game_state) {
                move_quadshape_down(game_state);
            } else {
                lock_quadshape(&textures, game_state);
                clear_completed_rows(game_state);
            }
        }
    }
}

fn quadshape_should_lock(game_state: &GameState) -> bool {
    if let Some(ref quadshape) = game_state.active_quadshape {
        let new_y = quadshape.y + 1;
        if new_y >= GRID_HEIGHT as i32 {
            return true;
        }
        if quadshape_collides(quadshape, quadshape.x, new_y, &game_state.grid) {
            return true;
        }
    }
    false
}

fn lock_quadshape(textures: &[Rc<G2dTexture>], game_state: &mut GameState) {
    if let Some(ref mut quadshape) = game_state.active_quadshape {
        for &(block_x, block_y) in QUAD_SHAPES[quadshape.shape].0[quadshape.rotation].iter() {
            let abs_x = quadshape.x + block_x;
            let abs_y = quadshape.y + block_y;
            game_state.grid.data[abs_y as usize][abs_x as usize].state = CellState::Locked;
            game_state.grid.data[abs_y as usize][abs_x as usize].texture =
                Some(textures[7].deref().clone());
        }
    }
    game_state.active_quadshape = None;
}

fn clear_completed_rows(game_state: &mut GameState) {
    let mut rows_to_clear: Vec<usize> = Vec::new();
    for y in 0..GRID_HEIGHT {
        let mut rows_completed = true;
        for x in 0..GRID_WIDTH {
            if game_state.grid.data[y][x].state != CellState::Locked {
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
            game_state.rows += 1;
            for y in (1..=row).rev() {
                for x in 0..GRID_WIDTH {
                    game_state.grid.data[y][x] = game_state.grid.data[y - 1][x].clone();
                }
            }
            for x in 0..GRID_WIDTH {
                game_state.grid.data[0][x] = GridCell::default();
            }
        }
        let num_lines_cleared = rows_to_clear.len();
        game_state.score += 100_usize * num_lines_cleared * num_lines_cleared;
        game_state.level = game_state.score / 1000 + 1;
    }
}

fn game_over_condition(game_state: &GameState) -> bool {
    for &(x, y) in QUAD_SHAPES[0].0[0].iter() {
        let abs_x = (GRID_WIDTH as i32 / 2) - 1 + x;
        let abs_y = y;
        if game_state.grid.data[abs_y as usize][abs_x as usize].state == CellState::Locked {
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
    let bg_width = bg_image.get_width() as f64;
    let bg_height = bg_image.get_height() as f64;
    let grid_width_px = GRID_WIDTH as f64 * CELL_SIZE;
    let grid_height_px = GRID_HEIGHT as f64 * CELL_SIZE;
    let horizontal_offset = (bg_width - grid_width_px) / 2.0;
    let vertical_offset = (bg_height - grid_height_px) / 2.0;
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

fn draw_game_over(game_state: &GameState, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
    if game_state.game_over {
        let message = "Game Over!";
        let center_x = 580.0;
        let center_y = 200.0;
        text::Text::new_color([1.0, 0.0, 0.0, 1.0], 22)
            .draw(
                message,
                glyphs,
                &c.draw_state,
                c.transform.trans(center_x, center_y),
                g,
            )
            .unwrap();
    }
}
