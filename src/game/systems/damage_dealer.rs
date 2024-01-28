use crate::game::{
    components::{
        collidable::Collidable, damage::Damage, health::Health, ignore_entity::IgnoreEntity,
    },
    utils::space::Space,
};
use hecs::{Entity, World};

pub struct DamageDealer;

impl DamageDealer {
    pub fn run(world: &World) {
        let space = Space::read();
        let space = space.read().unwrap();
        let mut entities_to_damage = Vec::<(Damage, Entity)>::new();

        for (entity_a, (collidable, damage)) in world.query::<(&Collidable, &Damage)>().iter() {
            if let Some(space_object) = collidable.space_object.as_ref() {
                for object in space.collisions(space_object, true) {
                    if let Some(entity_b) = object.entity {
                        if entity_b != entity_a {
                            if IgnoreEntity::should_be_ignored(world, entity_a, entity_b) {
                                continue;
                            }

                            entities_to_damage.push((damage.clone(), entity_b));
                        }
                    }
                }
            }
        }

        for (damage, entity) in entities_to_damage {
            let mut query = world.query::<&mut Health>();
            let mut view = query.view();
            let [entity_query] = view.get_mut_n([entity]);

            if let Some(health) = entity_query {
                if !damage.layer.allow_damage(health.layer) {
                    continue;
                }

                health.value -= damage.value;
            }
        }
    }
}
