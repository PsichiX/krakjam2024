#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum DamageLayer {
    None,
    Enemy,
}

impl DamageLayer {
    pub fn allow_damage(&self, other: Self) -> bool {
        *self == DamageLayer::None || other == DamageLayer::None || *self != other
    }
}

impl Default for DamageLayer {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Damage {
    pub value: f32,
    pub layer: DamageLayer,
}
