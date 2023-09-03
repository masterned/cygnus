use std::collections::HashMap;

use crate::{ability::Abilities, feat::Feat, modifiers::Resistance};

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

pub struct Template {
    pub name: String,
    pub creature_type: CreatureType,
    pub size: Size,
    pub walking_speed: usize,
    pub abilities: Abilities,
    pub damage_resistances: HashMap<DamageType, Resistance>,
    pub condition_resistances: HashMap<Condition, Resistance>,
    pub languages: Vec<Language>,
}

pub struct Race {
    name: String,
    creature_type: CreatureType,
    size: Size,
    walking_speed: usize,
    abilities: Abilities,
    damage_resistances: HashMap<DamageType, Resistance>,
    condition_resistances: HashMap<Condition, Resistance>,
    languages: Vec<Language>,
    feats: Vec<Feat>,
}

impl Race {
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn get_creature_type(&self) -> &CreatureType {
        &self.creature_type
    }

    #[must_use]
    pub fn get_size(&self) -> &Size {
        &self.size
    }

    #[must_use]
    pub fn get_walking_speed(&self) -> usize {
        self.walking_speed
    }

    #[must_use]
    pub fn get_abilities(&self) -> &Abilities {
        &self.abilities
    }

    #[must_use]
    pub fn get_damage_resistance(&self, damage_type: &DamageType) -> Option<&Resistance> {
        self.damage_resistances.get(damage_type)
    }

    #[must_use]
    pub fn get_condition_resistance(&self, condition: &Condition) -> Option<&Resistance> {
        self.condition_resistances.get(condition)
    }

    #[must_use]
    pub fn get_languages(&self) -> &[Language] {
        &self.languages
    }

    #[must_use]
    pub fn can_speak(&self, language: &Language) -> bool {
        self.languages.contains(language)
    }

    pub fn get_feats(&self) -> Vec<&Feat> {
        self.feats.iter().collect()
    }

    pub fn add_feat(&mut self, feat: Feat) {
        self.feats.push(feat);
    }
}

impl From<Template> for Race {
    fn from(value: Template) -> Self {
        Race {
            name: value.name,
            creature_type: value.creature_type,
            size: value.size,
            walking_speed: value.walking_speed,
            abilities: value.abilities,
            damage_resistances: value.damage_resistances,
            condition_resistances: value.condition_resistances,
            languages: value.languages,
            feats: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ability::AbilitiesTemplate;

    use super::*;

    impl Race {
        #[must_use]
        pub fn human() -> Self {
            Race {
                name: "Human".into(),
                creature_type: CreatureType::Humanoid,
                size: Size::Medium,
                walking_speed: 30,
                abilities: Abilities::from(AbilitiesTemplate {
                    strength: Some(1),
                    dexterity: Some(1),
                    constitution: Some(1),
                    intelligence: Some(1),
                    wisdom: Some(1),
                    charisma: Some(1),
                }),
                damage_resistances: HashMap::new(),
                condition_resistances: HashMap::new(),
                languages: vec![Language::Common],
                feats: vec![],
            }
        }

        #[must_use]
        pub fn shadar_kai() -> Self {
            Race {
                name: "Shadar-kai".into(),
                creature_type: CreatureType::Humanoid,
                size: Size::Medium,
                walking_speed: 30,
                abilities: Abilities::from(AbilitiesTemplate {
                    intelligence: Some(2),
                    dexterity: Some(1),
                    ..AbilitiesTemplate::default()
                }),
                damage_resistances: HashMap::from([(DamageType::Necrotic, Resistance::Resistant)]),
                condition_resistances: HashMap::from([(
                    Condition::MagicalSleep,
                    Resistance::Immune,
                )]),
                languages: vec![Language::Common, Language::Undercommon],
                feats: vec![],
            }
        }
    }
}
