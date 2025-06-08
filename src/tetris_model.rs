use rand::Rng;

pub const GRID_WIDTH: usize = 15;
pub const GRID_HEIGHT: usize = 30;
pub const FIGURES: [[[(i32, usize); 4]; 4]; 7] = [
    [[(0, 0), (1, 0), (0, 1), (1, 1)]; 4], // Square (No rotation)
    [
        [(0, 0), (0, 1), (0, 2), (0, 3)], // Rotation 0
        [(0, 0), (1, 0), (2, 0), (3, 0)], // Rotation 1
        [(0, 0), (0, 1), (0, 2), (0, 3)], // Rotation 0
        [(0, 0), (1, 0), (2, 0), (3, 0)], // Rotation 1
    ], // Straight
    [
        [(0, 1), (1, 1), (1, 0), (2, 0)], // Rotation 0
        [(0, 0), (0, 1), (1, 1), (1, 2)], // Rotation 1
        [(0, 1), (1, 1), (1, 0), (2, 0)], // Rotation 0
        [(0, 0), (0, 1), (1, 1), (1, 2)], // Rotation 1
    ], // S
    [
        [(0, 0), (1, 0), (1, 1), (2, 1)], // Rotation 0
        [(1, 0), (1, 1), (0, 1), (0, 2)], // Rotation 1
        [(0, 0), (1, 0), (1, 1), (2, 1)], // Rotation 0
        [(1, 0), (1, 1), (0, 1), (0, 2)], // Rotation 1
    ], // Z
    [
        [(0, 0), (1, 0), (1, 1), (2, 0)], // Rotation 0
        [(0, 1), (1, 0), (1, 1), (1, 2)], // Rotation 1
        [(0, 1), (1, 0), (1, 1), (2, 1)], // Rotation 2
        [(0, 0), (0, 1), (1, 1), (0, 2)], // Rotation 3
    ], // T-Shaped Quadshape
    [
        [(1, 0), (1, 1), (1, 2), (0, 2)], // Rotation 0
        [(0, 0), (0, 1), (1, 1), (2, 1)], // Rotation 1
        [(0, 0), (1, 0), (0, 1), (0, 2)], // Rotation 2
        [(0, 0), (1, 0), (2, 0), (2, 1)], // Rotation 3
    ], // J
    [
        [(0, 0), (0, 1), (0, 2), (1, 2)], // Rotation 0
        [(0, 0), (1, 0), (2, 0), (0, 1)], // Rotation 1
        [(0, 0), (1, 0), (1, 1), (1, 2)], // Rotation 2
        [(0, 1), (1, 1), (2, 1), (2, 0)], // Rotation 3
    ], // L
];

#[derive(Default)]
pub struct Score(i32, u64);

impl Score {
    pub fn inc(&mut self) {
        self.0 += 1;
    }
    pub fn get(&self) -> (i32, u64) {
        (self.0, self.1)
    }
    pub fn update(&mut self) {
        self.1 = match self.0 {
            0..1 => 0,
            1..2 => 1,
            2..3 => 2,
            _ => 3,
        }
    }
}

#[derive(Default)]
pub struct Figure {
    pub coor: [(i32, usize); 4],
    pub shape: (usize, usize),
}

impl Figure {
    pub fn new(x: i32, y: usize, color: usize, rotate: usize) -> Self {
        Self {
            shape: (color, rotate),
            coor: FIGURES[color][rotate].map(|(i, j)| (x + i, y + j)),
        }
    }
    pub fn shift(&self, shift: (i32, usize)) -> Self {
        Self {
            shape: self.shape,
            coor: self.coor.map(|coor| (coor.0 + shift.0, coor.1 + shift.1)),
        }
    }
    pub fn rotate(&self) -> Self {
        let rotate = if (0..FIGURES[self.shape.0].len() - 1).contains(&self.shape.1) {
            self.shape.1 + 1
        } else {
            0
        };
        let (mut x, mut y) = (GRID_WIDTH as i32, GRID_HEIGHT);
        for (i, j) in self.coor {
            if x > i {
                x = i
            };
            if y > j {
                y = j
            }
        }
        Self::new(x, y, self.shape.0, rotate)
    }
}

pub struct Next(usize);

impl Default for Next {
    fn default() -> Self {
        Self(rand::rng().random_range(0..FIGURES.len()))
    }
}

impl Next {
    pub fn get(&self) -> usize {
        self.0
    }
    pub fn draw(&self) -> [[Option<usize>; 4]; 4] {
        let mut next = [[None; 4]; 4];
        for (x, y) in FIGURES[self.0][0] {
            next[y][x as usize] = Some(self.0);
        }
        next
    }
}

pub struct Grid(pub Vec<[Option<usize>; GRID_WIDTH]>);

impl Default for Grid {
    fn default() -> Self {
        Self(vec![[None; GRID_WIDTH]; GRID_HEIGHT])
    }
}

impl Grid {
    pub fn draw(&self, curr: &Figure) -> Vec<Vec<Option<usize>>> {
        let mut field = Vec::new();
        for line in &self.0 {
            field.push(Vec::from(line));
        }
        for (x, y) in curr.coor {
            field[y][x as usize] = Some(curr.shape.0);
        }
        field
    }
    pub fn check(&self, temp: Figure) -> Option<Figure> {
        for (x, y) in temp.coor {
            if !(0..self.0.len()).contains(&y)
                || !(0..self.0[y].len() as i32).contains(&x)
                || self.0[y][x as usize].is_some()
            {
                return None;
            }
        }
        Some(temp)
    }
    pub fn find_full_line(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for line in 0..self.0.len() {
            if !self.0[line].contains(&None) {
                result.push(line);
            };
        }
        result
    }
}
