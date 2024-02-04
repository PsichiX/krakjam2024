use crate::game::components::movement::Movement;
use hecs::World;
use micro_games_kit::{context::GameContext, third_party::vek::Transform};

pub struct MovementController;

impl MovementController {
    pub fn run(world: &World, delta_time: f32) {
        for (_, (transform, movement)) in world
            .query::<(&mut Transform<f32, f32, f32>, &mut Movement)>()
            .iter()
        {
            movement.velocity += movement.acceleration * delta_time;

            movement.acceleration = Default::default();

            if let Some(speed_limit) = movement.speed_limit {
                if movement.velocity.magnitude() > speed_limit {
                    movement.velocity = movement.velocity.normalized() * speed_limit;
                }
            }

            transform.position += movement.velocity * delta_time;

            movement.velocity *= 1.0 - movement.static_friction;
        }
    }
}
