use crate::game::utils::magic::spell_tag::SpellTagEffect;
use micro_games_kit::third_party::vek::Rgba;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EffectReaction {
    // pair of same effects or all effects combined
    #[default]
    None,
    // fire + water
    Steam,
    // water + electricity
    Paralize,
    // electricity + fire
    Explode,
}

impl EffectReaction {
    pub fn damage(&self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::Steam => 10.0,
            Self::Paralize => 0.0,
            Self::Explode => 50.0,
        }
    }

    pub fn immobile_time(&self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::Steam => 0.0,
            Self::Paralize => 3.0,
            Self::Explode => 0.0,
        }
    }

    pub fn push_distance(&self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::Steam => 150.0,
            Self::Paralize => 0.0,
            Self::Explode => 10.0,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Effect {
    pub fire: bool,
    pub water: bool,
    pub electricity: bool,
}

impl From<SpellTagEffect> for Effect {
    fn from(value: SpellTagEffect) -> Self {
        return Effect {
            electricity: value == SpellTagEffect::Electric,
            fire: value == SpellTagEffect::Fire,
            water: value == SpellTagEffect::Water,
        };
    }
}

impl Effect {
    pub fn react(&mut self, other: &mut Self) -> EffectReaction {
        let fire = self.fire || other.fire;
        let water = self.water || other.water;
        let electricity = self.electricity || other.electricity;

        self.fire = fire;
        self.water = water;
        self.electricity = electricity;

        other.fire = fire;
        other.water = water;
        other.electricity = electricity;

        if fire && water && electricity {
            EffectReaction::None
        } else if fire && water {
            self.fire = false;
            other.fire = false;
            self.water = false;
            other.water = false;
            EffectReaction::Steam
        } else if water && electricity {
            self.water = false;
            other.water = false;
            self.electricity = false;
            other.electricity = false;
            EffectReaction::Paralize
        } else if electricity && fire {
            self.electricity = false;
            other.electricity = false;
            self.fire = false;
            other.fire = false;
            EffectReaction::Explode
        } else {
            EffectReaction::None
        }
    }

    pub fn slime_tint(&self) -> Rgba<f32> {
        if self.fire {
            Rgba::new(1.0, 0.75, 0.5, 1.0)
        } else if self.water {
            Rgba::new(0.5, 0.5, 1.0, 1.0)
        } else if self.electricity {
            Rgba::new(1.0, 1.0, 0.0, 1.0)
        } else {
            Rgba::new(1.0, 1.0, 1.0, 1.0)
        }
    }

    pub fn to_effect_tag(&self) -> SpellTagEffect {
        if self.electricity {
            return SpellTagEffect::Electric;
        } else if self.fire {
            return SpellTagEffect::Fire;
        } else if self.water {
            return SpellTagEffect::Water;
        }

        return SpellTagEffect::None;
    }

    pub fn empty(&self) -> bool {
        !self.electricity && !self.fire && !self.water
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effect_reactions() {
        let mut a = Effect {
            fire: true,
            ..Default::default()
        };
        let mut b = Effect {
            fire: true,
            ..Default::default()
        };
        assert_eq!(a.react(&mut b), EffectReaction::None);
        assert_eq!(
            a,
            Effect {
                fire: true,
                ..Default::default()
            }
        );
        assert_eq!(
            b,
            Effect {
                fire: true,
                ..Default::default()
            }
        );

        let mut a = Effect {
            fire: true,
            ..Default::default()
        };
        let mut b = Effect {
            water: true,
            ..Default::default()
        };
        assert_eq!(a.react(&mut b), EffectReaction::Steam);
        assert_eq!(
            a,
            Effect {
                ..Default::default()
            }
        );
        assert_eq!(
            b,
            Effect {
                ..Default::default()
            }
        );

        let mut a = Effect {
            fire: true,
            ..Default::default()
        };
        let mut b = Effect {
            electricity: true,
            ..Default::default()
        };
        assert_eq!(a.react(&mut b), EffectReaction::Explode);
        assert_eq!(
            a,
            Effect {
                ..Default::default()
            }
        );
        assert_eq!(
            b,
            Effect {
                ..Default::default()
            }
        );

        let mut a = Effect {
            water: true,
            ..Default::default()
        };
        let mut b = Effect {
            electricity: true,
            ..Default::default()
        };
        assert_eq!(a.react(&mut b), EffectReaction::Paralize);
        assert_eq!(
            a,
            Effect {
                ..Default::default()
            }
        );
        assert_eq!(
            b,
            Effect {
                ..Default::default()
            }
        );

        let mut a = Effect {
            fire: true,
            water: true,
            ..Default::default()
        };
        let mut b = Effect {
            electricity: true,
            water: true,
            ..Default::default()
        };
        assert_eq!(a.react(&mut b), EffectReaction::None);
        assert_eq!(
            a,
            Effect {
                fire: true,
                water: true,
                ..Default::default()
            }
        );
        assert_eq!(
            b,
            Effect {
                electricity: true,
                water: true,
                ..Default::default()
            }
        );
    }
}
