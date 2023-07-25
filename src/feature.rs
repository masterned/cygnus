use std::collections::HashSet;

use crate::spell::Spell;

pub enum Feature {
    Spellcasting(Vec<HashSet<Spell>>),
}
