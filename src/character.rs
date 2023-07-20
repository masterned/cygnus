use crate::{
    ability::{Abilities, Ability},
    class::Class,
    modifiers::Proficiency,
    race::{Race, Size},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Conformity {
    Lawful,
    Neutral,
    Chaotic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Morality {
    Good,
    Neutral,
    Evil,
}

pub type Alignment = (Conformity, Morality);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

pub struct Personality {
    pub personality_traits: Vec<String>,
    pub ideals: Vec<String>,
    pub bonds: Vec<String>,
    pub flaws: Vec<String>,
}

pub struct Character {
    pub name: String,
    pub alignment: Alignment,
    pub gender: Option<Gender>,
    pub personality: Personality,
    race: Box<dyn Race>,
    abilities: Abilities,
    classes: Vec<Box<dyn Class>>,
}

impl Character {
    pub fn get_size(&self) -> Size {
        self.race.get_size()
    }

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
    use crate::{
        class::{Artificer, Wizard},
        race::Human,
    };

    use super::*;

    impl Character {
        fn dummy() -> Self {
            Self {
                name: "Dummy".into(),
                alignment: (Conformity::Neutral, Morality::Neutral),
                gender: None,
                abilities: Abilities::default(),
                race: Box::new(Human),
                classes: vec![],
                personality: Personality {
                    personality_traits: vec![],
                    ideals: vec![],
                    bonds: vec![],
                    flaws: vec![],
                },
            }
        }
    }

    #[test]
    fn _characters_with_no_classes_should_be_level_0() {
        let character = Character::dummy();

        assert_eq!(character.get_level(), 0);
    }

    #[test]
    fn _characters_with_only_one_class_should_be_the_classs_level() {
        let mut character = Character::dummy();
        character.classes = vec![Box::new(Artificer { level: 1 })];

        assert_eq!(character.get_level(), 1);

        character.classes = vec![Box::new(Artificer { level: 20 })];

        assert_eq!(character.get_level(), 20);
    }

    #[test]
    fn _multiclass_characters_should_sum_classes_levels() {
        let mut character = Character::dummy();
        character.classes = vec![
            Box::new(Artificer { level: 1 }),
            Box::new(Wizard { level: 1 }),
        ];

        assert_eq!(character.get_level(), 2);
    }

    #[test]
    fn _level_1_character_should_have_proficiency_bonus_of_2() {
        let mut character = Character::dummy();
        character.classes = vec![Box::new(Artificer { level: 1 })];

        assert_eq!(character.get_proficiency_bonus(), 2);
    }

    #[test]
    fn _proficiency_bonus_sholud_go_up_by_1_every_4_level_ups() {
        let mut character = Character::dummy();

        character.classes = vec![Box::new(Artificer { level: 4 })];
        assert_eq!(character.get_proficiency_bonus(), 2);

        character.classes = vec![Box::new(Artificer { level: 5 })];
        assert_eq!(character.get_proficiency_bonus(), 3);

        character.classes = vec![Box::new(Artificer { level: 9 })];
        assert_eq!(character.get_proficiency_bonus(), 4);

        character.classes = vec![Box::new(Artificer { level: 13 })];
        assert_eq!(character.get_proficiency_bonus(), 5);

        character.classes = vec![Box::new(Artificer { level: 17 })];
        assert_eq!(character.get_proficiency_bonus(), 6);
    }

    #[test]
    fn _characters_without_a_class_should_have_no_saving_throw_proficiencies() {
        let character = Character::dummy();

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
        let mut character = Character::dummy();
        character.classes = vec![Box::new(Artificer { level: 1 })];

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
        let mut multiclass_character = Character::dummy();
        multiclass_character.classes = vec![
            Box::new(Wizard { level: 1 }),
            Box::new(Artificer { level: 1 }),
        ];

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
        let mut character = Character::dummy();
        character.classes = vec![Box::new(Artificer { level: 1 })];

        assert_eq!(character.get_saving_throw_mod(Ability::Strength), -1);
    }

    #[test]
    fn _should_get_saving_throw_mod_including_proficiency_bonus() {
        let mut character = Character::dummy();
        character.classes = vec![Box::new(Artificer { level: 1 })];

        assert_eq!(character.get_saving_throw_mod(Ability::Constitution), 1);
    }

    #[test]
    fn _should_get_initial_size_from_race() {
        let character = Character::dummy();

        assert_eq!(character.get_size(), Size::Medium);
    }
}
