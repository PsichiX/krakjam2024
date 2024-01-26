use super::spell_tag::SpellTag;
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

    pub fn parse(&self, text: &str) -> HashSet<SpellTag> {
        text.split_whitespace()
            .map(|word| {
                self.records
                    .get(word)
                    .cloned()
                    .unwrap_or_else(|| hash_set![SpellTag::random()])
            })
            .flatten()
            .collect()
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
            .into_iter()
            .collect::<Vec<_>>();
        tags.sort();
        assert_eq!(
            tags,
            vec![
                SpellTag::Size(SpellTagSize::Large),
                SpellTag::Shape(SpellTagShape::Circle),
                SpellTag::Effect(SpellTagEffect::Fire),
            ]
        );

        let mut tags = database.parse("meteor").into_iter().collect::<Vec<_>>();
        tags.sort();
        assert_eq!(
            tags,
            vec![
                SpellTag::Size(SpellTagSize::Large),
                SpellTag::Shape(SpellTagShape::Circle),
                SpellTag::Effect(SpellTagEffect::Fire),
            ]
        );

        let mut tags = database.parse("fire").into_iter().collect::<Vec<_>>();
        tags.sort();
        assert_eq!(tags, vec![SpellTag::Effect(SpellTagEffect::Fire),]);
    }
}
