use micro_games_kit::third_party::vek::Vec2;

#[derive(Default)]
pub struct Movement {
    pub velocity: Vec2<f32>,
    pub acceleration: Vec2<f32>,
}

impl Movement {
    pub fn new(velocity: Vec2<f32>, acceleration: Vec2<f32>) -> Self {
        Self {
            velocity,
            acceleration,
        }
    }
}
