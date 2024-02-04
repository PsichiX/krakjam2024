use micro_games_kit::third_party::vek::Vec2;

pub struct Movement {
    pub velocity: Vec2<f32>,
    pub acceleration: Vec2<f32>,
    pub static_friction: f32,
    pub speed_limit: Option<f32>,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            velocity: Default::default(),
            acceleration: Default::default(),
            static_friction: 1.0,
            speed_limit: None,
        }
    }
}

impl Movement {
    pub fn new(velocity: Vec2<f32>, acceleration: Vec2<f32>, static_friction: f32) -> Self {
        Self {
            velocity,
            acceleration,
            static_friction,
            ..Default::default()
        }
    }

    pub fn apply_force(&mut self, force: Vec2<f32>) {
        self.acceleration += force;
    }
}
