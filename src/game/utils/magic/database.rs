use micro_games_kit::third_party::kira::track::effect;

use super::spell_tag::{SpellTag, SpellTagSize};
use crate::{game::components::spell::Spell, hash_set};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct WordToSpellTagDatabase {
    pub records: HashMap<String, HashSet<SpellTag>>,
}

impl WordToSpellTagDatabase {
    pub fn with(mut self, word: impl ToString, tag: SpellTag) -> Self {
        self.records
            .entry(word.to_string())
            .or_default()
            .insert(tag);
        self
    }

    pub fn parse(&self, text: &str) -> Option<Spell> {
        // convert words to tags (or generate random ones in their place).
        let tags = text
            .split_whitespace()
            .map(|word| {
                self.records
                    .get(word)
                    .cloned()
                    .unwrap_or_else(|| hash_set![SpellTag::random()])
            })
            .flatten()
            .collect::<HashSet<_>>();
        // construct structured spell tags with required categories or use defaults.
        if let Some(effect) = tags.iter().find_map(|tag| tag.as_effect()) {
            let size = tags
                .iter()
                .find_map(|tag| tag.as_size())
                .unwrap_or_else(|| Default::default());
            let speed = tags
                .iter()
                .find_map(|tag| tag.as_speed())
                .unwrap_or_else(|| Default::default());
            let shape = tags
                .iter()
                .find_map(|tag| tag.as_shape())
                .unwrap_or_else(|| Default::default());
            let direction = tags
                .iter()
                .find_map(|tag| tag.as_direction())
                .unwrap_or_else(|| Default::default());
            let trajectory = tags
                .iter()
                .find_map(|tag| tag.as_trajectory())
                .unwrap_or_else(|| Default::default());
            Some(Spell {
                size,
                speed,
                effect,
                shape,
                direction,
                trajectory,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WordToSpellTagDatabase;
    use crate::game::{
        components::spell::{self, Spell},
        utils::magic::spell_tag::{SpellTag, SpellTagEffect, SpellTagShape, SpellTagSize},
    };

    #[test]
    fn test_word_to_spell_tag_database() {
        let database = WordToSpellTagDatabase::default()
            .with("fire", SpellTag::Effect(SpellTagEffect::Fire))
            .with("big", SpellTag::Size(SpellTagSize::Large))
            .with("ball", SpellTag::Shape(SpellTagShape::Point))
            .with("meteor", SpellTag::Effect(SpellTagEffect::Fire))
            .with("meteor", SpellTag::Size(SpellTagSize::Large))
            .with("meteor", SpellTag::Shape(SpellTagShape::Point));

        let spell = database.parse("big fire ball").unwrap();
        assert_eq!(
            spell,
            Spell {
                size: SpellTagSize::Large,
                speed: Default::default(),
                effect: SpellTagEffect::Fire,
                shape: SpellTagShape::Point,
                direction: Default::default(),
                trajectory: Default::default(),
            }
        );

        let spell = database.parse("meteor").unwrap();
        assert_eq!(
            spell,
            Spell {
                size: SpellTagSize::Large,
                speed: Default::default(),
                effect: SpellTagEffect::Fire,
                shape: SpellTagShape::Point,
                direction: Default::default(),
                trajectory: Default::default(),
            }
        );

        let spell = database.parse("fire").unwrap();
        assert_eq!(
            spell,
            Spell {
                size: Default::default(),
                speed: Default::default(),
                effect: SpellTagEffect::Fire,
                shape: Default::default(),
                direction: Default::default(),
                trajectory: Default::default(),
            }
        );
    }
}
