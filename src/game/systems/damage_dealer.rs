use crate::game::{
    components::{
        collidable::Collidable, damage::Damage, health::Health, ignore_player::IgnorePlayer,
    },
    utils::space::Space,
};
use hecs::{Entity, World};

pub struct DamageDealer;

impl DamageDealer {
    pub fn run(world: &World) {
        let space = Space::read();
        let space = space.read().unwrap();
        let mut entities_to_damage = Vec::<(f32, Entity)>::new();

        for (entity_a, (collidable, damage, ignore_player)) in world
            .query::<(&Collidable, &Damage, Option<&IgnorePlayer>)>()
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

                            entities_to_damage.push((damage.value, entity_b));
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
                health.value -= damage;
            }
        }
    }
}
