use hecs::World;
use micro_games_kit::{
    context::GameContext,
    third_party::vek::{Transform, Vec2},
};

use crate::game::{
    components::{collidable::Collidable, projectile::Projectile, spell::Spell},
    utils::magic::spell_tag::{SpellTagDirection, SpellTagSize, SpellTagSpeed, SpellTagTrajectory},
};

pub struct SpellController;

impl SpellController {
    pub fn run(world: &World, _context: &mut GameContext) {
        // Velocity calculation
        for (_, (projectile, spell)) in world.query::<(&mut Projectile, &Spell)>().iter() {
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
            transform.scale = match spell.size {
                SpellTagSize::Large => Vec2::new(4.0, 4.0).into(),
                SpellTagSize::Medium => Vec2::new(2.0, 2.0).into(),
                SpellTagSize::Small => Vec2::new(1.0, 1.0).into(),
            };

            if let Some(space_object) = collidable.space_object.as_mut() {
                space_object.collider_radius = match spell.size {
                    SpellTagSize::Large => 40.0,
                    SpellTagSize::Medium => 20.0,
                    SpellTagSize::Small => 10.0,
                };
            }
        }
    }
}