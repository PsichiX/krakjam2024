use std::vec;

use hecs::World;
use micro_games_kit::{context::GameContext, third_party::vek::Transform};

use crate::game::{components::{collidable::Collidable, enemy::Enemy, spell::Spell}, utils::{events::{Event, Events}, space::{self, Space, SpaceObject, SpaceObjectId}}};

pub struct CollisionDetector;

impl CollisionDetector {
    pub fn run(world: &World, context: &mut GameContext, delta_time: f32) {
        // Update space oobject positions
        for (id, (collidable, transform)) in world.query::<(&mut Collidable, &Transform<f32, f32, f32>)>().iter() {
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

        for (_, (_, collidable)) in world.query::<(&Enemy, &Collidable)>().iter() {
            if let Some(space_object) = collidable.space_object.as_ref() {
                if let SpaceObjectId::Enemy = space_object.id {
                    for object in space.collisions(space_object, true) {
                        match object.id {
                            SpaceObjectId::Player => {
                                // println!("Enemy with player collision");

                                // self.player.state.write().unwrap().consume_item(item);
                                // Events::write(Event::KillItem { id: item_id });
                                // let _ = Audio::write().write().unwrap().play("collect");
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        for (id, (_, collidable)) in world.query::<(&Spell, &Collidable)>().iter() {
            if let Some(space_object) = collidable.space_object.as_ref() {
                if let SpaceObjectId::Spell = space_object.id {
                    for object in space.collisions(space_object, true) {
                        if let Some(entity) = object.entity {
                            match object.id {
                                SpaceObjectId::Enemy => {
                                    Events::write(Event::KillEntity { entity });
                                    Events::write(Event::KillEntity { entity: id });
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
}