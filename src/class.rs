use crate::{ability::Ability, modifiers::Proficiency};

pub trait Class {
    fn get_level(&self) -> usize;

    fn get_saving_throw_proficiency(&self, ability: Ability) -> Option<Proficiency>;
}

pub struct Artificer {
    pub level: usize,
}

impl Class for Artificer {
    fn get_level(&self) -> usize {
        self.level
    }

    fn get_saving_throw_proficiency(&self, ability: Ability) -> Option<Proficiency> {
        match ability {
            Ability::Constitution | Ability::Intelligence => Some(Proficiency::Proficiency),
            _ => None,
        }
    }
}

pub struct Wizard {
    pub level: usize,
}

impl Class for Wizard {
    fn get_level(&self) -> usize {
        self.level
    }

    fn get_saving_throw_proficiency(&self, ability: Ability) -> Option<Proficiency> {
        match ability {
            Ability::Intelligence | Ability::Wisdom => Some(Proficiency::Proficiency),
            _ => None,
        }
    }
}
