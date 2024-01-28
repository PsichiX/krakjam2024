use micro_games_kit::third_party::rand::{thread_rng, Rng};
use micro_games_kit::third_party::vek::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Enemy {
    pub acceleration: f32,
    pub speed_limit: f32,
    pub jump_phase: f32,
    pub shoot_cooldown: f32,
    pub direction: Vec2<f32>,
    pub direction_rotation: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            acceleration: 50.0,
            speed_limit: 150.0,
            jump_phase: thread_rng().gen_range(0.0..1.0),
            shoot_cooldown: 10.0,
            direction: Vec2::<f32>::zero(),
            direction_rotation: 0.0,
        }
    }
}
