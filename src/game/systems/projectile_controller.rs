use hecs::World;
use micro_games_kit::{context::GameContext, third_party::vek::Transform};

use crate::game::components::projectile::Projectile;

pub struct ProjectileController;

impl ProjectileController {
    pub fn run(world: &World, context: &mut GameContext, delta_time: f32) {
        for (_, (projectile, transform)) in world
            .query::<(&Projectile, &mut Transform<f32, f32, f32>)>()
            .iter()
        {
            transform.position += projectile.direction * projectile.speed * delta_time;
        }
    }
}
