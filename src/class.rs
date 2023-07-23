use crate::{ability::Ability, modifiers::Proficiency};

pub trait Class {
    fn get_level(&self) -> usize;

    fn get_saving_throw_proficiency(&self, ability: &Ability) -> Option<Proficiency>;
}

pub struct Classes(Vec<Box<dyn Class>>);

impl Classes {
    pub fn empty() -> Self {
        Classes(vec![])
    }

    pub fn add_class(&mut self, class: Box<dyn Class>) {
        self.0.push(class);
    }

    pub fn get_level(&self) -> usize {
        self.0.iter().fold(0, |acc, class| acc + class.get_level())
    }

    pub fn get_saving_throw_proficiency(&self, ability: &Ability) -> Option<Proficiency> {
        self.0
            .first()
            .and_then(|primary_class| primary_class.get_saving_throw_proficiency(ability))
    }

    pub fn get_proficiency_bonus(&self) -> usize {
        self.get_level()
            .checked_sub(1)
            .map(|r| r / 4 + 2)
            .unwrap_or(0)
    }

    pub fn get_saving_throw_bonus(&self, ability: &Ability) -> usize {
        self.get_proficiency_bonus()
            * (self
                .get_saving_throw_proficiency(ability)
                .map(|p| p as usize)
                .unwrap_or(0))
    }
}

pub struct Artificer {
    pub level: usize,
}

impl Class for Artificer {
    fn get_level(&self) -> usize {
        self.level
    }

    fn get_saving_throw_proficiency(&self, ability: &Ability) -> Option<Proficiency> {
        match ability {
            Ability::Constitution | Ability::Intelligence => Some(Proficiency::Proficiency),
            _ => None,
        }
    }
}

pub struct Wizard {
    pub level: usize,
}

impl Class for Wizard {
    fn get_level(&self) -> usize {
        self.level
    }

    fn get_saving_throw_proficiency(&self, ability: &Ability) -> Option<Proficiency> {
        match ability {
            Ability::Intelligence | Ability::Wisdom => Some(Proficiency::Proficiency),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _classless_should_be_level_0() {
        assert_eq!(Classes::empty().get_level(), 0);
    }

    #[test]
    fn _monoclasses_should_be_the_class_level() {
        let classes = Classes(vec![Box::new(Artificer { level: 1 })]);

        assert_eq!(classes.get_level(), 1);

        let classes = Classes(vec![Box::new(Artificer { level: 20 })]);

        assert_eq!(classes.get_level(), 20);
    }

    #[test]
    fn _multiclasses_should_sum_classes_levels() {
        let classes = Classes(vec![
            Box::new(Artificer { level: 1 }),
            Box::new(Wizard { level: 1 }),
        ]);

        assert_eq!(classes.get_level(), 2);
    }

    #[test]
    fn _classless_should_have_0_proficiency_bonus() {
        let classes = Classes::empty();

        assert_eq!(classes.get_proficiency_bonus(), 0);
    }

    #[test]
    fn _level_1_character_should_have_proficiency_bonus_of_2() {
        let classes = Classes(vec![Box::new(Artificer { level: 1 })]);

        assert_eq!(classes.get_proficiency_bonus(), 2);
    }

    #[test]
    fn _proficiency_bonus_should_go_up_by_1_every_4_level_ups() {
        let classes = Classes(vec![Box::new(Artificer { level: 4 })]);
        assert_eq!(classes.get_proficiency_bonus(), 2);

        let classes = Classes(vec![Box::new(Artificer { level: 5 })]);
        assert_eq!(classes.get_proficiency_bonus(), 3);

        let classes = Classes(vec![Box::new(Artificer { level: 9 })]);
        assert_eq!(classes.get_proficiency_bonus(), 4);

        let classes = Classes(vec![Box::new(Artificer { level: 13 })]);
        assert_eq!(classes.get_proficiency_bonus(), 5);

        let classes = Classes(vec![Box::new(Artificer { level: 17 })]);
        assert_eq!(classes.get_proficiency_bonus(), 6);
    }

    #[test]
    fn _classless_should_have_no_saving_throw_proficiencies() {
        let classes = Classes::empty();

        assert_eq!(
            classes.get_saving_throw_proficiency(&Ability::Strength),
            None
        );
        assert_eq!(
            classes.get_saving_throw_proficiency(&Ability::Dexterity),
            None
        );
        assert_eq!(
            classes.get_saving_throw_proficiency(&Ability::Constitution),
            None
        );
        assert_eq!(
            classes.get_saving_throw_proficiency(&Ability::Intelligence),
            None
        );
        assert_eq!(classes.get_saving_throw_proficiency(&Ability::Wisdom), None);
        assert_eq!(
            classes.get_saving_throw_proficiency(&Ability::Charisma),
            None
        );
    }

    #[test]
    fn _monoclasses_should_derive_their_saving_throw_proficiencies_from_it() {
        let classes = Classes(vec![Box::new(Artificer { level: 1 })]);

        assert_eq!(
            classes.get_saving_throw_proficiency(&Ability::Strength),
            None
        );
        assert_eq!(
            classes.get_saving_throw_proficiency(&Ability::Dexterity),
            None
        );
        assert_eq!(
            classes.get_saving_throw_proficiency(&Ability::Constitution),
            Some(Proficiency::Proficiency)
        );
        assert_eq!(
            classes.get_saving_throw_proficiency(&Ability::Intelligence),
            Some(Proficiency::Proficiency)
        );
        assert_eq!(classes.get_saving_throw_proficiency(&Ability::Wisdom), None);
        assert_eq!(
            classes.get_saving_throw_proficiency(&Ability::Charisma),
            None
        );
    }

    #[test]
    fn _multiclasses_should_only_inherit_proficiencies_from_first_class() {
        let classes = Classes(vec![
            Box::new(Wizard { level: 1 }),
            Box::new(Artificer { level: 1 }),
        ]);

        assert_eq!(
            classes.get_saving_throw_proficiency(&Ability::Strength),
            None
        );
        assert_eq!(
            classes.get_saving_throw_proficiency(&Ability::Dexterity),
            None
        );
        assert_eq!(
            classes.get_saving_throw_proficiency(&Ability::Constitution),
            None
        );
        assert_eq!(
            classes.get_saving_throw_proficiency(&Ability::Intelligence),
            Some(Proficiency::Proficiency)
        );
        assert_eq!(
            classes.get_saving_throw_proficiency(&Ability::Wisdom),
            Some(Proficiency::Proficiency)
        );
        assert_eq!(
            classes.get_saving_throw_proficiency(&Ability::Charisma),
            None
        );
    }
}
