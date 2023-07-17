use std::collections::HashMap;

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
    pub fn all() -> Vec<Self> {
        vec![
            Ability::Strength,
            Ability::Dexterity,
            Ability::Constitution,
            Ability::Intelligence,
            Ability::Wisdom,
            Ability::Charisma,
        ]
    }
}

pub struct Abilities(HashMap<Ability, usize>);

impl Abilities {
    pub fn get_score(&self, ability: Ability) -> usize {
        *self.0.get(&ability).unwrap_or(&0)
    }

    pub fn get_modifier(&self, ability: Ability) -> isize {
        self.get_score(ability) as isize / 2 - 5
    }
}

impl Default for Abilities {
    fn default() -> Self {
        Abilities(HashMap::from_iter(
            Ability::all().iter().map(|&ability| (ability, 8)),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _ability_score_of_10_should_have_modifier_of_0() {
        let abilities = Abilities(HashMap::from([(Ability::Strength, 10)]));

        assert_eq!(abilities.get_modifier(Ability::Strength), 0);
    }

    #[test]
    fn _ability_scores_less_than_10_should_have_negative_modifier() {
        let abilities = Abilities(HashMap::from([
            (Ability::Strength, 8),
            (Ability::Dexterity, 6),
            (Ability::Constitution, 4),
            (Ability::Intelligence, 2),
            (Ability::Wisdom, 0),
        ]));

        assert_eq!(abilities.get_modifier(Ability::Strength), -1);
        assert_eq!(abilities.get_modifier(Ability::Dexterity), -2);
        assert_eq!(abilities.get_modifier(Ability::Constitution), -3);
        assert_eq!(abilities.get_modifier(Ability::Intelligence), -4);
        assert_eq!(abilities.get_modifier(Ability::Wisdom), -5);
    }

    #[test]
    fn _ability_scores_greater_than_10_should_have_positive_modifiers() {
        let abilities = Abilities(HashMap::from([
            (Ability::Strength, 12),
            (Ability::Dexterity, 14),
            (Ability::Constitution, 16),
            (Ability::Intelligence, 18),
            (Ability::Wisdom, 20),
        ]));

        assert_eq!(abilities.get_modifier(Ability::Strength), 1);
        assert_eq!(abilities.get_modifier(Ability::Dexterity), 2);
        assert_eq!(abilities.get_modifier(Ability::Constitution), 3);
        assert_eq!(abilities.get_modifier(Ability::Intelligence), 4);
        assert_eq!(abilities.get_modifier(Ability::Wisdom), 5);
    }
}
