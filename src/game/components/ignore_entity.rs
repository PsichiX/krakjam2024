use hecs::{Entity, World};

pub struct IgnoreEntity {
    pub ignore_time: f32,
    pub ignored_entity: Entity,
}

impl IgnoreEntity {
    pub fn should_be_ignored(world: &World, entity_a: Entity, entity_b: Entity) -> bool {
        if let Ok(ignore_player) = world.get::<&IgnoreEntity>(entity_a) {
            return ignore_player.ignored_entity == entity_b && ignore_player.ignore_time > 0.0;
        }

        if let Ok(ignore_player) = world.get::<&IgnoreEntity>(entity_b) {
            return ignore_player.ignored_entity == entity_a && ignore_player.ignore_time > 0.0;
        }

        false
    }
}
