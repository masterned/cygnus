use crate::{
    ability::{Abilities, Ability},
    class::Classes,
    item::{Item, Items},
    modifiers::{Encumbrance, Proficiency},
    race::{CreatureType, Race, Size},
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
    classes: Classes,
    items: Items,
}

impl Character {
    pub fn get_creature_type(&self) -> CreatureType {
        self.race.get_creature_type()
    }

    pub fn get_size(&self) -> Size {
        self.race.get_size()
    }

    pub fn get_walking_speed(&self) -> usize {
        self.race.get_walking_speed()
    }

    pub fn get_ability_score(&self, ability: &Ability) -> usize {
        self.abilities.get_base_score(ability)
    }

    pub fn get_ability_modifier(&self, ability: &Ability) -> isize {
        Ability::calculate_modifier(self.get_ability_score(ability))
    }

    pub fn get_level(&self) -> usize {
        self.classes.get_level()
    }

    pub fn get_proficiency_bonus(&self) -> usize {
        self.classes.get_proficiency_bonus()
    }

    pub fn get_saving_throw_proficiency(&self, ability: &Ability) -> Option<Proficiency> {
        self.classes.get_saving_throw_proficiency(ability)
    }

    pub fn get_saving_throw_mod(&self, ability: &Ability) -> isize {
        self.classes.get_saving_throw_bonus(ability) as isize + self.get_ability_modifier(ability)
    }

    pub fn get_variant_encumbrance(&self) -> Option<Encumbrance> {
        let total_weight_carried = self.items.get_total_weight();
        let strength_score = self.get_ability_score(&Ability::Strength);

        if total_weight_carried > (10 * strength_score) {
            Some(Encumbrance::HeavilyEncumbered)
        } else if total_weight_carried > (5 * strength_score) {
            Some(Encumbrance::Encumbered)
        } else {
            None
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.add_item(item);
    }
}

#[cfg(test)]
mod tests {
    use crate::{class::Artificer, race::Human};

    use super::*;

    impl Character {
        fn dummy() -> Self {
            Self {
                name: "Dummy".into(),
                alignment: (Conformity::Neutral, Morality::Neutral),
                gender: None,
                abilities: Abilities::default(),
                race: Box::<Human>::default(),
                classes: Classes::empty(),
                personality: Personality {
                    personality_traits: vec![],
                    ideals: vec![],
                    bonds: vec![],
                    flaws: vec![],
                },
                items: Items::default(),
            }
        }
    }

    #[test]
    fn _should_default_character_creature_type_to_race_creature_type() {
        let mut character = Character::dummy();
        let race = Human::default();
        let race_creature_type = race.get_creature_type();
        character.race = Box::new(race);

        assert_eq!(character.get_creature_type(), race_creature_type);
    }

    #[test]
    fn _should_get_saving_throw_mod_without_proficiency() {
        let character = Character::dummy();

        assert_eq!(character.get_saving_throw_mod(&Ability::Strength), -1);
    }

    #[test]
    fn _should_get_saving_throw_mod_including_proficiency_bonus() {
        let mut character = Character::dummy();
        character
            .classes
            .add_class(Box::new(Artificer { level: 1 }));

        assert_eq!(character.get_saving_throw_mod(&Ability::Constitution), 1);
    }

    #[test]
    fn _should_get_initial_size_from_race() {
        let character = Character::dummy();

        assert_eq!(character.get_size(), Size::Medium);
    }

    #[test]
    fn _should_get_initial_walking_speed_from_race() {
        let character = Character::dummy();

        assert_eq!(character.get_walking_speed(), 30);
    }

    #[test]
    fn _characters_with_strength_and_no_items_should_not_be_encumbered() {
        let character = Character::dummy();

        assert_eq!(character.get_variant_encumbrance(), None);
    }

    #[test]
    fn _characters_with_more_than_5_times_strength_score_in_item_weight_should_be_encumbered() {
        let mut character = Character::dummy();
        character.add_item(Item::new(42));

        assert_eq!(
            character.get_variant_encumbrance(),
            Some(Encumbrance::Encumbered)
        );
    }

    #[test]
    fn _characters_with_more_than_10_times_str_score_in_item_weight_should_be_heavily_encumbered() {
        let mut character = Character::dummy();
        character.add_item(Item::new(81));

        assert_eq!(
            character.get_variant_encumbrance(),
            Some(Encumbrance::HeavilyEncumbered)
        );
    }
}
