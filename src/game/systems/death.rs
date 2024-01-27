use crate::game::components::health::Health;
use hecs::World;

pub struct Death;

impl Death {
    pub fn run(world: &mut World) {
        let to_destroy = world
            .query::<&Health>()
            .iter()
            .filter(|(_, health)| health.value <= 0.0)
            .map(|(entity, _)| entity)
            .collect::<Vec<_>>();

        for entity in to_destroy {
            let _ = world.despawn(entity);
        }
    }
}
