use std::ops::DerefMut;

use crate::game::{
    components::{
        effect::Effect, enemy::Enemy, immobility::Immobility, player::Player, speed::Speed,
        spell::Spell,
    },
    states::new_gameplay::NewGameplay,
    utils::magic::spell_tag::{
        SpellTagDirection, SpellTagDuration, SpellTagEffect, SpellTagShape, SpellTagSize,
        SpellTagSpeed, SpellTagTrajectory,
    },
};
use hecs::{Entity, World};
use micro_games_kit::third_party::{
    rand::{thread_rng, Rng},
    vek::{Transform, Vec2},
};

use super::player_controller::PlayerCastAction;

pub struct EnemyController;

impl EnemyController {
    pub fn run(world: &mut World, delta_time: f32) {
        let mut rng = thread_rng();
        let mut cast_spells = Vec::<(Entity, PlayerCastAction)>::new();

        if let Some(player_position) = world
            .query::<&Transform<f32, f32, f32>>()
            .with::<&Player>()
            .iter()
            .next()
            .map(|(_, transform)| transform.position.xy())
        {
            for (entity, (enemy, speed, transform, immobility, effect)) in world
                .query::<(
                    &mut Enemy,
                    &mut Speed,
                    &mut Transform<f32, f32, f32>,
                    Option<&Immobility>,
                    Option<&Effect>,
                )>()
                .iter()
            {
                let to_player_direction = (player_position - transform.position.xy())
                    .try_normalized()
                    .unwrap_or_default();

                enemy.direction += to_player_direction.rotated_z(enemy.direction_rotation);
                enemy.direction.normalize();

                let mut velocity = enemy.direction * speed.value * delta_time;

                if let Some(immobility) = immobility {
                    if immobility.time_left > 0.0 {
                        velocity = Vec2::<f32>::zero();
                    }
                }

                if velocity.x >= 0.0 {
                    transform.scale.x = transform.scale.x.abs();
                } else {
                    transform.scale.x = -transform.scale.x.abs();
                }
                transform.position += velocity;
                speed.value =
                    (speed.value + enemy.acceleration * delta_time).min(enemy.speed_limit);

                enemy.shoot_cooldown -= delta_time;

                if enemy.shoot_cooldown <= 0.0 {
                    enemy.direction_rotation =
                        rng.gen_range(-std::f32::consts::FRAC_PI_2..=std::f32::consts::FRAC_PI_2);

                    speed.value = 0.0;

                    enemy.shoot_cooldown = rng.gen_range(5.0..=15.0);

                    cast_spells.push((
                        entity,
                        PlayerCastAction {
                            direction: to_player_direction,
                            position: transform.position.into(),
                            spell: Spell {
                                direction: SpellTagDirection::Forward,
                                duration: match effect {
                                    None => SpellTagDuration::Instant,
                                    Some(v) => {
                                        if v.to_effect_tag() == SpellTagEffect::None {
                                            SpellTagDuration::Instant
                                        } else {
                                            SpellTagDuration::Medium
                                        }
                                    }
                                },
                                effect: match effect {
                                    None => SpellTagEffect::None,
                                    Some(v) => v.to_effect_tag(),
                                },
                                shape: SpellTagShape::Point,
                                size: SpellTagSize::Small,
                                speed: SpellTagSpeed::Medium,
                                trajectory: SpellTagTrajectory::Straight,
                            },
                        },
                    ));
                }
            }
        }

        for spell in cast_spells {
            NewGameplay::cast_spell(world, spell.1, spell.0);
        }
    }
}
