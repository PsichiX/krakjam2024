use crate::game::utils::magic::spell_tag::SpellTag;
use std::collections::HashSet;

pub struct Spell {
    pub description: HashSet<SpellTag>,
}