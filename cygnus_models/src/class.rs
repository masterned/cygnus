use std::{collections::HashMap, error, fmt};

use crate::{ability::Ability, feat::Feat, modifiers::Proficiency, spell::SpellList};

#[derive(Clone, Debug, Default)]
pub struct HPIncreases(Vec<usize>);

impl HPIncreases {
    #[must_use]
    pub fn new(hit_die_sides: usize) -> Self {
        HPIncreases(vec![hit_die_sides])
    }

    #[must_use]
    pub fn get_hit_points(&self, constitution_modifier: isize) -> usize {
        let increase_count = self.0.len() as isize;

        (self.0.iter().sum::<usize>() as isize + (constitution_modifier * increase_count)) as usize
    }

    ///
    /// # Errors
    ///
    /// - `IncorrectNumberOfIncreases`: if the caller tries to add more than 20 increases
    ///
    pub fn add_increase(&mut self, increase: usize) -> Result<(), HPIncreaseConstructionError> {
        if self.0.len() >= 20 {
            return Err(HPIncreaseConstructionError::IncorrectNumberOfIncreases);
        }

        self.0.push(increase);

        Ok(())
    }
}

impl TryFrom<Vec<usize>> for HPIncreases {
    type Error = HPIncreaseConstructionError;

    fn try_from(value: Vec<usize>) -> Result<Self, Self::Error> {
        if value.len() > 20 {
            return Err(HPIncreaseConstructionError::IncorrectNumberOfIncreases);
        }

        Ok(HPIncreases(value))
    }
}

#[derive(Debug)]
pub enum HPIncreaseConstructionError {
    IncorrectNumberOfIncreases,
}

impl fmt::Display for HPIncreaseConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            HPIncreaseConstructionError::IncorrectNumberOfIncreases => {
                "Cannot have more increases than maximum level."
            }
        };

        write!(f, "{result}")
    }
}

impl error::Error for HPIncreaseConstructionError {}

#[derive(Clone, Debug, Default)]
pub struct Builder {
    name: Option<String>,
    level: Option<usize>,
    saving_throw_proficiencies: HashMap<Ability, Proficiency>,
    spell_list: Option<SpellList>,
    hp_increases: Option<HPIncreases>,
    feats: Vec<Feat>,
}

impl Builder {
    pub fn new() -> Self {
        Builder::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Result<Self, ClassConstructionError> {
        let name: String = name.into();

        if name.is_empty() {
            return Err(ClassConstructionError::MissingName);
        }

        let _ = self.name.insert(name);

        Ok(self)
    }

    pub fn level(mut self, level: usize) -> Result<Self, ClassConstructionError> {
        if level > 20 {
            return Err(ClassConstructionError::LevelOutOfBounds);
        }

        let _ = self.level.insert(level);

        Ok(self)
    }

    pub fn add_saving_throw_proficiency(
        mut self,
        ability: Ability,
    ) -> Result<Self, ClassConstructionError> {
        self.saving_throw_proficiencies
            .insert(ability, Proficiency::Proficiency);

        Ok(self)
    }

    pub fn spell_list(mut self, spell_list: SpellList) -> Result<Self, ClassConstructionError> {
        self.spell_list = Some(spell_list);

        Ok(self)
    }

    pub fn hp_increases(
        mut self,
        hp_increases: HPIncreases,
    ) -> Result<Self, ClassConstructionError> {
        self.hp_increases = Some(hp_increases);

        Ok(self)
    }

    pub fn add_feat(mut self, feat: Feat) -> Result<Self, ClassConstructionError> {
        self.feats.push(feat);

        Ok(self)
    }

    pub fn build(self) -> Result<Class, ClassConstructionError> {
        let name = self.name.ok_or(ClassConstructionError::MissingName)?;

        let level = self.level.ok_or(ClassConstructionError::LevelOutOfBounds)?;

        let saving_throw_proficiencies = self.saving_throw_proficiencies;

        let spell_list = self.spell_list;

        let hp_increases = self.hp_increases.unwrap_or_default();

        let feats = self.feats;

        Ok(Class {
            name,
            level,
            saving_throw_proficiencies,
            spell_list,
            hp_increases,
            feats,
        })
    }
}

#[derive(Debug)]
pub enum ClassConstructionError {
    MissingName,
    LevelOutOfBounds,
}

impl fmt::Display for ClassConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            ClassConstructionError::MissingName => "Cannot create a Class without a name.",
            ClassConstructionError::LevelOutOfBounds => "Level must be between 1 and 20.",
        };

        write!(f, "{result}")
    }
}

impl error::Error for ClassConstructionError {}

pub struct Template {
    pub name: String,
    pub level: usize,
    pub saving_throw_proficiencies: HashMap<Ability, Proficiency>,
    pub spell_list: Option<SpellList>,
    pub hp_increases: HPIncreases,
    pub feats: Vec<Feat>,
}

#[derive(Clone, Debug)]
pub struct Class {
    name: String,
    level: usize,
    saving_throw_proficiencies: HashMap<Ability, Proficiency>,
    spell_list: Option<SpellList>,
    hp_increases: HPIncreases,
    feats: Vec<Feat>,
}

impl TryFrom<Template> for Class {
    type Error = TryFromError;

    fn try_from(value: Template) -> Result<Self, Self::Error> {
        let mut class = Class {
            name: value.name,
            level: 0,
            saving_throw_proficiencies: value.saving_throw_proficiencies,
            spell_list: value.spell_list,
            hp_increases: value.hp_increases,
            feats: value.feats,
        };
        class.set_level(value.level)?;
        Ok(class)
    }
}

impl Class {
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn get_level(&self) -> usize {
        self.level
    }

    /// # Errors
    ///
    /// `LevelOutOfBounds`: when user tries to set lvl above 20
    ///
    pub fn set_level(&mut self, level: usize) -> Result<(), TryFromError> {
        if level > 20 {
            return Err(TryFromError::LevelOutOfBounds);
        }

        self.level = level;

        Ok(())
    }

    #[must_use]
    pub fn get_saving_throw_proficiency(&self, ability: Ability) -> Option<&Proficiency> {
        self.saving_throw_proficiencies.get(&ability)
    }

    pub fn get_spell_list(&self) -> Option<&SpellList> {
        self.spell_list.as_ref()
    }

    #[must_use]
    pub fn get_hit_points(&self, constitution_modifier: isize) -> usize {
        self.hp_increases.get_hit_points(constitution_modifier)
    }

    pub fn get_feats(&self) -> &[Feat] {
        &self.feats
    }

    pub fn add_feat(&mut self, feat: Feat) {
        self.feats.push(feat);
    }
}

#[derive(Debug)]
pub enum TryFromError {
    LevelOutOfBounds,
}

#[derive(Clone, Debug, Default)]
pub struct Classes(Vec<Class>);

impl Classes {
    pub fn add_class(&mut self, class: Class) {
        self.0.push(class);
    }

    pub fn get_level(&self) -> usize {
        self.0.iter().map(Class::get_level).sum()
    }

    #[must_use]
    pub fn get_proficiency_bonus(&self) -> usize {
        self.get_level().checked_sub(1).map_or(0, |r| r / 4 + 2)
    }

    #[must_use]
    pub fn get_saving_throw_proficiency(&self, ability: Ability) -> Option<&Proficiency> {
        self.0
            .first()
            .and_then(|primary_class| primary_class.get_saving_throw_proficiency(ability))
    }

    #[must_use]
    pub fn get_hit_points(&self, constitution_modifier: isize) -> usize {
        self.0.iter().fold(0, |acc, class| {
            class.get_hit_points(constitution_modifier) + acc
        })
    }

    pub fn get_feats(&self) -> Vec<&Feat> {
        self.0.iter().flat_map(|class| class.get_feats()).collect()
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
        #[must_use]
        pub fn wizard() -> Self {
            Class {
                name: "Wizard".into(),
                level: 1,
                saving_throw_proficiencies: HashMap::from([
                    (Ability::Intelligence, Proficiency::Proficiency),
                    (Ability::Wisdom, Proficiency::Proficiency),
                ]),
                spell_list: Some(SpellList::default()),
                hp_increases: HPIncreases::new(6),
                feats: vec![],
            }
        }

        #[must_use]
        pub fn artificer() -> Self {
            Class {
                name: "Artificer".into(),
                level: 1,
                saving_throw_proficiencies: HashMap::from([
                    (Ability::Intelligence, Proficiency::Proficiency),
                    (Ability::Constitution, Proficiency::Proficiency),
                ]),
                spell_list: Some(SpellList::default()),
                hp_increases: HPIncreases::new(8),
                feats: vec![],
            }
        }
    }

    mod hp_increase {
        use super::*;

        mod construct {
            use std::error::Error;

            use super::*;

            #[test]
            fn _default_should_be_empty() {
                let hpi = HPIncreases::default();

                assert!(hpi.0.is_empty(), "should be empty");
            }

            #[test]
            fn _new_should_contain_the_single_starting_hp() {
                let hpi = HPIncreases::new(10);

                assert_eq!(hpi.0, vec![10]);
            }

            #[test]
            fn _creating_from_vec_with_20_or_less_increases_should_be_successful(
            ) -> Result<(), Box<dyn Error>> {
                let hpi = HPIncreases::try_from(vec![1, 2, 3, 4])?;

                assert_eq!(hpi.0, vec![1, 2, 3, 4]);

                Ok(())
            }

            #[test]
            fn _create_from_vec_with_over_20_increases_should_result_in_error() {
                let hpi = HPIncreases::try_from(vec![
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                ]);

                assert!(hpi.is_err(), "should result in error");
            }
        }

        mod get_hit_points {
            use super::*;

            #[test]
            fn _should_return_0_on_empty_hp_increases_without_con_mod() {
                let hpi = HPIncreases::default();

                assert_eq!(hpi.get_hit_points(0), 0);
            }

            #[test]
            fn _should_return_0_on_empty_hp_increases_with_con_mod() {
                let hpi = HPIncreases::default();

                assert_eq!(hpi.get_hit_points(3), 0);
            }

            #[test]
            fn _should_sum_increases_without_con_mod() {
                let hpi = HPIncreases(vec![8, 5, 5, 5, 5]);

                assert_eq!(hpi.get_hit_points(0), 28);
            }

            #[test]
            fn _should_add_con_mod_to_each_increase_before_summing() {
                let hpi = HPIncreases(vec![8, 5, 5, 5, 5]);

                assert_eq!(hpi.get_hit_points(3), 43);
            }
        }

        mod add_increase {
            use std::error::Error;

            use super::*;

            #[test]
            fn _should_add_new_increase_to_collection() -> Result<(), Box<dyn Error>> {
                let mut hpi = HPIncreases::default();
                hpi.add_increase(3)?;

                assert_eq!(hpi.0, vec![3]);

                Ok(())
            }

            #[test]
            fn _should_add_increase_to_end() -> Result<(), Box<dyn Error>> {
                let mut hpi = HPIncreases(vec![1, 2, 3]);
                hpi.add_increase(4)?;

                assert_eq!(hpi.0, vec![1, 2, 3, 4]);

                Ok(())
            }

            #[test]
            fn _should_result_in_error_when_trying_to_add_more_than_20_increases() {
                let mut hpi = HPIncreases(vec![
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                ]);
                let result = hpi.add_increase(21);

                assert!(result.is_err(), "should result in error");
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
            spell_list: None,
            hp_increases: HPIncreases::default(),
            feats: vec![],
        }]);
        assert_eq!(lvl4.get_proficiency_bonus(), 2);

        let lvl5 = Classes(vec![Class {
            name: "lvl5".into(),
            level: 5,
            saving_throw_proficiencies: HashMap::new(),
            spell_list: None,
            hp_increases: HPIncreases::default(),
            feats: vec![],
        }]);
        assert_eq!(lvl5.get_proficiency_bonus(), 3);

        let lvl9 = Classes(vec![Class {
            name: "lvl9".into(),
            level: 9,
            saving_throw_proficiencies: HashMap::new(),
            spell_list: None,
            hp_increases: HPIncreases::default(),
            feats: vec![],
        }]);
        assert_eq!(lvl9.get_proficiency_bonus(), 4);

        let lvl13 = Classes(vec![Class {
            name: "lvl13".into(),
            level: 13,
            saving_throw_proficiencies: HashMap::new(),
            spell_list: None,
            hp_increases: HPIncreases::default(),
            feats: vec![],
        }]);
        assert_eq!(lvl13.get_proficiency_bonus(), 5);

        let lvl17 = Classes(vec![Class {
            name: "lvl17".into(),
            level: 17,
            saving_throw_proficiencies: HashMap::new(),
            spell_list: None,
            hp_increases: HPIncreases::default(),
            feats: vec![],
        }]);
        assert_eq!(lvl17.get_proficiency_bonus(), 6);
    }

    #[test]
    fn _classless_should_have_no_saving_throw_proficiencies() {
        let classless = Classes::default();

        assert_eq!(
            classless.get_saving_throw_proficiency(Ability::Strength),
            None
        );
        assert_eq!(
            classless.get_saving_throw_proficiency(Ability::Dexterity),
            None
        );
        assert_eq!(
            classless.get_saving_throw_proficiency(Ability::Constitution),
            None
        );
        assert_eq!(
            classless.get_saving_throw_proficiency(Ability::Intelligence),
            None
        );
        assert_eq!(
            classless.get_saving_throw_proficiency(Ability::Wisdom),
            None
        );
        assert_eq!(
            classless.get_saving_throw_proficiency(Ability::Charisma),
            None
        );
    }

    #[test]
    fn _monoclasses_should_derive_their_saving_throw_proficiencies_from_it() {
        let monoclass = Classes(vec![Class::artificer()]);

        assert_eq!(
            monoclass.get_saving_throw_proficiency(Ability::Strength),
            None
        );
        assert_eq!(
            monoclass.get_saving_throw_proficiency(Ability::Dexterity),
            None
        );
        assert_eq!(
            monoclass.get_saving_throw_proficiency(Ability::Constitution),
            Some(&Proficiency::Proficiency)
        );
        assert_eq!(
            monoclass.get_saving_throw_proficiency(Ability::Intelligence),
            Some(&Proficiency::Proficiency)
        );
        assert_eq!(
            monoclass.get_saving_throw_proficiency(Ability::Wisdom),
            None
        );
        assert_eq!(
            monoclass.get_saving_throw_proficiency(Ability::Charisma),
            None
        );
    }

    #[test]
    fn _multiclasses_should_only_inherit_proficiencies_from_first_class() {
        let multiclass = Classes(vec![Class::wizard(), Class::artificer()]);

        assert_eq!(
            multiclass.get_saving_throw_proficiency(Ability::Strength),
            None
        );
        assert_eq!(
            multiclass.get_saving_throw_proficiency(Ability::Dexterity),
            None
        );
        assert_eq!(
            multiclass.get_saving_throw_proficiency(Ability::Constitution),
            None
        );
        assert_eq!(
            multiclass.get_saving_throw_proficiency(Ability::Intelligence),
            Some(&Proficiency::Proficiency)
        );
        assert_eq!(
            multiclass.get_saving_throw_proficiency(Ability::Wisdom),
            Some(&Proficiency::Proficiency)
        );
        assert_eq!(
            multiclass.get_saving_throw_proficiency(Ability::Charisma),
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

    #[test]
    fn _should_return_feats_of_all_classes() {
        let sharpshooter = Feat::new(
            "Sharpshooter",
            "Double range distance and ignore half cover.",
        );
        let mut artificer = Class::artificer();
        artificer.add_feat(sharpshooter.clone());

        let war_caster = Feat::new("War Caster", "Can cast cantrip as attack of opportunity. Advantate on CON saving throws when concentrating.");
        let mut wizard = Class::wizard();
        wizard.add_feat(war_caster.clone());

        let multiclass = Classes(vec![artificer, wizard]);

        assert_eq!(multiclass.get_feats(), vec![&sharpshooter, &war_caster]);
    }
}
