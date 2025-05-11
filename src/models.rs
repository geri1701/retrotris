use rand::Rng;

pub const CELL_SIZE: f64 = 24.0;
pub const GRID_WIDTH: usize = 12;
pub const GRID_HEIGHT: usize = 24;
pub const QUAD_SHAPES: [[&[(i32, i32)]; 4]; 7] = [
    // Square (No rotation)
    [
        &[(0, 0), (1, 0), (0, 1), (1, 1)],
        &[(0, 0), (1, 0), (0, 1), (1, 1)],
        &[(0, 0), (1, 0), (0, 1), (1, 1)],
        &[(0, 0), (1, 0), (0, 1), (1, 1)],
    ],
    // Straight
    [
        &[(0, 0), (1, 0), (2, 0), (3, 0)],
        &[(0, 0), (0, 1), (0, 2), (0, 3)],
        &[(0, 0), (1, 0), (2, 0), (3, 0)],
        &[(0, 0), (0, 1), (0, 2), (0, 3)],
    ],
    // T-Shaped Quadshape
    [
        &[(0, 0), (1, 0), (2, 0), (1, 1)], // Rotation 0
        &[(1, 0), (0, 1), (1, 1), (1, 2)], // Rotation 1
        &[(0, 1), (1, 0), (1, 1), (2, 1)], // Rotation 2
        &[(1, 0), (1, 1), (1, 2), (2, 1)], // Rotation 3
    ],
    // L
    [
        &[(0, 0), (1, 0), (2, 0), (2, 1)],
        &[(1, 0), (1, 1), (1, 2), (0, 2)],
        &[(0, 0), (0, 1), (1, 1), (2, 1)],
        &[(1, 0), (2, 0), (1, 1), (1, 2)],
    ],
    // J
    [
        &[(0, 0), (1, 0), (2, 0), (0, 1)],
        &[(1, 0), (1, 1), (1, 2), (2, 2)],
        &[(0, 1), (1, 1), (2, 1), (2, 0)],
        &[(1, 0), (1, 1), (1, 2), (0, 0)],
    ],
    // S
    [
        &[(1, 0), (2, 0), (0, 1), (1, 1)],
        &[(0, 0), (0, 1), (1, 1), (1, 2)],
        &[(1, 0), (2, 0), (0, 1), (1, 1)],
        &[(0, 0), (0, 1), (1, 1), (1, 2)],
    ],
    // Z
    [
        &[(0, 0), (1, 0), (1, 1), (2, 1)],
        &[(1, 0), (0, 1), (1, 1), (0, 2)],
        &[(0, 0), (1, 0), (1, 1), (2, 1)],
        &[(1, 0), (0, 1), (1, 1), (0, 2)],
    ],
];

#[derive(Clone)]
pub struct Quadshape {
    pub x: i32,
    pub y: i32,
    pub shape: usize,
    pub rotation: usize,
    pub active_block_texture: usize,
}

#[derive(Default)]
pub struct Model {
    pub active_quadshape: Option<Quadshape>,
    pub upcoming_quadshape: Option<Quadshape>,
    pub grid: [[Option<usize>; GRID_WIDTH]; GRID_HEIGHT],
    pub score: usize,
    pub level: usize,
    pub rows: usize,
    pub game_over: bool,
    pub down_held: bool,
}

impl Model {
    fn lock_quadshape(&mut self) {
        if let Some(ref mut quadshape) = self.active_quadshape {
            for &(block_x, block_y) in QUAD_SHAPES[quadshape.shape][quadshape.rotation] {
                let abs_x = quadshape.x + block_x;
                let abs_y = quadshape.y + block_y;
                //self.grid.data[abs_y as usize][abs_x as usize].state = CellState::Locked;
                self.grid[abs_y as usize][abs_x as usize] = Some(7);
            }
        }
        self.active_quadshape = None;
    }

    fn clear_completed_rows(&mut self) {
        let mut rows_to_clear: Vec<usize> = Vec::new();
        for y in 0..GRID_HEIGHT {
            let mut rows_completed = true;
            for x in 0..GRID_WIDTH {
                if self.grid[y][x].is_none() {
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
                        self.grid[y][x] = self.grid[y - 1][x];
                    }
                }
                for x in 0..GRID_WIDTH {
                    self.grid[0][x] = None;
                }
            }
            let num_lines_cleared = rows_to_clear.len();
            self.score += 100_usize * num_lines_cleared * num_lines_cleared;
            self.level = self.score / 1000 + 1;
        }
    }

    pub fn update_game_state(&mut self) {
        if self.down_held {
            self.move_quadshape_down();
        } else if self.active_quadshape.is_none() {
            self.create_quadshape();
        } else if !self.quadshape_should_lock() {
            self.move_quadshape_down();
        } else {
            self.lock_quadshape();
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

    pub fn move_quadshape_left(&mut self) {
        if let Some(ref mut quadshape) = self.active_quadshape {
            let new_x = quadshape.x - 1;
            if new_x >= -1
                || QUAD_SHAPES[quadshape.shape][quadshape.rotation] != [(1, 0), (2, 0), (2, 1)]
            {
                if new_x >= -1 && !quadshape_collides(quadshape, new_x, quadshape.y, &self.grid) {
                    quadshape.x = new_x;
                }
            }
        }
    }

    pub fn move_quadshape_right(&mut self) {
        if let Some(ref mut quadshape) = self.active_quadshape {
            let new_x = quadshape.x + 1;
            if new_x < GRID_WIDTH as i32
                && !quadshape_collides(quadshape, new_x, quadshape.y, &self.grid)
            {
                quadshape.x = new_x;
            }
        }
    }

    pub fn rotate_quadshape(&mut self) {
        if let Some(ref mut quadshape) = self.active_quadshape {
            if quadshape_can_rotate(quadshape, &self.grid) {
                let new_rotation = (quadshape.rotation + 1) % 4;
                quadshape.rotation = new_rotation;
            }
        }
    }

    pub fn move_quadshape_down(&mut self) {
        if let Some(ref mut quadshape) = self.active_quadshape {
            let new_y = quadshape.y + 1;
            if new_y < GRID_HEIGHT as i32
                && !quadshape_collides(quadshape, quadshape.x, new_y, &self.grid)
            {
                quadshape.y = new_y;
            }
        }
    }

    pub fn game_over_condition(&self) -> bool {
        for &(x, y) in QUAD_SHAPES[0][0] {
            let abs_x = (GRID_WIDTH as i32 / 2) - 1 + x;
            let abs_y = y;
            if self.grid[abs_y as usize][abs_x as usize].is_some() {
                return true;
            }
        }
        false
    }

    pub fn create_quadshape(&mut self) {
        let shape_index = rand::rng().random_range(0..QUAD_SHAPES.len());
        let rotation = 0;
        let active_block_texture = shape_index;
        // initial position
        let x = (GRID_WIDTH as i32 / 2) - 1;
        let y = 0;
        let temp_quadshape = Quadshape {
            x,
            y,
            shape: shape_index,
            rotation,
            active_block_texture,
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

fn quadshape_collides(
    quadshape: &Quadshape,
    x: i32,
    y: i32,
    grid: &[[Option<usize>; GRID_WIDTH]; GRID_HEIGHT],
) -> bool {
    for &(block_x, block_y) in QUAD_SHAPES[quadshape.shape][quadshape.rotation] {
        let new_x = x + block_x;
        let new_y = y + block_y;
        if new_x < 0 || new_x >= GRID_WIDTH as i32 || new_y >= GRID_HEIGHT as i32 {
            return true;
        }
        if new_y >= 0 && grid[new_y as usize][new_x as usize].is_some() {
            return true;
        }
    }
    false
}

fn quadshape_can_rotate(
    quadshape: &Quadshape,
    grid: &[[Option<usize>; GRID_WIDTH]; GRID_HEIGHT],
) -> bool {
    let new_rotation = (quadshape.rotation + 1) % 4;
    let temp_quadshape = Quadshape {
        x: quadshape.x,
        y: quadshape.y,
        shape: quadshape.shape,
        rotation: new_rotation,
        active_block_texture: quadshape.active_block_texture,
    };
    for &(block_x, block_y) in QUAD_SHAPES[temp_quadshape.shape][temp_quadshape.rotation] {
        let new_x = temp_quadshape.x + block_x;
        let new_y = temp_quadshape.y + block_y;
        if new_x < 0
            || new_x >= GRID_WIDTH as i32
            || new_y >= GRID_HEIGHT as i32
            || (new_y >= 0 && grid[new_y as usize][new_x as usize].is_some())
        {
            return false;
        }
    }
    true
}
