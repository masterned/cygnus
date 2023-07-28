use std::collections::HashMap;

use crate::{ability::Ability, modifiers::Proficiency};

pub struct ClassTemplate {
    pub name: String,
    pub level: usize,
    pub saving_throw_proficiencies: HashMap<Ability, Proficiency>,
}

pub struct Class {
    name: String,
    level: usize,
    saving_throw_proficiencies: HashMap<Ability, Proficiency>,
}

impl TryFrom<ClassTemplate> for Class {
    type Error = ClassError;

    fn try_from(value: ClassTemplate) -> Result<Self, Self::Error> {
        let mut class = Class {
            name: value.name,
            level: 0,
            saving_throw_proficiencies: value.saving_throw_proficiencies,
        };
        class.set_level(value.level)?;
        Ok(class)
    }
}

impl Class {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_level(&self) -> usize {
        self.level
    }

    pub fn set_level(&mut self, level: usize) -> Result<(), ClassError> {
        if level > 20 {
            return Err(ClassError::LevelOutOfBounds);
        }

        self.level = level;

        Ok(())
    }

    pub fn get_saving_throw_proficiency(&self, ability: &Ability) -> Option<&Proficiency> {
        self.saving_throw_proficiencies.get(ability)
    }
}

#[derive(Debug)]
pub enum ClassError {
    LevelOutOfBounds,
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Class {
        pub fn wizard() -> Self {
            Class {
                name: "Wizard".into(),
                level: 1,
                saving_throw_proficiencies: HashMap::from([
                    (Ability::Intelligence, Proficiency::Proficiency),
                    (Ability::Wisdom, Proficiency::Proficiency),
                ]),
            }
        }

        pub fn artificer() -> Self {
            Class {
                name: "Artificer".into(),
                level: 1,
                saving_throw_proficiencies: HashMap::from([
                    (Ability::Intelligence, Proficiency::Proficiency),
                    (Ability::Constitution, Proficiency::Proficiency),
                ]),
            }
        }
    }
}
