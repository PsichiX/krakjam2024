use std::vec;

use hecs::World;
use micro_games_kit::{context::GameContext, third_party::vek::Transform};

use crate::game::{
    components::{collidable::Collidable, enemy::Enemy, spell::Spell},
    utils::{
        events::{Event, Events},
        space::{Space, SpaceObject, SpaceObjectId},
    },
};

pub struct CollisionDetector;

impl CollisionDetector {
    pub fn run(world: &World, context: &mut GameContext, delta_time: f32) {
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
        let mut space_objects = vec::Vec::<SpaceObject>::new();

        for (_, (collidable,)) in world.query::<(&Collidable,)>().iter() {
            if let Some(space_object) = collidable.space_object.as_ref() {
                space_objects.push(space_object.clone());
            }
        }

        Space::write().write().unwrap().maintain(space_objects);

        // Finally resolve collisions
        Self::resolve_collisions(world);
    }

    fn resolve_collisions(world: &World) {
        let space = Space::read();
        let space = space.read().unwrap();

        for (id, (_, collidable)) in world.query::<(&Spell, &Collidable)>().iter() {
            if let Some(space_object) = collidable.space_object.as_ref() {
                for object in space.collisions(space_object, true) {
                    if let Some(entity) = object.entity {
                        let mut query = world.query_one::<(&Enemy,)>(entity);

                        if let Ok(query) = query.as_mut() {
                            if query.get().is_some() {
                                Events::write(Event::KillEntity { entity });
                                Events::write(Event::KillEntity { entity: id });
                            }
                        }
                    }
                }
            }
        }
    }
}
