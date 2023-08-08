use std::{collections::HashMap, fmt};

use crate::{ability::Ability, modifiers::Proficiency};

pub struct ClassTemplate {
    pub name: String,
    pub level: usize,
    pub saving_throw_proficiencies: HashMap<Ability, Proficiency>,
}

#[derive(Debug)]
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

#[derive(Debug, Default)]
pub struct Classes(Vec<Class>);

impl Classes {
    pub fn add_class(&mut self, class: Class) {
        self.0.push(class);
    }

    pub fn get_level(&self) -> usize {
        self.0.iter().map(|class| class.get_level()).sum()
    }

    pub fn get_proficiency_bonus(&self) -> usize {
        self.get_level()
            .checked_sub(1)
            .map(|r| r / 4 + 2)
            .unwrap_or(0)
    }

    pub fn get_saving_throw_proficiency(&self, ability: &Ability) -> Option<&Proficiency> {
        self.0
            .first()
            .and_then(|primary_class| primary_class.get_saving_throw_proficiency(ability))
    }
}

impl fmt::Display for Classes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().fold(String::new(), |acc, class| {
                format!("{acc} {} {}", class.name, class.level)
            })
        )
    }
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

    #[test]
    fn _level_0_should_not_have_proficiency_bonus() {
        let lvl0 = Classes::default();

        assert_eq!(lvl0.get_proficiency_bonus(), 0);
    }

    #[test]
    fn _level_1_character_should_have_proficiency_bonus_of_2() {
        let lvl1 = Classes(vec![Class::artificer()]);

        assert_eq!(lvl1.get_proficiency_bonus(), 2);
    }

    #[test]
    fn _proficiency_bonus_should_go_up_by_1_every_4_level_ups() {
        let lvl4 = Classes(vec![Class {
            name: "lvl4".into(),
            level: 4,
            saving_throw_proficiencies: HashMap::new(),
        }]);
        assert_eq!(lvl4.get_proficiency_bonus(), 2);

        let lvl5 = Classes(vec![Class {
            name: "lvl5".into(),
            level: 5,
            saving_throw_proficiencies: HashMap::new(),
        }]);
        assert_eq!(lvl5.get_proficiency_bonus(), 3);

        let lvl9 = Classes(vec![Class {
            name: "lvl9".into(),
            level: 9,
            saving_throw_proficiencies: HashMap::new(),
        }]);
        assert_eq!(lvl9.get_proficiency_bonus(), 4);

        let lvl13 = Classes(vec![Class {
            name: "lvl13".into(),
            level: 13,
            saving_throw_proficiencies: HashMap::new(),
        }]);
        assert_eq!(lvl13.get_proficiency_bonus(), 5);

        let lvl17 = Classes(vec![Class {
            name: "lvl17".into(),
            level: 17,
            saving_throw_proficiencies: HashMap::new(),
        }]);
        assert_eq!(lvl17.get_proficiency_bonus(), 6);
    }

    #[test]
    fn _classless_should_have_no_saving_throw_proficiencies() {
        let classless = Classes::default();

        assert_eq!(
            classless.get_saving_throw_proficiency(&Ability::Strength),
            None
        );
        assert_eq!(
            classless.get_saving_throw_proficiency(&Ability::Dexterity),
            None
        );
        assert_eq!(
            classless.get_saving_throw_proficiency(&Ability::Constitution),
            None
        );
        assert_eq!(
            classless.get_saving_throw_proficiency(&Ability::Intelligence),
            None
        );
        assert_eq!(
            classless.get_saving_throw_proficiency(&Ability::Wisdom),
            None
        );
        assert_eq!(
            classless.get_saving_throw_proficiency(&Ability::Charisma),
            None
        );
    }

    #[test]
    fn _monoclasses_should_derive_their_saving_throw_proficiencies_from_it() {
        let monoclass = Classes(vec![Class::artificer()]);

        assert_eq!(
            monoclass.get_saving_throw_proficiency(&Ability::Strength),
            None
        );
        assert_eq!(
            monoclass.get_saving_throw_proficiency(&Ability::Dexterity),
            None
        );
        assert_eq!(
            monoclass.get_saving_throw_proficiency(&Ability::Constitution),
            Some(&Proficiency::Proficiency)
        );
        assert_eq!(
            monoclass.get_saving_throw_proficiency(&Ability::Intelligence),
            Some(&Proficiency::Proficiency)
        );
        assert_eq!(
            monoclass.get_saving_throw_proficiency(&Ability::Wisdom),
            None
        );
        assert_eq!(
            monoclass.get_saving_throw_proficiency(&Ability::Charisma),
            None
        );
    }

    #[test]
    fn _multiclasses_should_only_inherit_proficiencies_from_first_class() {
        let multiclass = Classes(vec![Class::wizard(), Class::artificer()]);

        assert_eq!(
            multiclass.get_saving_throw_proficiency(&Ability::Strength),
            None
        );
        assert_eq!(
            multiclass.get_saving_throw_proficiency(&Ability::Dexterity),
            None
        );
        assert_eq!(
            multiclass.get_saving_throw_proficiency(&Ability::Constitution),
            None
        );
        assert_eq!(
            multiclass.get_saving_throw_proficiency(&Ability::Intelligence),
            Some(&Proficiency::Proficiency)
        );
        assert_eq!(
            multiclass.get_saving_throw_proficiency(&Ability::Wisdom),
            Some(&Proficiency::Proficiency)
        );
        assert_eq!(
            multiclass.get_saving_throw_proficiency(&Ability::Charisma),
            None
        );
    }

    #[test]
    fn _classless_should_be_level_0() {
        let classless = Classes::default();

        assert_eq!(classless.get_level(), 0);
    }

    #[test]
    fn _monoclasses_should_be_the_class_level() {
        let mut monoclass = Classes(vec![Class::artificer()]);

        assert_eq!(monoclass.get_level(), 1);

        let mut artificer = Class::artificer();
        artificer.set_level(20).unwrap();
        monoclass = Classes(vec![artificer]);

        assert_eq!(monoclass.get_level(), 20);
    }

    #[test]
    fn _multiclasses_should_sum_classes_levels() {
        let multiclass = Classes(vec![Class::artificer(), Class::wizard()]);

        assert_eq!(multiclass.get_level(), 2);
    }
}
