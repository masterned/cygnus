use std::{fmt, ops};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl Ability {
    pub fn all() -> Vec<Ability> {
        vec![
            Ability::Strength,
            Ability::Dexterity,
            Ability::Constitution,
            Ability::Intelligence,
            Ability::Wisdom,
            Ability::Charisma,
        ]
    }

    pub fn abbr(&self) -> &str {
        match self {
            Ability::Strength => "Str",
            Ability::Dexterity => "Dex",
            Ability::Constitution => "Con",
            Ability::Intelligence => "Int",
            Ability::Wisdom => "Wis",
            Ability::Charisma => "Cha",
        }
    }
}

impl fmt::Display for Ability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Ability::Strength => "Strength",
                Ability::Dexterity => "Dexterity",
                Ability::Constitution => "Constitution",
                Ability::Intelligence => "Intelligence",
                Ability::Wisdom => "Wisdom",
                Ability::Charisma => "Charisma",
            }
        )
    }
}

#[derive(Debug, Default)]
pub struct AbilitiesTemplate {
    pub strength: Option<usize>,
    pub dexterity: Option<usize>,
    pub constitution: Option<usize>,
    pub intelligence: Option<usize>,
    pub wisdom: Option<usize>,
    pub charisma: Option<usize>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Abilities {
    strength: Option<usize>,
    dexterity: Option<usize>,
    constitution: Option<usize>,
    intelligence: Option<usize>,
    wisdom: Option<usize>,
    charisma: Option<usize>,
}

impl Abilities {
    #[must_use]
    pub fn get_score(&self, ability: Ability) -> Option<usize> {
        match ability {
            Ability::Strength => self.strength,
            Ability::Dexterity => self.dexterity,
            Ability::Constitution => self.constitution,
            Ability::Intelligence => self.intelligence,
            Ability::Wisdom => self.wisdom,
            Ability::Charisma => self.charisma,
        }
    }

    pub fn set_score(&mut self, ability: Ability, score: Option<usize>) {
        match ability {
            Ability::Strength => self.strength = score,
            Ability::Dexterity => self.dexterity = score,
            Ability::Constitution => self.constitution = score,
            Ability::Intelligence => self.intelligence = score,
            Ability::Wisdom => self.wisdom = score,
            Ability::Charisma => self.charisma = score,
        }
    }

    #[must_use]
    pub fn get_modifier(&self, ability: Ability) -> Option<isize> {
        self.get_score(ability)
            .map(|ability_score| ability_score as isize / 2 - 5)
    }
}

impl From<AbilitiesTemplate> for Abilities {
    fn from(value: AbilitiesTemplate) -> Self {
        Abilities {
            strength: value.strength,
            dexterity: value.dexterity,
            constitution: value.constitution,
            intelligence: value.intelligence,
            wisdom: value.wisdom,
            charisma: value.charisma,
        }
    }
}

fn sum_options<T: ops::Add<Output = T>>(o1: Option<T>, o2: Option<T>) -> Option<T> {
    match (o1, o2) {
        (Some(v1), Some(v2)) => Some(v1 + v2),
        (Some(v1), None) => Some(v1),
        (None, Some(v2)) => Some(v2),
        _ => None,
    }
}

impl ops::Add<Abilities> for Abilities {
    type Output = Abilities;

    fn add(self, rhs: Abilities) -> Self::Output {
        Abilities {
            strength: sum_options(self.strength, rhs.strength),
            dexterity: sum_options(self.dexterity, rhs.dexterity),
            constitution: sum_options(self.constitution, rhs.constitution),
            intelligence: sum_options(self.intelligence, rhs.intelligence),
            wisdom: sum_options(self.wisdom, rhs.wisdom),
            charisma: sum_options(self.charisma, rhs.charisma),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _ability_score_of_10_should_have_modifier_of_0() {
        let abilities = Abilities {
            strength: Some(10),
            ..Abilities::default()
        };

        assert_eq!(abilities.get_modifier(Ability::Strength), Some(0));
    }

    #[test]
    fn _ability_scores_less_than_10_should_have_negative_modifier() {
        let abilities = Abilities {
            strength: Some(8),
            dexterity: Some(6),
            constitution: Some(4),
            intelligence: Some(2),
            wisdom: Some(0),
            ..Abilities::default()
        };

        assert_eq!(abilities.get_modifier(Ability::Strength), Some(-1));
        assert_eq!(abilities.get_modifier(Ability::Dexterity), Some(-2));
        assert_eq!(abilities.get_modifier(Ability::Constitution), Some(-3));
        assert_eq!(abilities.get_modifier(Ability::Intelligence), Some(-4));
        assert_eq!(abilities.get_modifier(Ability::Wisdom), Some(-5));
    }

    #[test]
    fn _ability_scores_greater_than_10_should_have_positive_modifiers() {
        let abilities = Abilities {
            strength: Some(12),
            dexterity: Some(14),
            constitution: Some(16),
            intelligence: Some(18),
            wisdom: Some(20),
            ..Abilities::default()
        };

        assert_eq!(abilities.get_modifier(Ability::Strength), Some(1));
        assert_eq!(abilities.get_modifier(Ability::Dexterity), Some(2));
        assert_eq!(abilities.get_modifier(Ability::Constitution), Some(3));
        assert_eq!(abilities.get_modifier(Ability::Intelligence), Some(4));
        assert_eq!(abilities.get_modifier(Ability::Wisdom), Some(5));
    }
}
