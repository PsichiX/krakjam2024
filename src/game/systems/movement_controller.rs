use crate::game::components::movement::Movement;
use hecs::World;
use micro_games_kit::{context::GameContext, third_party::vek::Transform};

pub struct MovementController;

impl MovementController {
    pub fn run(world: &World, context: &mut GameContext, delta_time: f32) {
        for (_, (transform, movement)) in world
            .query::<(&mut Transform<f32, f32, f32>, &mut Movement)>()
            .iter()
        {
            movement.velocity += movement.acceleration * delta_time;
            transform.position += movement.velocity * delta_time;
        }
    }
}
