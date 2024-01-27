use crate::game::components::{enemy::Enemy, player::Player, speed::Speed};
use hecs::World;
use micro_games_kit::third_party::vek::Transform;

pub struct EnemyController;

impl EnemyController {
    pub fn run(world: &World, delta_time: f32) {
        if let Some(player_position) = world
            .query::<&Transform<f32, f32, f32>>()
            .with::<&Player>()
            .iter()
            .next()
            .map(|(_, transform)| transform.position.xy())
        {
            for (_, (speed, transform)) in world
                .query::<(&Speed, &mut Transform<f32, f32, f32>)>()
                .with::<&Enemy>()
                .iter()
            {
                let direction = (player_position - transform.position.xy())
                    .try_normalized()
                    .unwrap_or_default();
                transform.position += direction * speed.value * delta_time;
            }
        }
    }
}
