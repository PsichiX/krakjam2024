use crate::game::{
    components::{collidable::Collidable, ignore_entity::IgnoreEntity, projectile::Projectile},
    utils::space::Space,
};
use hecs::{Entity, World};

pub struct ProjectileController;

impl ProjectileController {
    pub fn run(world: &mut World, delta_time: f32) {
        let mut to_despawn = Vec::<Entity>::new();

        for (entity, (projectile,)) in world.query::<(&mut Projectile,)>().iter() {
            projectile.alive_time += delta_time;
            projectile.ttl -= delta_time;

            if projectile.ttl <= 0.0 {
                to_despawn.push(entity);
            }
        }

        for entity in to_despawn {
            let _ = world.despawn(entity);
        }

        let space = Space::read();
        let space = space.read().unwrap();
        let mut entities_to_remove = Vec::<(Entity, Entity)>::new();

        for (projectile_entity, (collidable, _)) in
            world.query::<(&Collidable, &mut Projectile)>().iter()
        {
            if let Some(space_object) = collidable.space_object.as_ref() {
                for object in space.collisions(space_object, true) {
                    if let Some(entity_b) = object.entity {
                        if entity_b != projectile_entity {
                            if IgnoreEntity::should_be_ignored(world, projectile_entity, entity_b) {
                                continue;
                            }

                            entities_to_remove.push((projectile_entity, entity_b));
                        }
                    }
                }
            }
        }

        for (projectile_entity, entity_b) in entities_to_remove {
            if world.get::<&Projectile>(entity_b).is_ok() {
                continue;
            }

            if let Ok(projectile) = world.get::<&mut Projectile>(projectile_entity).as_mut() {
                projectile.ttl -= 0.2;
            }
        }
    }
}
