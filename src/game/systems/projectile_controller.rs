use hecs::World;
use micro_games_kit::{
    context::GameContext,
    third_party::vek::{Transform, Vec2},
};

use crate::game::{
    components::projectile::Projectile, utils::magic::spell_tag::SpellTagTrajectory,
};

pub struct ProjectileController;

impl ProjectileController {
    pub fn run(world: &World, _context: &mut GameContext, delta_time: f32) {
        for (_, (projectile, transform)) in world
            .query::<(&mut Projectile, &mut Transform<f32, f32, f32>)>()
            .iter()
        {
            projectile.trajectory_time += delta_time;

            match projectile.trajectory {
                SpellTagTrajectory::Straight => {
                    transform.position += projectile.direction * projectile.speed * delta_time;
                }
                SpellTagTrajectory::Circle => {
                    let circle_direction = Vec2::new(
                        (projectile.trajectory_time / 1.0 * std::f32::consts::PI).cos(),
                        (projectile.trajectory_time / 1.0 * std::f32::consts::PI).sin(),
                    );

                    transform.position +=
                        projectile.direction * projectile.speed / 10.0 * circle_direction;
                }
                SpellTagTrajectory::Sinus => {
                    let perpendicular_direction =
                        Vec2::new(projectile.direction.y, -projectile.direction.x);

                    transform.position += (projectile.direction * projectile.speed * delta_time)
                        + projectile.speed / 10.0
                            * (projectile.trajectory_time / 0.5 * std::f32::consts::PI).sin()
                            * perpendicular_direction;
                }
            }
        }
    }
}
