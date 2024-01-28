use crate::game::components::{enemy::Enemy, immobility::Immobility, player::Player, speed::Speed};
use hecs::World;
use micro_games_kit::third_party::vek::{Transform, Vec2};

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
            for (_, (enemy, speed, transform, immobility)) in world
                .query::<(
                    &Enemy,
                    &mut Speed,
                    &mut Transform<f32, f32, f32>,
                    Option<&Immobility>,
                )>()
                .iter()
            {
                let direction = (player_position - transform.position.xy())
                    .try_normalized()
                    .unwrap_or_default();

                let mut velocity = direction * speed.value * delta_time;

                if let Some(immobility) = immobility {
                    if immobility.time_left > 0.0 {
                        println!("Enemy immobility: {}", immobility.time_left);
                        velocity = Vec2::<f32>::zero();
                    }
                }

                transform.position += velocity;
                speed.value =
                    (speed.value + enemy.acceleration * delta_time).min(enemy.speed_limit);
            }
        }
    }
}
