#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Enemy {
    pub acceleration: f32,
    pub speed_limit: f32,
    pub shoot_cooldown: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            acceleration: 50.0,
            speed_limit: 200.0,
            shoot_cooldown: 10.0,
        }
    }
}
