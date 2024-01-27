use micro_games_kit::third_party::vek::Vec2;

use crate::game::utils::magic::spell_tag::{SpellTag, SpellTagTrajectory};

pub struct Projectile {
    pub speed: f32,
    pub direction: Vec2<f32>,
    pub trajectory: SpellTagTrajectory,
    pub trajectory_time: f32,
}

impl Projectile {
    pub fn new(speed: f32, direction: Vec2<f32>, trajectory: SpellTagTrajectory) -> Self {
        Projectile {
            speed,
            direction,
            trajectory,
            trajectory_time: 0.0,
        }
    }
}
