use crate::game::{
    components::{collidable::Collidable, effect::Effect, health::Health, immobility::Immobility},
    utils::space::Space,
};
use hecs::{Entity, World};
use micro_games_kit::third_party::vek::Transform;

pub struct EffectsReactions;

impl EffectsReactions {
    pub fn run(world: &World) {
        let space = Space::read();
        let space = space.read().unwrap();
        let mut entities_to_process = Vec::<(Entity, Entity)>::new();

        for (entity_a, (collidable, _)) in world.query::<(&Collidable, &Effect)>().iter() {
            if let Some(space_object) = collidable.space_object.as_ref() {
                for object in space.collisions(space_object, true) {
                    if let Some(entity_b) = object.entity {
                        if entity_b == entity_a {
                            continue;
                        }

                        println!("Entity: {:?}, Id: {:?}", entity_b, entity_a);
                        entities_to_process.push((entity_a, entity_b));
                    }
                }
            }
        }

        for (entity_a, entity_b) in entities_to_process {
            let mut query = world.query::<(
                &mut Effect,
                Option<&mut Health>,
                Option<&mut Immobility>,
                Option<&mut Transform<f32, f32, f32>>,
            )>();
            let mut view = query.view();
            let [entity_a_query, entity_b_query] = view.get_mut_n([entity_a, entity_b]);

            if let Some((effect_a, mut health_a, mut immobility_a, mut transform_a)) =
                entity_a_query
            {
                println!("--- COLLISION");

                if let Some((effect_b, mut health_b, mut immobility_b, mut transform_b)) =
                    entity_b_query
                {
                    println!("--- TEST");

                    let reaction = effect_a.react(effect_b);
                    println!("--- REACTION: {:?}", reaction);

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
                        let direction_ab = (transform_b.position.xy() - transform_a.position.xy())
                            .try_normalized()
                            .unwrap_or_default();
                        transform_a.position -= direction_ab * push_distance;
                        transform_b.position += direction_ab * push_distance;
                    }
                }
            }
        }
    }
}
