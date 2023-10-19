use std::{error, fmt};

use crate::{
    ability::{self, Abilities},
    class::{Class, Classes},
    feat::Feat,
    item::{self, Item, Items},
    modifiers::{Encumbrance, Proficiency},
    personality::Personality,
    proficiencies::Proficiencies,
    race::{CreatureType, Race, Size},
    senses::Senses,
    skills::{self, Skills},
    slot::{ItemSlots, Slot, SlotsError},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Conformity {
    Lawful,
    Neutral,
    Chaotic,
}

impl fmt::Display for Conformity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            Conformity::Lawful => "Lawful",
            Conformity::Neutral => "Neutral",
            Conformity::Chaotic => "Chaotic",
        };

        write!(f, "{result}")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Morality {
    Good,
    Neutral,
    Evil,
}

impl fmt::Display for Morality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            Morality::Good => "Good",
            Morality::Neutral => "Neutral",
            Morality::Evil => "Evil",
        };

        write!(f, "{result}")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Alignment(pub Conformity, pub Morality);

impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Gender::Male => "Male",
                Gender::Female => "Female",
            }
        )
    }
}

#[derive(Clone, Debug, Default)]
pub struct Builder {
    name: Option<String>,
    alignment: Option<Alignment>,
    gender: Option<Gender>,
    personality: Option<Personality>,
    race: Option<Race>,
    base_ability_scores: Option<Abilities>,
    classes: Option<Classes>,
    skill_proficiencies: Option<Skills>,
    inventory: Option<Items>,
    equipment: Option<ItemSlots>,
    senses: Option<Senses>,
    proficiencies: Option<Proficiencies>,
}

impl Builder {
    pub fn new() -> Self {
        Builder::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Result<Self, ConstructionError> {
        let name: String = name.into();

        if name.is_empty() {
            return Err(ConstructionError::MissingField("name".into()));
        }

        let _ = self.name.insert(name);

        Ok(self)
    }

    pub fn alignment(
        mut self,
        conformity: Conformity,
        morality: Morality,
    ) -> Result<Self, ConstructionError> {
        let _ = self.alignment.insert(Alignment(conformity, morality));

        Ok(self)
    }

    pub fn gender(mut self, gender: Gender) -> Result<Self, ConstructionError> {
        let _ = self.gender.insert(gender);

        Ok(self)
    }

    pub fn personality(mut self, personality: Personality) -> Result<Self, ConstructionError> {
        let _ = self.personality.insert(personality);

        Ok(self)
    }

    pub fn race(mut self, race: Race) -> Result<Self, ConstructionError> {
        let _ = self.race.insert(race);

        Ok(self)
    }

    pub fn base_ability_scores(
        mut self,
        ability_scores: Abilities,
    ) -> Result<Self, ConstructionError> {
        let _ = self.base_ability_scores.insert(ability_scores);

        Ok(self)
    }

    pub fn add_class(mut self, class: Class) -> Result<Self, ConstructionError> {
        let classes = self.classes.get_or_insert_with(Default::default);

        classes.add_class(class);

        Ok(self)
    }

    pub fn add_skill_proficiency(
        mut self,
        skill: skills::Identifier,
    ) -> Result<Self, ConstructionError> {
        let skills = self
            .skill_proficiencies
            .get_or_insert_with(Default::default);

        skills.set_proficiency(skill, Some(Proficiency::Proficiency));

        Ok(self)
    }

    pub fn add_skill_expertise(
        mut self,
        skill: skills::Identifier,
    ) -> Result<Self, ConstructionError> {
        let skills = self
            .skill_proficiencies
            .get_or_insert_with(Default::default);

        skills.set_proficiency(skill, Some(Proficiency::Expertise));

        Ok(self)
    }

    pub fn inventory(mut self, inventory: Items) -> Result<Self, ConstructionError> {
        let _ = self.inventory.insert(inventory);

        Ok(self)
    }

    pub fn add_item_to_inventory(mut self, item: Item) -> Result<Self, ConstructionError> {
        let inventory = self.inventory.get_or_insert_with(Default::default);

        inventory.add_item(item);

        Ok(self)
    }

    pub fn equipment(mut self, equipment: ItemSlots) -> Result<Self, ConstructionError> {
        let _ = self.equipment.insert(equipment);

        Ok(self)
    }

    pub fn add_equipment_slot(
        mut self,
        slot_name: impl Into<String>,
        slot: Slot<Item, fn(&Item) -> bool>,
    ) -> Result<Self, ConstructionError> {
        let equipment = self.equipment.get_or_insert_with(Default::default);

        equipment.add_slot(slot_name, slot);

        Ok(self)
    }

    pub fn senses(mut self, senses: Senses) -> Result<Self, ConstructionError> {
        let _ = self.senses.insert(senses);

        Ok(self)
    }

    pub fn add_armor_proficiency(
        mut self,
        armor_class: impl Into<String>,
    ) -> Result<Self, ConstructionError> {
        let proficiencies = self.proficiencies.get_or_insert_with(Default::default);

        proficiencies.add_armor_proficiency(armor_class);

        Ok(self)
    }

    pub fn add_weapon_proficiency(
        mut self,
        weapon: impl Into<String>,
    ) -> Result<Self, ConstructionError> {
        let proficiencies = self.proficiencies.get_or_insert_with(Default::default);

        proficiencies.add_weapon_proficiency(weapon);

        Ok(self)
    }

    pub fn add_tool_proficiency(
        mut self,
        tool: impl Into<String>,
    ) -> Result<Self, ConstructionError> {
        let proficiencies = self.proficiencies.get_or_insert_with(Default::default);

        proficiencies.add_tool_proficiency(tool);

        Ok(self)
    }

    pub fn build(self) -> Result<Character, ConstructionError> {
        let name = self
            .name
            .ok_or(ConstructionError::MissingField("name".into()))?;

        let alignment = self
            .alignment
            .ok_or(ConstructionError::MissingField("alignment".into()))?;

        let personality = self.personality.unwrap_or_default();

        let race = self
            .race
            .ok_or(ConstructionError::MissingField("race".into()))?;

        let base_ability_scores = self
            .base_ability_scores
            .ok_or(ConstructionError::MissingField("ability scores".into()))?;

        let classes = self
            .classes
            .ok_or(ConstructionError::MissingField("class(es)".into()))?;

        let skill_proficiencies = self.skill_proficiencies.unwrap_or_default();

        let inventory = self.inventory.unwrap_or_default();

        let equipment = self.equipment.unwrap_or_default();

        let senses = self.senses.unwrap_or_default();

        let proficiencies = self.proficiencies.unwrap_or_default();

        Ok(Character {
            name,
            alignment,
            gender: self.gender,
            personality,
            race,
            base_ability_scores,
            classes,
            skills: skill_proficiencies,
            inventory,
            equipment,
            exhaustion_level: 0,
            damage: 0,
            senses,
            proficiencies,
        })
    }
}

#[derive(Debug)]
pub enum ConstructionError {
    MissingField(String),
}

impl fmt::Display for ConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            ConstructionError::MissingField(field_name) => {
                format!("Unable to create Character without {field_name}.")
            }
        };

        write!(f, "{result}")
    }
}

impl error::Error for ConstructionError {}

#[derive(Debug)]
pub struct Character {
    name: String,
    alignment: Alignment,
    gender: Option<Gender>,
    personality: Personality,
    race: Race,
    base_ability_scores: Abilities,
    classes: Classes,
    skills: Skills,
    inventory: Items,
    equipment: ItemSlots,
    exhaustion_level: usize,
    damage: usize,
    senses: Senses,
    proficiencies: Proficiencies,
}

impl Character {
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn get_alignment(&self) -> Alignment {
        self.alignment
    }

    #[must_use]
    pub fn get_gender(&self) -> Option<Gender> {
        self.gender
    }

    #[must_use]
    pub fn get_personality(&self) -> &Personality {
        &self.personality
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
        let constitution_modifier = self.get_ability_modifier(ability::Identifier::Constitution);

        self.classes.get_hit_points(constitution_modifier)
    }

    #[must_use]
    pub fn get_initiative(&self) -> isize {
        self.get_ability_modifier(ability::Identifier::Dexterity)
    }

    #[must_use]
    pub fn get_armor_class(&self) -> usize {
        let dex_mod = self.get_ability_modifier(ability::Identifier::Dexterity);

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

    pub fn get_abilities(&self) -> Abilities {
        self.base_ability_scores.clone() + self.race.get_abilities().clone()
    }

    #[must_use]
    pub fn get_ability_score(&self, ability: ability::Identifier) -> usize {
        self.get_abilities().get_score(ability).unwrap_or(0)
    }

    #[must_use]
    pub fn get_ability_modifier(&self, ability: ability::Identifier) -> isize {
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
    pub fn get_saving_throw_proficiency(
        &self,
        ability: ability::Identifier,
    ) -> Option<&Proficiency> {
        self.classes.get_saving_throw_proficiency(ability)
    }

    #[must_use]
    pub fn get_saving_throw_mod(&self, ability: ability::Identifier) -> isize {
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
        let strength_score = self.get_ability_score(ability::Identifier::Strength);

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

    pub fn get_skill_proficiency(&self, skill: skills::Identifier) -> Option<Proficiency> {
        self.skills.get_proficiency(skill)
    }

    #[must_use]
    pub fn get_skill_modifier(&self, skill: skills::Identifier) -> isize {
        self.skills.get_modifier(
            skill,
            self.get_ability_modifier(skill.get_default_ability()),
            self.get_proficiency_bonus(),
        )
    }

    #[must_use]
    pub fn get_passive_perception(&self) -> usize {
        self.senses
            .get_passive_perception(self.get_skill_modifier(skills::Identifier::Perception))
    }

    #[must_use]
    pub fn get_passive_investigation(&self) -> usize {
        self.senses
            .get_passive_perception(self.get_skill_modifier(skills::Identifier::Investigation))
    }

    #[must_use]
    pub fn get_passive_insight(&self) -> usize {
        self.senses
            .get_passive_insight(self.get_skill_modifier(skills::Identifier::Insight))
    }

    pub fn get_darkvision(&self) -> Option<usize> {
        self.senses.get_darkvision()
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

    pub fn get_armor_proficiencies_string(&self) -> String {
        self.proficiencies.get_armor_proficiencies_string()
    }

    pub fn get_weapon_proficiencies_string(&self) -> String {
        self.proficiencies.get_weapon_proficiencies_string()
    }

    pub fn get_tool_proficiencies_string(&self) -> String {
        self.proficiencies.get_tool_proficiencies_string()
    }

    pub fn get_languages_string(&self) -> String {
        let languages: Vec<String> = self
            .proficiencies
            .get_languages()
            .iter()
            .map(|lang| lang.to_string())
            .chain(
                self.race
                    .get_languages()
                    .iter()
                    .map(|lang| lang.to_string()),
            )
            .collect();

        languages.join(", ")
    }
}

type CharacterResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Equipment(SlotsError),
    Inventory(item::ConstructionError),
}

impl From<SlotsError> for Error {
    fn from(value: SlotsError) -> Self {
        Error::Equipment(value)
    }
}

impl From<item::ConstructionError> for Error {
    fn from(value: item::ConstructionError) -> Self {
        Error::Inventory(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            Error::Equipment(e) => format!("Equipment: {e}"),
            Error::Inventory(e) => format!("Inventory: {e}"),
        };

        write!(f, "{result}")
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use crate::{
        ability::AbilitiesTemplate,
        item::{self, ArmorClass},
    };

    use super::*;

    impl Character {
        fn dummy() -> Self {
            Self {
                name: "Dummy".into(),
                alignment: Alignment(Conformity::Neutral, Morality::Neutral),
                gender: None,
                base_ability_scores: Abilities::from(AbilitiesTemplate {
                    strength: 8,
                    dexterity: 8,
                    constitution: 8,
                    intelligence: 8,
                    wisdom: 8,
                    charisma: 8,
                }),
                race: Race::human(),
                classes: Classes::default(),
                personality: Personality::default(),
                skills: Skills::default(),
                inventory: Items::default(),
                exhaustion_level: 0,
                damage: 0,
                equipment: ItemSlots::default(),
                senses: Senses::default(),
                proficiencies: Proficiencies::default(),
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

        assert_eq!(
            character.get_saving_throw_mod(ability::Identifier::Strength),
            -1
        );
    }

    #[test]
    fn _should_get_saving_throw_mod_including_proficiency_bonus() {
        let mut character = Character::dummy();
        character.add_class(Class::artificer());

        assert_eq!(
            character.get_saving_throw_mod(ability::Identifier::Constitution),
            1
        );
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
    fn _characters_with_more_than_5_times_strength_score_in_item_weight_should_be_encumbered(
    ) -> CharacterResult<()> {
        let mut character = Character::dummy();

        let item = item::Builder::new().name("test")?.weight(46)?.build()?;
        character.add_item(item);

        assert_eq!(
            character.get_variant_encumbrance(),
            Some(Encumbrance::Encumbered)
        );

        Ok(())
    }

    #[test]
    fn _characters_with_more_than_10_times_str_score_in_item_weight_should_be_heavily_encumbered(
    ) -> CharacterResult<()> {
        let mut character = Character::dummy();

        let item = item::Builder::new().name("test")?.weight(91)?.build()?;
        character.add_item(item);

        assert_eq!(
            character.get_variant_encumbrance(),
            Some(Encumbrance::HeavilyEncumbered)
        );

        Ok(())
    }

    #[test]
    fn _should_include_inventory_and_equipment_in_total_carried_weight() -> CharacterResult<()> {
        let mut character = Character::dummy();

        let rapier = item::Builder::new()
            .name("Rapier")?
            .weight(2)?
            .add_type("weapon")?
            .build()?;
        character.add_item(rapier);

        character.add_equipment_slot("armor", Slot::new(|_| true));
        let chain_mail = item::Builder::new()
            .name("Chain Mail")?
            .weight(55)?
            .add_type("armor")?
            .armor_class(ArmorClass::Heavy(16))?
            .build()?;
        let _ = character.equip_item(chain_mail, "armor");

        assert_eq!(character.get_total_weight_carried(), 57);

        Ok(())
    }

    #[test]
    fn _encumbered_characters_should_reduce_their_speed_by_10() -> CharacterResult<()> {
        let mut character = Character::dummy();

        let item = item::Builder::new().name("test")?.weight(46)?.build()?;
        character.add_item(item);

        assert_eq!(character.get_walking_speed(), 20);

        Ok(())
    }

    #[test]
    fn _heavily_encumbered_characters_should_reduce_their_speed_by_20() -> CharacterResult<()> {
        let mut character = Character::dummy();

        let item = item::Builder::new().name("test")?.weight(91)?.build()?;
        character.add_item(item);

        assert_eq!(character.get_walking_speed(), 10);

        Ok(())
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

        assert_eq!(character.get_skill_modifier(skills::Identifier::Arcana), -1);
    }

    #[test]
    fn _proficiency_should_affect_skill_modifier() {
        let mut character = Character::dummy();

        let mut class = Class::wizard();
        class.set_level(1).unwrap();
        character.add_class(class);

        character
            .skills
            .set_proficiency(skills::Identifier::Arcana, Some(Proficiency::Proficiency));

        assert_eq!(character.get_skill_modifier(skills::Identifier::Arcana), 1);
    }

    #[test]
    fn _expertise_should_affect_skill_modifier() {
        let mut character = Character::dummy();

        let mut class = Class::wizard();
        class.set_level(1).unwrap();

        character.add_class(class);
        character
            .skills
            .set_proficiency(skills::Identifier::Arcana, Some(Proficiency::Expertise));

        assert_eq!(character.get_skill_modifier(skills::Identifier::Arcana), 3);
    }

    #[test]
    fn _passive_perception_should_be_10_plus_perception_modifier() {
        let mut character = Character::dummy();
        character.add_class(Class::wizard());

        assert_eq!(character.get_passive_perception(), 9);

        character.skills.set_proficiency(
            skills::Identifier::Perception,
            Some(Proficiency::Proficiency),
        );
        assert_eq!(character.get_passive_perception(), 11);
    }

    #[test]
    fn _passive_investigation_should_be_10_plus_investigation_modifier() {
        let mut character = Character::dummy();
        character.add_class(Class::wizard());

        assert_eq!(character.get_passive_investigation(), 9);

        character.skills.set_proficiency(
            skills::Identifier::Investigation,
            Some(Proficiency::Proficiency),
        );
        assert_eq!(character.get_passive_investigation(), 11);
    }

    #[test]
    fn _passive_insight_should_be_10_plus_insight_modifier() {
        let mut character = Character::dummy();
        character.add_class(Class::wizard());

        assert_eq!(character.get_passive_insight(), 9);

        character
            .skills
            .set_proficiency(skills::Identifier::Insight, Some(Proficiency::Proficiency));
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

        let breastplate = item::Builder::new()
            .name("Breastplate")?
            .weight(25)?
            .add_type("armor")?
            .armor_class(ArmorClass::Medium(14))?
            .build()?;
        character.equip_item(breastplate, "chestplate")?;

        let pickelbonnet = item::Builder::new()
            .name("Pickelbonnet")?
            .weight(2)?
            .add_type("armor")?
            .armor_class(ArmorClass::Heavy(3))?
            .build()?;
        character.equip_item(pickelbonnet, "helmet")?;

        assert_eq!(character.get_armor_class(), 16);

        Ok(())
    }
}
