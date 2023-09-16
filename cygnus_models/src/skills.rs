use std::{collections::HashMap, fmt};

use crate::{ability, modifiers::Proficiency};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Identifier {
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

impl Identifier {
    pub fn all() -> Vec<Identifier> {
        vec![
            Identifier::Acrobatics,
            Identifier::AnimalHandling,
            Identifier::Arcana,
            Identifier::Athletics,
            Identifier::Deception,
            Identifier::History,
            Identifier::Insight,
            Identifier::Intimidation,
            Identifier::Investigation,
            Identifier::Medicine,
            Identifier::Nature,
            Identifier::Perception,
            Identifier::Performance,
            Identifier::Persuasion,
            Identifier::Religion,
            Identifier::SlightOfHand,
            Identifier::Stealth,
            Identifier::Survival,
        ]
    }

    #[must_use]
    pub fn get_default_ability(&self) -> ability::Identifier {
        match self {
            Identifier::Acrobatics | Identifier::SlightOfHand | Identifier::Stealth => {
                ability::Identifier::Dexterity
            }
            Identifier::AnimalHandling
            | Identifier::Insight
            | Identifier::Medicine
            | Identifier::Perception
            | Identifier::Survival => ability::Identifier::Wisdom,
            Identifier::Arcana
            | Identifier::History
            | Identifier::Investigation
            | Identifier::Nature
            | Identifier::Religion => ability::Identifier::Intelligence,
            Identifier::Athletics => ability::Identifier::Strength,
            Identifier::Deception
            | Identifier::Intimidation
            | Identifier::Performance
            | Identifier::Persuasion => ability::Identifier::Charisma,
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Identifier::Acrobatics => "Acrobatics",
                Identifier::AnimalHandling => "Animal Handling",
                Identifier::Arcana => "Arcana",
                Identifier::Athletics => "Athletics",
                Identifier::Deception => "Deception",
                Identifier::History => "History",
                Identifier::Insight => "Insight",
                Identifier::Intimidation => "Intimidation",
                Identifier::Investigation => "Investigation",
                Identifier::Medicine => "Medicine",
                Identifier::Nature => "Nature",
                Identifier::Perception => "Perception",
                Identifier::Performance => "Performance",
                Identifier::Persuasion => "Persuasion",
                Identifier::Religion => "Religion",
                Identifier::SlightOfHand => "Slight of Hand",
                Identifier::Stealth => "Stealth",
                Identifier::Survival => "Survival",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Advantage {
    Advantage,
    Disadvantage,
}

#[derive(Clone, Debug, Default)]
pub struct Skill {
    proficiency: Option<Proficiency>,
    advantage: Option<Advantage>,
}

impl Skill {
    pub fn get_proficiency(&self) -> Option<Proficiency> {
        self.proficiency
    }

    pub fn get_advantage(&self) -> Option<Advantage> {
        self.advantage
    }

    pub fn get_modifier(&self, ability_modifier: isize, proficiency_bonus: usize) -> isize {
        let proficiency_multiplier = self.proficiency.map(|p| p as isize).unwrap_or(0);

        (proficiency_bonus as isize * proficiency_multiplier) + ability_modifier
    }
}

#[derive(Clone, Debug)]
pub struct Skills(HashMap<Identifier, Skill>);

impl Skills {
    #[must_use]
    pub fn get_proficiency(&self, skill: Identifier) -> Option<Proficiency> {
        self.0.get(&skill).and_then(|skill| skill.get_proficiency())
    }

    pub fn set_proficiency(&mut self, skill: Identifier, proficiency: Option<Proficiency>) {
        if let Some(skill) = self.0.get_mut(&skill) {
            skill.proficiency = proficiency;
        }
    }

    pub fn get_modifier(
        &self,
        skill: Identifier,
        ability_modifier: isize,
        proficiency_bonus: usize,
    ) -> isize {
        self.0
            .get(&skill)
            .map(|s| s.get_modifier(ability_modifier, proficiency_bonus))
            .unwrap_or(0)
    }
}

impl Default for Skills {
    fn default() -> Self {
        let hm: HashMap<Identifier, Skill> = Identifier::all()
            .iter()
            .map(|&id| (id, Skill::default()))
            .collect();

        Skills(hm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod skill {
        use super::*;

        #[test]
        fn _should_calculate_mod_with_no_proficiency() {
            let performance = Skill {
                proficiency: None,
                advantage: None,
            };

            assert_eq!(performance.get_modifier(5, 4), 5);
        }

        #[test]
        fn _should_calculate_mod_with_proficiency() {
            let history = Skill {
                proficiency: Some(Proficiency::Proficiency),
                advantage: None,
            };

            assert_eq!(history.get_modifier(-1, 4), 3);
        }

        #[test]
        fn _should_calculate_mod_with_expertise() {
            let performance = Skill {
                proficiency: Some(Proficiency::Expertise),
                advantage: None,
            };

            assert_eq!(performance.get_modifier(5, 4), 13);
        }
    }
}
