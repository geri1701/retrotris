use rand::Rng;

const GRID_WIDTH: usize = 20;
const GRID_HEIGHT: usize = 32;
const FIGURES: [[[(i32, usize); 4]; 4]; 7] = [
    [
        [(0, 0), (0, 1), (1, 0), (1, 1)],
        [(0, 0), (0, 1), (1, 0), (1, 1)],
        [(0, 0), (0, 1), (1, 0), (1, 1)],
        [(0, 0), (0, 1), (1, 0), (1, 1)],
    ], // O
    [
        [(0, 0), (0, 1), (0, 2), (0, 3)],
        [(0, 0), (1, 0), (2, 0), (3, 0)],
        [(0, 0), (0, 1), (0, 2), (0, 3)],
        [(0, 0), (1, 0), (2, 0), (3, 0)],
    ], // I
    [
        [(0, 0), (1, 0), (1, 1), (2, 0)],
        [(0, 1), (1, 0), (1, 1), (1, 2)],
        [(0, 1), (1, 0), (1, 1), (2, 1)],
        [(0, 0), (0, 1), (1, 1), (0, 2)],
    ], // T
    [
        [(0, 1), (1, 1), (1, 0), (2, 0)],
        [(0, 0), (0, 1), (1, 1), (1, 2)],
        [(0, 1), (1, 1), (1, 0), (2, 0)],
        [(0, 0), (0, 1), (1, 1), (1, 2)],
    ], // S
    [
        [(0, 0), (1, 0), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (0, 1), (0, 2)],
        [(0, 0), (1, 0), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (0, 1), (0, 2)],
    ], // Z
    [
        [(1, 0), (1, 1), (1, 2), (0, 2)], // _|
        [(0, 0), (0, 1), (1, 1), (2, 1)], // |_
        [(0, 0), (1, 0), (0, 1), (0, 2)], // |-
        [(0, 0), (1, 0), (2, 0), (2, 1)], // -|
    ], // J
    [
        [(0, 0), (0, 1), (0, 2), (1, 2)], // |_
        [(0, 0), (1, 0), (2, 0), (0, 1)], // |-
        [(0, 0), (1, 0), (1, 1), (1, 2)], // -|
        [(0, 1), (1, 1), (2, 1), (2, 0)], // _|
    ], // L
];

#[derive(Default)]
struct Figure {
    coor: [(i32, usize); 4],
    shape: (usize, usize),
}

impl Figure {
    fn new(x: i32, y: usize, color: usize, rotate: usize) -> Self {
        Self {
            shape: (color, rotate),
            coor: FIGURES[color][rotate].map(|(i, j)| (x + i, y + j)),
        }
    }
    fn shift(&self, shift: (i32, usize)) -> Self {
        Self {
            shape: self.shape,
            coor: self.coor.map(|coor| (coor.0 + shift.0, coor.1 + shift.1)),
        }
    }
    fn rotate(&self) -> Self {
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

pub struct Model {
    pub value: (i32, u64),
    grid: Vec<[Option<usize>; GRID_WIDTH]>,
    curr: Figure,
    next: usize,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            value: (0, 0),
            grid: vec![[None; GRID_WIDTH]; GRID_HEIGHT],
            curr: Figure::default(),
            next: rand::rng().random_range(0..FIGURES.len()),
        }
    }
}

impl Model {
    pub fn set_grid(&mut self) {
        self.grid = vec![[None; GRID_WIDTH]; GRID_HEIGHT];
    }
    pub fn set_curr(&mut self) {
        self.curr = Figure::new(10, 0, self.next, 0);
        self.next = rand::rng().random_range(0..FIGURES.len());
    }
    pub fn field(&self) -> Vec<Vec<Option<usize>>> {
        let mut field = Vec::new();
        for line in &self.grid {
            field.push(Vec::from(line));
        }
        for (x, y) in self.curr.coor {
            field[y][x as usize] = Some(self.curr.shape.0);
        }
        field
    }
    pub fn next(&self) -> [[Option<usize>; 4]; 4] {
        let mut next = [[None; 4]; 4];
        for (x, y) in FIGURES[self.next][0] {
            next[y][x as usize] = Some(self.next);
        }
        next
    }
    fn check(&self, temp: Figure) -> Option<Figure> {
        for (x, y) in temp.coor {
            if !(0..self.grid.len()).contains(&y)
                || !(0..self.grid[y].len() as i32).contains(&x)
                || self.grid[y][x as usize].is_some()
            {
                return None;
            }
        }
        Some(temp)
    }
    pub fn shift(&mut self, direction: (i32, usize)) {
        if let Some(temp) = self.check(self.curr.shift(direction)) {
            self.curr = temp;
        }
    }
    pub fn rotate(&mut self) {
        if let Some(temp) = self.check(self.curr.rotate()) {
            self.curr = temp;
        }
    }
    pub fn down(&mut self) -> bool {
        if let Some(temp) = self.check(self.curr.shift((0, 1))) {
            self.curr = temp;
        } else {
            for (x, y) in self.curr.coor {
                if y == 0 {
                    return false;
                } else {
                    self.grid[y][x as usize] = Some(self.curr.shape.0);
                }
            }
            self.set_curr();
        };
        true
    }
    fn find_full_line(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for line in 0..self.grid.len() {
            if !self.grid[line].contains(&None) {
                result.push(line);
            };
        }
        result
    }
    pub fn play(&mut self) -> bool {
        for line in self.find_full_line() {
            self.grid.remove(line);
            self.grid.insert(0, [None; GRID_WIDTH]);
            self.value.0 += 1;
        }
        if !self.down() {
            return false;
        }
        self.value.1 = match self.value.0 {
            0..1 => 0,
            1..2 => 1,
            2..3 => 2,
            _ => 3,
        };
        true
    }
}
