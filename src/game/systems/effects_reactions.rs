use crate::game::{
    components::{collidable::Collidable, effect::Effect, health::Health, immobility::Immobility},
    utils::space::Space,
};
use hecs::World;
use micro_games_kit::third_party::vek::Transform;

pub struct EffectsReactions;

impl EffectsReactions {
    pub fn run(world: &World) {
        let space = Space::read();
        let space = space.read().unwrap();

        for (id, (collidable, effect_a, mut health_a, mut immobility_a, mut transform_a)) in world
            .query::<(
                &Collidable,
                &mut Effect,
                Option<&mut Health>,
                Option<&mut Immobility>,
                Option<&mut Transform<f32, f32, f32>>,
            )>()
            .iter()
        {
            if let Some(space_object) = collidable.space_object.as_ref() {
                for object in space.collisions(space_object, true) {
                    if let Some(entity) = object.entity {
                        if entity == id {
                            continue;
                        }

                        if let Ok(query) = world
                            .query_one::<(
                                &mut Effect,
                                Option<&mut Health>,
                                Option<&mut Immobility>,
                                Option<&mut Transform<f32, f32, f32>>,
                            )>(entity)
                            .as_mut()
                        {
                            if let Some((
                                effect_b,
                                mut health_b,
                                mut immobility_b,
                                mut transform_b,
                            )) = query.get()
                            {
                                let reaction = effect_a.react(effect_b);
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
                                    .normalized();
                                    transform_a.position -= direction_ab * push_distance;
                                    transform_b.position += direction_ab * push_distance;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
