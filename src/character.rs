use crate::{
    ability::{Abilities, Ability},
    class::Class,
    modifiers::Proficiency,
};

pub struct Character {
    abilities: Abilities,
    classes: Vec<Box<dyn Class>>,
}

impl Character {
    pub fn get_ability_score(&self, ability: Ability) -> usize {
        self.abilities.get_score(ability)
    }

    pub fn get_ability_modifier(&self, ability: Ability) -> isize {
        self.abilities.get_modifier(ability)
    }

    pub fn get_level(&self) -> usize {
        self.classes
            .iter()
            .fold(0, |acc, class| acc + class.get_level())
    }

    pub fn get_proficiency_bonus(&self) -> usize {
        (self.get_level() - 1) / 4 + 2
    }

    pub fn get_saving_throw_proficiency(&self, ability: Ability) -> Option<Proficiency> {
        self.classes
            .first()
            .and_then(|primary_class| primary_class.get_saving_throw_proficiency(ability))
    }

    pub fn get_saving_throw_mod(&self, ability: Ability) -> isize {
        self.get_proficiency_bonus() as isize
            * self
                .get_saving_throw_proficiency(ability)
                .map(|p| p as isize)
                .unwrap_or(0)
            + self.abilities.get_modifier(ability)
    }
}

#[cfg(test)]
mod tests {
    use crate::class::{Artificer, Wizard};

    use super::*;

    #[test]
    fn _characters_with_no_classes_should_be_level_0() {
        let character = Character {
            abilities: Abilities::default(),
            classes: vec![],
        };

        assert_eq!(character.get_level(), 0);
    }

    #[test]
    fn _characters_with_only_one_class_should_be_the_classs_level() {
        let character = Character {
            abilities: Abilities::default(),
            classes: vec![Box::new(Artificer { level: 1 })],
        };

        assert_eq!(character.get_level(), 1);

        let character = Character {
            abilities: Abilities::default(),
            classes: vec![Box::new(Artificer { level: 20 })],
        };

        assert_eq!(character.get_level(), 20);
    }

    #[test]
    fn _multiclass_characters_should_sum_classes_levels() {
        let character = Character {
            abilities: Abilities::default(),
            classes: vec![
                Box::new(Artificer { level: 1 }),
                Box::new(Wizard { level: 1 }),
            ],
        };

        assert_eq!(character.get_level(), 2);
    }

    #[test]
    fn _level_1_character_should_have_proficiency_bonus_of_2() {
        let character = Character {
            abilities: Abilities::default(),
            classes: vec![Box::new(Artificer { level: 1 })],
        };

        assert_eq!(character.get_proficiency_bonus(), 2);
    }

    #[test]
    fn _proficiency_bonus_sholud_go_up_by_1_every_4_level_ups() {
        let lvl4 = Character {
            abilities: Abilities::default(),
            classes: vec![Box::new(Artificer { level: 4 })],
        };

        assert_eq!(lvl4.get_proficiency_bonus(), 2);

        let lvl5 = Character {
            abilities: Abilities::default(),
            classes: vec![Box::new(Artificer { level: 5 })],
        };

        assert_eq!(lvl5.get_proficiency_bonus(), 3);

        let lvl9 = Character {
            abilities: Abilities::default(),
            classes: vec![Box::new(Artificer { level: 9 })],
        };

        assert_eq!(lvl9.get_proficiency_bonus(), 4);

        let lvl13 = Character {
            abilities: Abilities::default(),
            classes: vec![Box::new(Artificer { level: 13 })],
        };

        assert_eq!(lvl13.get_proficiency_bonus(), 5);

        let lvl17 = Character {
            abilities: Abilities::default(),
            classes: vec![Box::new(Artificer { level: 17 })],
        };

        assert_eq!(lvl17.get_proficiency_bonus(), 6);
    }

    #[test]
    fn _characters_without_a_class_should_have_no_saving_throw_proficiencies() {
        let character = Character {
            abilities: Abilities::default(),
            classes: vec![],
        };

        assert_eq!(
            character.get_saving_throw_proficiency(Ability::Strength),
            None
        );
        assert_eq!(
            character.get_saving_throw_proficiency(Ability::Dexterity),
            None
        );
        assert_eq!(
            character.get_saving_throw_proficiency(Ability::Constitution),
            None
        );
        assert_eq!(
            character.get_saving_throw_proficiency(Ability::Intelligence),
            None
        );
        assert_eq!(
            character.get_saving_throw_proficiency(Ability::Wisdom),
            None
        );
        assert_eq!(
            character.get_saving_throw_proficiency(Ability::Charisma),
            None
        );
    }

    #[test]
    fn _characters_with_1_class_should_derive_their_saving_throw_proficiencies_from_it() {
        let character = Character {
            abilities: Abilities::default(),
            classes: vec![Box::new(Artificer { level: 1 })],
        };

        assert_eq!(
            character.get_saving_throw_proficiency(Ability::Constitution),
            Some(Proficiency::Proficiency)
        );
        assert_eq!(
            character.get_saving_throw_proficiency(Ability::Intelligence),
            Some(Proficiency::Proficiency)
        );
    }

    #[test]
    fn _multiclass_characters_should_only_inherit_proficiencies_from_first_class() {
        let multiclass_character = Character {
            abilities: Abilities::default(),
            classes: vec![
                Box::new(Wizard { level: 1 }),
                Box::new(Artificer { level: 1 }),
            ],
        };

        assert_eq!(
            multiclass_character.get_saving_throw_proficiency(Ability::Wisdom),
            Some(Proficiency::Proficiency)
        );
        assert_eq!(
            multiclass_character.get_saving_throw_proficiency(Ability::Constitution),
            None
        );
    }

    #[test]
    fn _should_get_saving_throw_mod_without_proficiency() {
        let character = Character {
            abilities: Abilities::default(),
            classes: vec![Box::new(Artificer { level: 1 })],
        };

        assert_eq!(character.get_saving_throw_mod(Ability::Strength), -1);
    }

    #[test]
    fn _should_get_saving_throw_mod_including_proficiency_bonus() {
        let character = Character {
            abilities: Abilities::default(),
            classes: vec![Box::new(Artificer { level: 1 })],
        };

        assert_eq!(character.get_saving_throw_mod(Ability::Constitution), 1);
    }
}
