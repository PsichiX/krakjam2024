use crate::game::{
    components::{
        collidable::Collidable,
        effect::{Effect, EffectReaction},
        enemy::Enemy,
        health::Health,
        ignore_entity::IgnoreEntity,
        immobility::Immobility,
        movement::Movement,
        particle::Particle,
        speed::Speed,
    },
    utils::space::Space,
};
use hecs::{Entity, World};
use micro_games_kit::third_party::vek::{Transform, Vec2};

pub struct EffectsReactions;

impl EffectsReactions {
    pub fn run(world: &mut World) {
        let space = Space::read();
        let space = space.read().unwrap();
        let mut entities_to_process = Vec::<(Entity, Entity)>::new();

        for (entity_a, (collidable, _)) in world.query::<(&Collidable, &Effect)>().iter() {
            if let Some(space_object) = collidable.space_object.as_ref() {
                for object in space.collisions(space_object, true) {
                    if let Some(entity_b) = object.entity {
                        if entity_b != entity_a {
                            if IgnoreEntity::should_be_ignored(&world, entity_a, entity_b) {
                                continue;
                            }

                            entities_to_process.push((entity_a, entity_b));
                        }
                    }
                }
            }
        }

        for (entity_a, entity_b) in entities_to_process {
            let mut reaction = EffectReaction::None;
            let mut reaction_transform = Transform::default();

            {
                let mut query = world.query::<(
                    &mut Effect,
                    Option<&mut Health>,
                    Option<&mut Immobility>,
                    Option<&mut Transform<f32, f32, f32>>,
                    Option<&mut Speed>,
                    Option<&mut Enemy>,
                )>();
                let mut view = query.view();
                let [entity_a_query, entity_b_query] = view.get_mut_n([entity_a, entity_b]);

                if let Some((effect_a, mut health_a, immobility_a, mut transform_a, ..)) =
                    entity_a_query
                {
                    if let Some((effect_b, mut health_b, immobility_b, mut transform_b, ..)) =
                        entity_b_query
                    {
                        reaction = effect_a.react(effect_b);
                        let damage = reaction.damage();
                        let immobile_time = reaction.immobile_time();
                        let push_distance = reaction.push_distance();
                        if let Some(health) = health_a.as_mut() {
                            health.value = (health.value - damage).max(0.0);
                        }
                        if let Some(health) = health_b.as_mut() {
                            health.value = (health.value - damage).max(0.0);
                        }
                        if let Some(immobility) = immobility_a {
                            if immobility.time_left <= 0.0 {
                                immobility.time_left = immobile_time;
                            }
                        }
                        if let Some(immobility) = immobility_b {
                            if immobility.time_left <= 0.0 {
                                immobility.time_left = immobile_time;
                            }
                        }
                        if let (Some(transform_a), Some(transform_b)) =
                            (transform_a.as_mut(), transform_b.as_mut())
                        {
                            let direction_ab = (transform_b.position.xy()
                                - transform_a.position.xy())
                            .try_normalized()
                            .unwrap_or_default();
                            transform_a.position -= direction_ab * push_distance;
                            transform_b.position += direction_ab * push_distance;

                            reaction_transform = transform_a.clone();
                        }
                        // if let Some(speed) = speed_a.as_mut() {
                        //     if let Some(enemy) = enemy_a.as_mut() {
                        //         speed.value = 30.0;
                        //         enemy.direction += enemy.direction;
                        //     }
                        //     else {
                        //         speed.value = 0.0;
                        //     }
                        // }
                        // if let Some(speed) = speed_b.as_mut() {
                        //     if let Some(enemy) = enemy_b.as_mut() {
                        //         speed.value = 30.0;
                        //         enemy.direction += enemy.direction;
                        //     }
                        //     else {
                        //         speed.value = 0.0;
                        //     }
                        // }
                    }
                }
            }

            if reaction != EffectReaction::None {
                for _ in 0..50 {
                    match reaction {
                        EffectReaction::None => {}
                        EffectReaction::Explode => {
                            let mut transform = Transform::<f32, f32, f32>::default();
                            transform.position = reaction_transform.position;

                            world.spawn((
                                transform,
                                Particle::new("particle/explosion".into(), 0.5..=2.0, 2.0..=4.0),
                                Movement::new(
                                    Particle::generate_velocity(
                                        100.0..=200.0,
                                        180.0f32.to_radians(),
                                    ),
                                    Default::default(),
                                ),
                            ));
                        }
                        EffectReaction::Paralize => {
                            let mut transform = Transform::<f32, f32, f32>::default();
                            transform.position = reaction_transform.position;

                            world.spawn((
                                transform,
                                Particle::new("particle/paralized".into(), 0.5..=1.5, 0.4..=1.2),
                                Movement::new(
                                    Particle::generate_velocity(40.0..=80.0, 180.0f32.to_radians()),
                                    Default::default(),
                                ),
                            ));
                        }
                        EffectReaction::Steam => {
                            let mut transform = Transform::<f32, f32, f32>::default();
                            transform.position = reaction_transform.position;

                            world.spawn((
                                transform,
                                Particle::new("particle/steam".into(), 0.5..=1.0, 1.0..=2.0),
                                Movement::new(
                                    Particle::generate_velocity(
                                        60.0..=100.0,
                                        180.0f32.to_radians(),
                                    ),
                                    Default::default(),
                                ),
                            ));
                        }
                    };
                }
            }
        }
    }
}
