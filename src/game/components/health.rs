use super::damage::DamageLayer;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Health {
    pub value: f32,
    pub limit: f32,
    pub layer: DamageLayer,
}
