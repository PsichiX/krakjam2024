use crate::game::utils::magic::spell_tag::SpellTagTrajectory;
use micro_games_kit::third_party::vek::Vec2;

pub struct Projectile {
    pub speed: f32,
    pub direction: Vec2<f32>,
    pub velocity: Vec2<f32>,
    pub alive_time: f32,
}

impl Projectile {
    pub fn new(speed: f32, direction: Vec2<f32>) -> Self {
        Projectile {
            speed,
            direction,
            velocity: direction * speed,
            alive_time: 0.0,
        }
    }
}
