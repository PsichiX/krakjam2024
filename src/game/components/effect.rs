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

#[derive(Debug, Default, Clone, Copy)]
pub struct Effect {
    pub fire: bool,
    pub water: bool,
    pub electricity: bool,
}

impl Effect {
    pub fn reaction(self, other: Self) -> EffectReaction {
        let fire = self.fire || other.fire;
        let water = self.water || other.water;
        let electricity = self.electricity || other.electricity;
        if fire && water && electricity {
            EffectReaction::None
        } else if fire && water {
            EffectReaction::Steam
        } else if water && electricity {
            EffectReaction::Paralize
        } else if electricity && fire {
            EffectReaction::Explode
        } else {
            EffectReaction::None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effect_reactions() {
        let a = Effect {
            fire: true,
            ..Default::default()
        };
        let b = Effect {
            fire: true,
            ..Default::default()
        };
        assert_eq!(a.reaction(b), EffectReaction::None);

        let b = Effect {
            water: true,
            ..Default::default()
        };
        assert_eq!(a.reaction(b), EffectReaction::Steam);

        let b = Effect {
            electricity: true,
            ..Default::default()
        };
        assert_eq!(a.reaction(b), EffectReaction::Explode);

        let a = Effect {
            water: true,
            ..Default::default()
        };
        assert_eq!(a.reaction(b), EffectReaction::Paralize);

        let a = Effect {
            fire: true,
            water: true,
            ..Default::default()
        };
        let b = Effect {
            electricity: true,
            water: true,
            ..Default::default()
        };
        assert_eq!(a.reaction(b), EffectReaction::None);
    }
}
