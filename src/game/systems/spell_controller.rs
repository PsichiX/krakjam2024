use hecs::World;
use micro_games_kit::third_party::{
    kira::track::effect,
    vek::{Transform, Vec2},
};

use crate::game::{
    components::{collidable::Collidable, effect::Effect, projectile::Projectile, spell::Spell},
    utils::magic::spell_tag::{SpellTagDirection, SpellTagSpeed, SpellTagTrajectory},
};

pub struct SpellController;

impl SpellController {
    pub fn run(world: &mut World) {
        // Velocity calculation
        for (entity, (projectile, spell)) in world.query::<(&mut Projectile, &Spell)>().iter() {
            let time_divider = match spell.speed {
                SpellTagSpeed::Fast => 0.5,
                SpellTagSpeed::Medium => 1.0,
                SpellTagSpeed::Slow => 1.5,
            };

            match spell.trajectory {
                SpellTagTrajectory::Straight => {
                    projectile.velocity = projectile.direction * projectile.speed;
                }
                SpellTagTrajectory::Circle => {
                    let circle_direction = Vec2::new(
                        (projectile.alive_time / time_divider * std::f32::consts::PI).cos(),
                        (projectile.alive_time / time_divider * std::f32::consts::PI).sin(),
                    );

                    projectile.velocity =
                        projectile.direction * projectile.speed * 1.5 * circle_direction;
                }
                SpellTagTrajectory::Sinus => {
                    let perpendicular_direction =
                        Vec2::new(projectile.direction.y, -projectile.direction.x);

                    projectile.velocity = projectile.direction * projectile.speed
                        + projectile.speed
                            * 1.5
                            * (projectile.alive_time / time_divider * 2.0 * std::f32::consts::PI)
                                .sin()
                            * perpendicular_direction;
                }
            }

            match spell.direction {
                SpellTagDirection::Backward => {
                    projectile.velocity *= -1.0;
                }
                SpellTagDirection::Forward => {}
                SpellTagDirection::Down => projectile.velocity = Vec2::zero(),
            }
        }

        // Size calculation
        for (_, (transform, spell, collidable)) in world
            .query::<(&mut Transform<f32, f32, f32>, &Spell, &mut Collidable)>()
            .iter()
        {
            transform.scale = spell.size.scale().into();

            if let Some(space_object) = collidable.space_object.as_mut() {
                space_object.collider_radius = spell.size.radius();
            }
        }
    }
}
