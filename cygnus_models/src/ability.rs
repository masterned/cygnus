use std::{collections::HashMap, fmt, ops};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Identifier {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl Identifier {
    pub fn all() -> Vec<Identifier> {
        vec![
            Identifier::Strength,
            Identifier::Dexterity,
            Identifier::Constitution,
            Identifier::Intelligence,
            Identifier::Wisdom,
            Identifier::Charisma,
        ]
    }

    pub fn abbr(&self) -> &str {
        match self {
            Identifier::Strength => "Str",
            Identifier::Dexterity => "Dex",
            Identifier::Constitution => "Con",
            Identifier::Intelligence => "Int",
            Identifier::Wisdom => "Wis",
            Identifier::Charisma => "Cha",
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Identifier::Strength => "Strength",
                Identifier::Dexterity => "Dexterity",
                Identifier::Constitution => "Constitution",
                Identifier::Intelligence => "Intelligence",
                Identifier::Wisdom => "Wisdom",
                Identifier::Charisma => "Charisma",
            }
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ability {
    score: usize,
}

impl Ability {
    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn get_modifier(&self) -> isize {
        self.score as isize / 2 - 5
    }
}

impl From<usize> for Ability {
    fn from(value: usize) -> Self {
        Ability { score: value }
    }
}

impl ops::Add<Ability> for Ability {
    type Output = Ability;

    fn add(self, rhs: Ability) -> Self::Output {
        Ability {
            score: self.score + rhs.score,
        }
    }
}

impl ops::AddAssign<Ability> for Ability {
    fn add_assign(&mut self, rhs: Ability) {
        self.score += rhs.score;
    }
}

#[derive(Debug, Default)]
pub struct AbilitiesTemplate {
    pub strength: usize,
    pub dexterity: usize,
    pub constitution: usize,
    pub intelligence: usize,
    pub wisdom: usize,
    pub charisma: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Abilities(HashMap<Identifier, Ability>);

impl Abilities {
    pub fn set_score(&mut self, ability: Identifier, score: usize) {
        self.0.insert(ability, Ability { score });
    }

    #[must_use]
    pub fn get_score(&self, ability: Identifier) -> Option<usize> {
        self.0.get(&ability).map(|ability| ability.get_score())
    }

    #[must_use]
    pub fn get_modifier(&self, ability: Identifier) -> Option<isize> {
        self.0.get(&ability).map(|ability| ability.get_modifier())
    }

    #[must_use]
    pub fn count_abilities(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn get_abilities(&self) -> Vec<(&Identifier, &Ability)> {
        self.0.keys().zip(self.0.values()).collect()
    }
}

impl From<AbilitiesTemplate> for Abilities {
    fn from(value: AbilitiesTemplate) -> Self {
        Self(HashMap::from([
            (Identifier::Strength, value.strength.into()),
            (Identifier::Dexterity, value.dexterity.into()),
            (Identifier::Constitution, value.constitution.into()),
            (Identifier::Intelligence, value.intelligence.into()),
            (Identifier::Wisdom, value.wisdom.into()),
            (Identifier::Charisma, value.charisma.into()),
        ]))
    }
}

impl ops::Add<Abilities> for Abilities {
    type Output = Abilities;

    fn add(self, rhs: Abilities) -> Self::Output {
        Abilities(self.0.iter().chain(rhs.0.iter()).fold(
            HashMap::new(),
            |mut acc, (&id, &new_ability)| {
                acc.entry(id)
                    .and_modify(|found_ability| *found_ability += new_ability)
                    .or_insert(new_ability);

                acc
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _ability_score_of_10_should_have_modifier_of_0() {
        let strength = Ability { score: 10 };

        assert_eq!(strength.get_modifier(), 0);
    }

    #[test]
    fn _ability_scores_less_than_10_should_have_negative_modifier() {
        let strength = Ability { score: 9 };
        let dexterity = Ability { score: 7 };
        let constitution = Ability { score: 5 };
        let intelligence = Ability { score: 3 };
        let wisdom = Ability { score: 1 };

        assert_eq!(strength.get_modifier(), -1);
        assert_eq!(dexterity.get_modifier(), -2);
        assert_eq!(constitution.get_modifier(), -3);
        assert_eq!(intelligence.get_modifier(), -4);
        assert_eq!(wisdom.get_modifier(), -5);
    }

    #[test]
    fn _ability_scores_greater_than_11_should_have_positive_modifiers() {
        let strength = Ability { score: 12 };
        let dexterity = Ability { score: 14 };
        let constitution = Ability { score: 16 };
        let intelligence = Ability { score: 18 };
        let wisdom = Ability { score: 20 };

        assert_eq!(strength.get_modifier(), 1);
        assert_eq!(dexterity.get_modifier(), 2);
        assert_eq!(constitution.get_modifier(), 3);
        assert_eq!(intelligence.get_modifier(), 4);
        assert_eq!(wisdom.get_modifier(), 5);
    }
}
