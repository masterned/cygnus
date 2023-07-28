use std::collections::HashMap;

use crate::{ability::Ability, feature::Feature, modifiers::Resistance};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CreatureType {
    Aberration,
    Beast,
    Celestial,
    Construct,
    Dragon,
    Elemental,
    Fey,
    Fiend,
    Giant,
    Humanoid,
    Monstrosity,
    Ooze,
    Plant,
    Undead,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DamageType {
    Necrotic,
    Radiant,
    Poison,
    Force,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Condition {
    MagicalSleep,
    Constrained,
    Unconscience,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    Common,
    Undercommon,
}

pub struct RaceTemplate {
    pub name: String,
    pub creature_type: CreatureType,
    pub size: Size,
    pub walking_speed: usize,
    pub abilities: HashMap<Ability, usize>,
    pub damage_resistances: HashMap<DamageType, Resistance>,
    pub condition_resistances: HashMap<Condition, Resistance>,
    pub languages: Vec<Language>,
    pub features: Vec<Feature>,
}

pub struct Race {
    name: String,
    creature_type: CreatureType,
    size: Size,
    walking_speed: usize,
    abilities: HashMap<Ability, usize>,
    damage_resistances: HashMap<DamageType, Resistance>,
    condition_resistances: HashMap<Condition, Resistance>,
    languages: Vec<Language>,
    features: Vec<Feature>,
}

impl Race {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_creature_type(&self) -> &CreatureType {
        &self.creature_type
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }

    pub fn get_walking_speed(&self) -> usize {
        self.walking_speed
    }

    pub fn get_ability_score_bonus(&self, ability: &Ability) -> usize {
        *self.abilities.get(ability).unwrap_or(&0)
    }

    pub fn get_damage_resistance(&self, damage_type: &DamageType) -> Option<&Resistance> {
        self.damage_resistances.get(damage_type)
    }

    pub fn get_condition_resistance(&self, condition: &Condition) -> Option<&Resistance> {
        self.condition_resistances.get(condition)
    }

    pub fn get_languages(&self) -> &[Language] {
        &self.languages
    }

    pub fn can_speak(&self, language: &Language) -> bool {
        self.languages.contains(language)
    }

    pub fn get_features(&self) -> &[Feature] {
        &self.features
    }
}

impl From<RaceTemplate> for Race {
    fn from(value: RaceTemplate) -> Self {
        Race {
            name: value.name,
            creature_type: value.creature_type,
            size: value.size,
            walking_speed: value.walking_speed,
            abilities: value.abilities,
            damage_resistances: value.damage_resistances,
            condition_resistances: value.condition_resistances,
            languages: value.languages,
            features: value.features,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Race {
        pub fn human() -> Self {
            Race {
                name: "Human".into(),
                creature_type: CreatureType::Humanoid,
                size: Size::Medium,
                walking_speed: 30,
                abilities: HashMap::from_iter(Ability::all().iter().map(|&a| (a, 1))),
                damage_resistances: HashMap::new(),
                condition_resistances: HashMap::new(),
                languages: vec![Language::Common],
                features: vec![],
            }
        }

        pub fn shadar_kai() -> Self {
            Race {
                name: "Shadar-kai".into(),
                creature_type: CreatureType::Humanoid,
                size: Size::Medium,
                walking_speed: 30,
                abilities: HashMap::from([(Ability::Intelligence, 2), (Ability::Dexterity, 1)]),
                damage_resistances: HashMap::from([(DamageType::Necrotic, Resistance::Resistant)]),
                condition_resistances: HashMap::from([(
                    Condition::MagicalSleep,
                    Resistance::Immune,
                )]),
                languages: vec![Language::Common, Language::Undercommon],
                features: vec![],
            }
        }
    }
}
