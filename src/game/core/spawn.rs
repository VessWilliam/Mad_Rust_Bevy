use bevy::prelude::*;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct SpawnArea {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl SpawnArea {
    #[inline]
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.min_x < self.max_x && self.min_y < self.max_y
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SpawnEdge {
    Top,
    Bottom,
    Left,
    Right,
}

impl SpawnEdge {
    #[inline]
    pub fn random(rng: &mut impl Rng) -> Self {
        match rng.random_range(0..4) {
            0 => Self::Top,
            1 => Self::Bottom,
            2 => Self::Left,
            _ => Self::Right,
        }
    }

    #[inline]
    pub fn as_string(&self) -> &'static str {
        match self {
            Self::Top => "Top",
            Self::Bottom => "Bottom",
            Self::Left => "Left",
            Self::Right => "Right",
        }
    }

    pub fn calculate_spawn(
        &self,
        rng: &mut impl Rng,
        area: SpawnArea,
        margin: f32,
        speed: f32,
    ) -> (f32, f32, Vec2) {
        match self {
            Self::Top => {
                let x = rng.random_range(area.min_x..area.max_x);
                let y = area.max_y - margin;
                let velocity = Vec2::new(rng.random_range(-speed..speed), -speed);
                (x, y, velocity)
            }
            Self::Bottom => {
                let x = rng.random_range(area.min_x..area.max_x);
                let y = area.min_y + margin;
                let velocity = Vec2::new(rng.random_range(-speed..speed), speed);
                (x, y, velocity)
            }
            Self::Left => {
                let x = area.min_x + margin;
                let y = rng.random_range(area.min_y..area.max_y);
                let velocity = Vec2::new(speed, rng.random_range(-speed..speed));
                (x, y, velocity)
            }
            Self::Right => {
                let x = area.max_x - margin;
                let y = rng.random_range(area.min_y..area.max_y);
                let velocity = Vec2::new(-speed, rng.random_range(-speed..speed));
                (x, y, velocity)
            }
        }
    }
}
