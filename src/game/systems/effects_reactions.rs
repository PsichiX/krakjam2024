use crate::game::{
    components::{
        collidable::Collidable,
        effect::{Effect, EffectReaction},
        health::Health,
        ignore_player::IgnorePlayer,
        immobility::Immobility,
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

        for (entity_a, (collidable, _, ignore_player)) in world
            .query::<(&Collidable, &Effect, Option<&IgnorePlayer>)>()
            .iter()
        {
            if let Some(space_object) = collidable.space_object.as_ref() {
                for object in space.collisions(space_object, true) {
                    if let Some(entity_b) = object.entity {
                        if entity_b != entity_a {
                            if let Some(ignore_player) = ignore_player {
                                if ignore_player.should_be_ignored(entity_a)
                                    && ignore_player.should_be_ignored(entity_b)
                                {
                                    continue;
                                }
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
                )>();
                let mut view = query.view();
                let [entity_a_query, entity_b_query] = view.get_mut_n([entity_a, entity_b]);

                if let Some((
                    effect_a,
                    mut health_a,
                    mut immobility_a,
                    mut transform_a,
                    mut speed_a,
                )) = entity_a_query
                {
                    if let Some((
                        effect_b,
                        mut health_b,
                        mut immobility_b,
                        mut transform_b,
                        mut speed_b,
                    )) = entity_b_query
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
                        if let Some(immobility) = immobility_a.as_mut() {
                            immobility.time_left = immobile_time;
                        }
                        if let Some(immobility) = immobility_b.as_mut() {
                            immobility.time_left = immobile_time;
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
                        if let Some(speed) = speed_a.as_mut() {
                            speed.value = 0.0;
                        }
                        if let Some(speed) = speed_b.as_mut() {
                            speed.value = 0.0;
                        }
                    }
                }
            }

            if reaction != EffectReaction::None {
                for _ in 0..50 {
                    match reaction {
                        EffectReaction::None => {}
                        EffectReaction::Explode => {
                            world.spawn((Particle::new(
                                match reaction {
                                    EffectReaction::None => "particle/smoke".into(),
                                    EffectReaction::Explode => "particle/explosion".into(),
                                    EffectReaction::Steam => "particle/steam".into(),
                                    EffectReaction::Paralize => "particle/paralized".into(),
                                },
                                reaction_transform.position.into(),
                                Vec2::<f32>::zero(),
                                180.0f32.to_radians(),
                                100.0..=200.0,
                                0.5..=2.0,
                                2.0..=4.0,
                            ),));
                        }
                        EffectReaction::Paralize => {
                            world.spawn((Particle::new(
                                match reaction {
                                    EffectReaction::None => "particle/smoke".into(),
                                    EffectReaction::Explode => "particle/explosion".into(),
                                    EffectReaction::Steam => "particle/steam".into(),
                                    EffectReaction::Paralize => "particle/paralized".into(),
                                },
                                reaction_transform.position.into(),
                                Vec2::<f32>::zero(),
                                180.0f32.to_radians(),
                                40.0..=80.0,
                                0.5..=1.5,
                                0.4..=1.2,
                            ),));
                        }
                        EffectReaction::Steam => {
                            world.spawn((Particle::new(
                                match reaction {
                                    EffectReaction::None => "particle/smoke".into(),
                                    EffectReaction::Explode => "particle/explosion".into(),
                                    EffectReaction::Steam => "particle/steam".into(),
                                    EffectReaction::Paralize => "particle/paralized".into(),
                                },
                                reaction_transform.position.into(),
                                Vec2::<f32>::zero(),
                                180.0f32.to_radians(),
                                60.0..=100.0,
                                0.5..=1.0,
                                1.0..=2.0,
                            ),));
                        }
                    };
                }
            }
        }
    }
}
