use crate::game::utils::magic::spell_tag::{
    SpellTagDirection, SpellTagDuration, SpellTagEffect, SpellTagShape, SpellTagSize,
    SpellTagSpeed, SpellTagTrajectory,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spell {
    pub size: SpellTagSize,
    pub speed: SpellTagSpeed,
    pub effect: SpellTagEffect,
    pub shape: SpellTagShape,
    pub direction: SpellTagDirection,
    pub trajectory: SpellTagTrajectory,
    pub duration: SpellTagDuration,
}

impl Spell {
    pub fn basic() -> Self {
        Self {
            direction: SpellTagDirection::Forward,
            duration: SpellTagDuration::Instant,
            effect: SpellTagEffect::None,
            shape: SpellTagShape::Point,
            size: SpellTagSize::Medium,
            speed: SpellTagSpeed::Medium,
            trajectory: SpellTagTrajectory::Straight,
        }
    }
}
