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
