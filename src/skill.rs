use std::collections::HashMap;

use crate::{ability::Ability, modifiers::Proficiency};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Skill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SlightOfHand,
    Stealth,
    Survival,
}

impl Skill {
    #[must_use]
    pub fn get_ability(&self) -> Ability {
        match self {
            Skill::Acrobatics | Skill::SlightOfHand | Skill::Stealth => Ability::Dexterity,
            Skill::AnimalHandling
            | Skill::Insight
            | Skill::Medicine
            | Skill::Perception
            | Skill::Survival => Ability::Wisdom,
            Skill::Arcana
            | Skill::History
            | Skill::Investigation
            | Skill::Nature
            | Skill::Religion => Ability::Intelligence,
            Skill::Athletics => Ability::Strength,
            Skill::Deception | Skill::Intimidation | Skill::Performance | Skill::Persuasion => {
                Ability::Charisma
            }
        }
    }
}

pub struct Skills(HashMap<Skill, Option<Proficiency>>);

impl Skills {
    #[must_use]
    pub fn get_proficiency(&self, skill: &Skill) -> Option<Proficiency> {
        self.0.get(skill).and_then(|&proficiency| proficiency)
    }

    pub fn set_proficiency(&mut self, skill: Skill, proficiency: Option<Proficiency>) {
        self.0.insert(skill, proficiency);
    }
}

impl Default for Skills {
    fn default() -> Self {
        Skills(HashMap::from([
            (Skill::Acrobatics, None),
            (Skill::AnimalHandling, None),
            (Skill::Arcana, None),
            (Skill::Athletics, None),
            (Skill::Deception, None),
            (Skill::History, None),
            (Skill::Insight, None),
            (Skill::Intimidation, None),
            (Skill::Investigation, None),
            (Skill::Medicine, None),
            (Skill::Nature, None),
            (Skill::Perception, None),
            (Skill::Performance, None),
            (Skill::Persuasion, None),
            (Skill::Religion, None),
            (Skill::SlightOfHand, None),
            (Skill::Stealth, None),
            (Skill::Survival, None),
        ]))
    }
}
