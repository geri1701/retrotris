use rand::Rng;
#[derive(Default)]
pub enum Scene {
    #[default]
    Welcome = 0,
    Playing,
    Paused,
    GameOver,
}

#[derive(Default)]
pub enum Difficulty {
    #[default]
    Normal = 0,
    Hard,
    Easy,
}

impl Difficulty {
    pub fn limit(&self) -> (f32, f32) {
        const CACTUS_MIN_GAP: f32 = 320.0;
        const CACTUS_MAX_GAP: f32 = 520.0;
        (
            CACTUS_MIN_GAP * self.gap_scale(),
            CACTUS_MAX_GAP * self.gap_scale(),
        )
    }
    pub fn scaled_gap(&self, first: bool) -> f32 {
        let (min, max) = self.limit();
        match first {
            true => (min + max) * 0.5,
            false => rand::rng().random_range(min..max),
        }
    }
    pub fn switch(&self) -> Self {
        match self {
            Self::Easy => Self::Normal,
            Self::Normal => Self::Hard,
            Self::Hard => Self::Easy,
        }
    }
    pub fn label(&self) -> &'static str {
        match self {
            Self::Easy => "Easy",
            Self::Normal => "Normal",
            Self::Hard => "Hard",
        }
    }
    pub fn speed_mul(&self) -> f32 {
        match self {
            Self::Easy => 0.9,
            Self::Normal => 1.0,
            Self::Hard => 1.12,
        }
    }
    pub fn gap_scale(&self) -> f32 {
        match self {
            Self::Easy => 1.15,
            Self::Normal => 1.0,
            Self::Hard => 0.85,
        }
    }
    pub fn ptero_enabled(&self) -> bool {
        matches!(self, Self::Normal | Self::Hard)
    }
}

pub enum ObstacleKind {
    Cactus(bool),      // 30x46
    Pterodactyl(bool), // flies at given y
}

pub struct Obstacle {
    pub rect: (f32, f32, f32, f32),
    pub kind: ObstacleKind,
}

#[derive(Default)]
pub struct Dino {
    pub ducking: bool,
    pub jumping: bool,
    pub leg_state: bool,
    pub leg_timer: f32,
    pub y: f32,
    pub vy: f32,
}

impl Dino {
    pub fn x(&self) -> f32 {
        120.0
    }
    pub fn w(&self) -> f32 {
        44.0
    }
    pub fn height(&self) -> f32 {
        if self.ducking { 30.0 } else { 48.0 }
    }
}
