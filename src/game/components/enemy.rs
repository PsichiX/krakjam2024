#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Enemy {
    pub acceleration: f32,
    pub speed_limit: f32,
    pub jump_phase: f32,
}
