use crate::game::{components::collidable::Collidable, utils::space::Space};
use hecs::World;
use micro_games_kit::third_party::vek::Transform;

pub struct CollisionDetector;

impl CollisionDetector {
    pub fn run(world: &World) {
        // Update space oobject positions
        for (id, (collidable, transform)) in world
            .query::<(&mut Collidable, &Transform<f32, f32, f32>)>()
            .iter()
        {
            if let Some(space_object) = collidable.space_object.as_mut() {
                space_object.entity = Some(id);
                space_object.position = transform.position.into();
            }
        }

        // Clone space_object and pass them to Space maintaint
        let space_objects = world
            .query::<(&Collidable,)>()
            .iter()
            .filter_map(|(_, (collidable,))| collidable.space_object.as_ref().cloned())
            .collect();
        Space::write().write().unwrap().maintain(space_objects);
    }
}
