use crate::game::components::projectile::Projectile;
use hecs::World;
use micro_games_kit::third_party::vek::Transform;

pub struct ProjectileController;

impl ProjectileController {
    pub fn run(world: &World, delta_time: f32) {
        for (_, (projectile, transform)) in world
            .query::<(&mut Projectile, &mut Transform<f32, f32, f32>)>()
            .iter()
        {
            projectile.alive_time += delta_time;
            transform.position += projectile.velocity * delta_time;
        }
    }
}
