use super::spell_tag::SpellTag;
use std::collections::{HashMap, HashSet};

pub struct WordToSpellTagDatabase {
    pub records: HashMap<String, HashSet<SpellTag>>,
}
