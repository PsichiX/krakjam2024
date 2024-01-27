use hecs::{Entity, World};

pub struct IgnorePlayer {
    pub ignore_time: f32,
}

impl IgnorePlayer {
    pub fn should_be_ignored(world: &World, entity: Entity) -> bool {
        if let Ok(ignore_player) = world.get::<&IgnorePlayer>(entity) {
            return ignore_player.ignore_time > 0.0;
        }

        false
    }
}
