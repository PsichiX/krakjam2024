use micro_games_kit::third_party::vek::Vec2;

use crate::game::utils::magic::spell_tag::SpellTagTrajectory;

pub struct Projectile {
    pub speed: f32,
    pub direction: Vec2<f32>,
    pub trajectory: SpellTagTrajectory,
}
