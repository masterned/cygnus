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

    pub fn calculate_modifier(ability_score: usize) -> isize {
        ability_score as isize / 2 - 5
    }
}

pub struct Abilities(HashMap<Ability, usize>);

impl Abilities {
    pub fn get_base_score(&self, ability: &Ability) -> usize {
        *self.0.get(&ability).unwrap_or(&0)
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
        assert_eq!(Ability::calculate_modifier(10), 0);
    }

    #[test]
    fn _ability_scores_less_than_10_should_have_negative_modifier() {
        assert_eq!(Ability::calculate_modifier(8), -1);
        assert_eq!(Ability::calculate_modifier(6), -2);
        assert_eq!(Ability::calculate_modifier(4), -3);
        assert_eq!(Ability::calculate_modifier(2), -4);
        assert_eq!(Ability::calculate_modifier(0), -5);
    }

    #[test]
    fn _ability_scores_greater_than_10_should_have_positive_modifiers() {
        assert_eq!(Ability::calculate_modifier(12), 1);
        assert_eq!(Ability::calculate_modifier(14), 2);
        assert_eq!(Ability::calculate_modifier(16), 3);
        assert_eq!(Ability::calculate_modifier(18), 4);
        assert_eq!(Ability::calculate_modifier(20), 5);
    }
}
