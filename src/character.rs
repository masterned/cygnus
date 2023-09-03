use std::{error, fmt};

use crate::{
    ability::{Abilities, Ability},
    class::{Class, Classes},
    feat::Feat,
    item::{Item, Items},
    modifiers::{Encumbrance, Proficiency},
    race::{CreatureType, Race, Size},
    skill::{Skill, Skills},
    slot::{ItemSlots, Slot, SlotsError},
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
    pub race: Race,
    pub abilities: Abilities,
    pub classes: Classes,
    pub skills: Skills,
    pub inventory: Items,
    pub equipment: ItemSlots,
    pub exhaustion_level: usize,
    pub damage: usize,
}

impl Character {
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn get_race_name(&self) -> &str {
        self.race.get_name()
    }

    #[must_use]
    pub fn get_class_details(&self) -> String {
        self.classes.to_string()
    }

    #[must_use]
    pub fn get_current_hit_points(&self) -> isize {
        self.get_hit_points_max() as isize - self.damage as isize
    }

    #[must_use]
    pub fn get_hit_points_max(&self) -> usize {
        self.classes
            .get_hit_points(self.get_ability_modifier(&Ability::Constitution))
    }

    #[must_use]
    pub fn get_initiative(&self) -> isize {
        self.abilities
            .get_modifier(&Ability::Dexterity)
            .unwrap_or(0)
    }

    #[must_use]
    pub fn get_armor_class(&self) -> usize {
        let dex_mod = self.get_ability_modifier(&Ability::Dexterity);
        self.equipment
            .get_equipped_items()
            .iter()
            .filter_map(|item| item.get_armor_class())
            .map(|armor_class| match armor_class {
                crate::item::ArmorClass::Light(ac) => ac as isize + dex_mod,
                crate::item::ArmorClass::Medium(ac) => {
                    ac as isize + if dex_mod > 2 { 2 } else { dex_mod }
                }
                crate::item::ArmorClass::Heavy(ac) => ac as isize,
            } as usize)
            .sum()
    }

    #[must_use]
    pub fn get_creature_type(&self) -> &CreatureType {
        self.race.get_creature_type()
    }

    #[must_use]
    pub fn get_size(&self) -> &Size {
        self.race.get_size()
    }

    #[must_use]
    pub fn get_walking_speed(&self) -> usize {
        let base_speed = self.race.get_walking_speed();
        let encumbrance_modifier = match self.get_variant_encumbrance() {
            Some(Encumbrance::Encumbered) => 10,
            Some(Encumbrance::HeavilyEncumbered) => 20,
            _ => 0,
        };
        let mut walking_speed = base_speed.saturating_sub(encumbrance_modifier);
        let exhaustion_level = self.get_exhaustion_level();
        if exhaustion_level >= 2 {
            walking_speed /= 2;
        }
        if exhaustion_level >= 5 {
            walking_speed = 0;
        }
        walking_speed
    }

    #[must_use]
    pub fn get_ability_score(&self, ability: &Ability) -> usize {
        self.get_abilities().get_score(ability).unwrap_or(0)
    }

    fn get_abilities(&self) -> Abilities {
        self.abilities + *self.race.get_abilities()
    }

    #[must_use]
    pub fn get_ability_modifier(&self, ability: &Ability) -> isize {
        self.get_abilities().get_modifier(ability).unwrap_or(0)
    }

    #[must_use]
    pub fn get_level(&self) -> usize {
        self.classes.get_level()
    }

    #[must_use]
    pub fn get_proficiency_bonus(&self) -> usize {
        self.classes.get_proficiency_bonus()
    }

    #[must_use]
    pub fn get_saving_throw_proficiency(&self, ability: &Ability) -> Option<&Proficiency> {
        self.classes.get_saving_throw_proficiency(ability)
    }

    #[must_use]
    pub fn get_saving_throw_mod(&self, ability: &Ability) -> isize {
        self.get_proficiency_bonus() as isize
            * (self
                .get_saving_throw_proficiency(ability)
                .map_or(0, |&p| p as isize))
            + self.get_ability_modifier(ability)
    }

    pub fn get_total_weight_carried(&self) -> usize {
        self.inventory.get_total_weight() + self.equipment.get_total_weight()
    }

    #[must_use]
    pub fn get_variant_encumbrance(&self) -> Option<Encumbrance> {
        let total_weight_carried = self.get_total_weight_carried();
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
        self.inventory.add_item(item);
    }

    #[must_use]
    pub fn get_exhaustion_level(&self) -> usize {
        self.exhaustion_level
    }

    pub fn set_exhaustion_level(&mut self, new_level: usize) {
        self.exhaustion_level = new_level;
    }

    #[must_use]
    pub fn get_skill_modifier(&self, skill: &Skill) -> isize {
        self.get_ability_modifier(&skill.get_ability())
            + (match self.skills.get_proficiency(skill) {
                Some(Proficiency::Proficiency) => self.get_proficiency_bonus(),
                Some(Proficiency::Expertise) => self.get_proficiency_bonus() * 2,
                None => 0,
            }) as isize
    }

    #[must_use]
    pub fn get_passive_perception(&self) -> usize {
        (10 + self.get_skill_modifier(&Skill::Perception)) as usize
    }

    #[must_use]
    pub fn get_passive_investigation(&self) -> usize {
        (10 + self.get_skill_modifier(&Skill::Investigation)) as usize
    }

    #[must_use]
    pub fn get_passive_insight(&self) -> usize {
        (10 + self.get_skill_modifier(&Skill::Insight)) as usize
    }

    pub fn add_class(&mut self, class: Class) {
        self.classes.add_class(class);
    }

    pub fn add_equipment_slot(
        &mut self,
        slot_name: impl Into<String>,
        slot: Slot<Item, fn(&Item) -> bool>,
    ) {
        self.equipment.add_slot(slot_name, slot);
    }

    pub fn equip_item(&mut self, item: Item, slot_name: impl Into<String>) -> CharacterResult<()> {
        self.equipment.equip(item, slot_name)?;

        Ok(())
    }

    pub fn has_item_equipped_matching_criteria(&self, item_criteria: fn(&Item) -> bool) -> bool {
        self.equipment
            .has_item_equipped_matching_criteria(item_criteria)
    }

    pub fn get_feats(&self) -> Vec<&Feat> {
        [self.classes.get_feats(), self.race.get_feats()].concat()
    }
}

type CharacterResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Equipment(SlotsError),
}

impl From<SlotsError> for Error {
    fn from(value: SlotsError) -> Self {
        Error::Equipment(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            Error::Equipment(e) => format!("Equipment: {e}"),
        };

        write!(f, "{result}")
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use crate::{ability::AbilitiesTemplate, item::ArmorClass};

    use super::*;

    impl Character {
        fn dummy() -> Self {
            Self {
                name: "Dummy".into(),
                alignment: (Conformity::Neutral, Morality::Neutral),
                gender: None,
                abilities: Abilities::from(AbilitiesTemplate {
                    strength: Some(8),
                    dexterity: Some(8),
                    constitution: Some(8),
                    intelligence: Some(8),
                    wisdom: Some(8),
                    charisma: Some(8),
                }),
                race: Race::human(),
                classes: Classes::default(),
                personality: Personality {
                    personality_traits: vec![],
                    ideals: vec![],
                    bonds: vec![],
                    flaws: vec![],
                },
                skills: Skills::default(),
                inventory: Items::default(),
                exhaustion_level: 0,
                damage: 0,
                equipment: ItemSlots::default(),
            }
        }
    }

    #[test]
    fn _should_default_character_creature_type_to_race_creature_type() {
        let character = Character::dummy();

        assert_eq!(character.get_creature_type(), &CreatureType::Humanoid);
    }

    #[test]
    fn _should_get_saving_throw_mod_without_proficiency() {
        let character = Character::dummy();

        assert_eq!(character.get_saving_throw_mod(&Ability::Strength), -1);
    }

    #[test]
    fn _should_get_saving_throw_mod_including_proficiency_bonus() {
        let mut character = Character::dummy();
        character.add_class(Class::artificer());

        assert_eq!(character.get_saving_throw_mod(&Ability::Constitution), 1);
    }

    #[test]
    fn _should_get_initial_size_from_race() {
        let character = Character::dummy();

        assert_eq!(character.get_size(), &Size::Medium);
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
        character.add_item(Item::new("test", 46, vec![], None));

        assert_eq!(
            character.get_variant_encumbrance(),
            Some(Encumbrance::Encumbered)
        );
    }

    #[test]
    fn _characters_with_more_than_10_times_str_score_in_item_weight_should_be_heavily_encumbered() {
        let mut character = Character::dummy();
        character.add_item(Item::new("test", 91, vec![], None));

        assert_eq!(
            character.get_variant_encumbrance(),
            Some(Encumbrance::HeavilyEncumbered)
        );
    }

    #[test]
    fn _should_include_inventory_and_equipment_in_total_carried_weight() {
        let mut character = Character::dummy();

        character.add_item(Item::new("Rapier", 2, vec!["weapon".into()], None));

        character.add_equipment_slot("armor", Slot::new(|_| true));
        let _ = character.equip_item(
            Item::new(
                "Chain Mail",
                55,
                vec!["armor".into()],
                Some(ArmorClass::Heavy(16)),
            ),
            "armor",
        );

        assert_eq!(character.get_total_weight_carried(), 57);
    }

    #[test]
    fn _encumbered_characters_should_reduce_their_speed_by_10() {
        let mut character = Character::dummy();
        character.add_item(Item::new("test", 46, vec![], None));

        assert_eq!(character.get_walking_speed(), 20);
    }

    #[test]
    fn _heavily_encumbered_characters_should_reduce_their_speed_by_20() {
        let mut character = Character::dummy();
        character.add_item(Item::new("test", 91, vec![], None));

        assert_eq!(character.get_walking_speed(), 10);
    }

    #[test]
    fn _characters_with_2_or_more_exhaustion_should_half_their_movement_speed() {
        let mut character = Character::dummy();

        character.set_exhaustion_level(2);
        assert_eq!(character.get_walking_speed(), 15);

        character.set_exhaustion_level(3);
        assert_eq!(character.get_walking_speed(), 15);

        character.set_exhaustion_level(4);
        assert_eq!(character.get_walking_speed(), 15);
    }

    #[test]
    fn _characters_with_5_levels_of_exhaustion_should_have_0_movement_speed() {
        let mut character = Character::dummy();

        character.set_exhaustion_level(5);
        assert_eq!(character.get_walking_speed(), 0);
    }

    #[test]
    fn _skill_modifier_should_default_to_related_ability_modifier() {
        let character = Character::dummy();

        assert_eq!(character.get_skill_modifier(&Skill::Arcana), -1);
    }

    #[test]
    fn _proficiency_should_affect_skill_modifier() {
        let mut character = Character::dummy();
        character.add_class(Class::wizard());
        character
            .skills
            .set_proficiency(Skill::Arcana, Some(Proficiency::Proficiency));

        assert_eq!(character.get_skill_modifier(&Skill::Arcana), 1);
    }

    #[test]
    fn _expertise_should_affect_skill_modifier() {
        let mut character = Character::dummy();
        character.add_class(Class::wizard());
        character
            .skills
            .set_proficiency(Skill::Arcana, Some(Proficiency::Expertise));

        assert_eq!(character.get_skill_modifier(&Skill::Arcana), 3);
    }

    #[test]
    fn _passive_perception_should_be_10_plus_perception_modifier() {
        let mut character = Character::dummy();
        character.add_class(Class::wizard());

        assert_eq!(character.get_passive_perception(), 9);

        character
            .skills
            .set_proficiency(Skill::Perception, Some(Proficiency::Proficiency));
        assert_eq!(character.get_passive_perception(), 11);
    }

    #[test]
    fn _passive_investigation_should_be_10_plus_investigation_modifier() {
        let mut character = Character::dummy();
        character.add_class(Class::wizard());

        assert_eq!(character.get_passive_investigation(), 9);

        character
            .skills
            .set_proficiency(Skill::Investigation, Some(Proficiency::Proficiency));
        assert_eq!(character.get_passive_investigation(), 11);
    }

    #[test]
    fn _passive_insight_should_be_10_plus_insight_modifier() {
        let mut character = Character::dummy();
        character.add_class(Class::wizard());

        assert_eq!(character.get_passive_insight(), 9);

        character
            .skills
            .set_proficiency(Skill::Insight, Some(Proficiency::Proficiency));
        assert_eq!(character.get_passive_insight(), 11);
    }

    #[test]
    fn _should_obtain_feats_from_classes_and_race() {
        let mut character = Character::dummy();

        let spell_sniper = Feat::new(
            "Spell Sniper",
            "Doubles casting distance and ignores half cover.",
        );
        let mut wizard = Class::wizard();
        wizard.add_feat(spell_sniper.clone());

        character.add_class(wizard);

        let elven_accuracy = Feat::new(
            "Elven Accuracy",
            "When rolling advantage on ranged checks, roll a third die.",
        );
        let mut shadar_kai = Race::shadar_kai();
        shadar_kai.add_feat(elven_accuracy.clone());
        character.race = shadar_kai;

        assert_eq!(character.get_feats(), vec![&spell_sniper, &elven_accuracy]);
    }

    #[test]
    fn _should_derive_armor_class_from_equipment_and_con_mod() -> CharacterResult<()> {
        let mut character = Character::dummy();
        character.add_equipment_slot("chestplate", Slot::new(|_| true));
        character.add_equipment_slot("helmet", Slot::new(|_| true));
        character.equip_item(
            Item::new(
                "Breastplate",
                25,
                vec!["armor".into()],
                Some(ArmorClass::Medium(14)),
            ),
            "chestplate",
        )?;
        character.equip_item(
            Item::new(
                "Pickelbonnet",
                2,
                vec!["armor".into()],
                Some(ArmorClass::Heavy(3)),
            ),
            "helmet",
        )?;

        assert_eq!(character.get_armor_class(), 16);

        Ok(())
    }
}
