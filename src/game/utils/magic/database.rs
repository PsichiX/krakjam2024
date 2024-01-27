use super::spell_tag::{SpellTag, SpellTagSize};
use crate::hash_set;
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

    pub fn parse(&self, text: &str) -> Option<HashSet<SpellTag>> {
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
        let mut result = HashSet::with_capacity(6);
        result.insert(
            tags.iter()
                .find(|tag| matches!(tag, SpellTag::Size(_)))
                .copied()
                .unwrap_or_else(|| SpellTag::Size(Default::default())),
        );
        result.insert(
            tags.iter()
                .find(|tag| matches!(tag, SpellTag::Speed(_)))
                .copied()
                .unwrap_or_else(|| SpellTag::Speed(Default::default())),
        );
        result.insert(
            tags.iter()
                .find(|tag| matches!(tag, SpellTag::Shape(_)))
                .copied()
                .unwrap_or_else(|| SpellTag::Shape(Default::default())),
        );
        result.insert(
            tags.iter()
                .find(|tag| matches!(tag, SpellTag::Direction(_)))
                .copied()
                .unwrap_or_else(|| SpellTag::Direction(Default::default())),
        );
        result.insert(
            tags.iter()
                .find(|tag| matches!(tag, SpellTag::Trajectory(_)))
                .copied()
                .unwrap_or_else(|| SpellTag::Trajectory(Default::default())),
        );
        if let Some(tag) = tags
            .iter()
            .find(|tag| matches!(tag, SpellTag::Effect(_)))
            .copied()
        {
            result.insert(tag);
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WordToSpellTagDatabase;
    use crate::game::utils::magic::spell_tag::{
        SpellTag, SpellTagEffect, SpellTagShape, SpellTagSize,
    };

    #[test]
    fn test_word_to_spell_tag_database() {
        let database = WordToSpellTagDatabase::default()
            .with("fire", SpellTag::Effect(SpellTagEffect::Fire))
            .with("big", SpellTag::Size(SpellTagSize::Large))
            .with("ball", SpellTag::Shape(SpellTagShape::Circle))
            .with("meteor", SpellTag::Effect(SpellTagEffect::Fire))
            .with("meteor", SpellTag::Size(SpellTagSize::Large))
            .with("meteor", SpellTag::Shape(SpellTagShape::Circle));

        let mut tags = database
            .parse("big fire ball")
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();
        tags.sort();
        assert_eq!(
            tags,
            vec![
                SpellTag::Size(SpellTagSize::Large),
                SpellTag::Speed(Default::default()),
                SpellTag::Effect(SpellTagEffect::Fire),
                SpellTag::Shape(SpellTagShape::Circle),
                SpellTag::Direction(Default::default()),
                SpellTag::Trajectory(Default::default()),
            ]
        );

        let mut tags = database
            .parse("meteor")
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();
        tags.sort();
        assert_eq!(
            tags,
            vec![
                SpellTag::Size(SpellTagSize::Large),
                SpellTag::Speed(Default::default()),
                SpellTag::Effect(SpellTagEffect::Fire),
                SpellTag::Shape(SpellTagShape::Circle),
                SpellTag::Direction(Default::default()),
                SpellTag::Trajectory(Default::default()),
            ]
        );

        let mut tags = database
            .parse("fire")
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();
        tags.sort();
        assert_eq!(
            tags,
            vec![
                SpellTag::Size(Default::default()),
                SpellTag::Speed(Default::default()),
                SpellTag::Effect(SpellTagEffect::Fire),
                SpellTag::Shape(Default::default()),
                SpellTag::Direction(Default::default()),
                SpellTag::Trajectory(Default::default()),
            ]
        );
    }
}
