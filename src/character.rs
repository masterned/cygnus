use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    ability::{Abilities, Ability},
    class::Class,
    item::{Item, Items},
    modifiers::{Encumbrance, Proficiency},
    race::{CreatureType, Race, Size},
    skill::{Skill, Skills},
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
    pub classes: Vec<Class>,
    pub skills: Skills,
    pub items: Items,
    pub exhaustion_level: usize,
}

impl Character {
    pub fn get_creature_type(&self) -> &CreatureType {
        self.race.get_creature_type()
    }

    pub fn get_size(&self) -> &Size {
        self.race.get_size()
    }

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

    pub fn get_ability_score(&self, ability: &Ability) -> usize {
        self.abilities.get_base_score(ability) + self.race.get_ability_score_bonus(ability)
    }

    pub fn get_ability_modifier(&self, ability: &Ability) -> isize {
        Ability::calculate_modifier(self.get_ability_score(ability))
    }

    pub fn get_level(&self) -> usize {
        self.classes.iter().map(|class| class.get_level()).sum()
    }

    pub fn get_proficiency_bonus(&self) -> usize {
        self.get_level()
            .checked_sub(1)
            .map(|r| r / 4 + 2)
            .unwrap_or(0)
    }

    pub fn get_saving_throw_proficiency(&self, ability: &Ability) -> Option<&Proficiency> {
        self.classes
            .first()
            .and_then(|primary_class| primary_class.get_saving_throw_proficiency(ability))
    }

    pub fn get_saving_throw_mod(&self, ability: &Ability) -> isize {
        self.get_proficiency_bonus() as isize
            * (self
                .get_saving_throw_proficiency(ability)
                .map(|&p| p as isize)
                .unwrap_or(0))
            + self.get_ability_modifier(ability)
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

    pub fn get_exhaustion_level(&self) -> usize {
        self.exhaustion_level
    }

    pub fn set_exhaustion_level(&mut self, new_level: usize) {
        self.exhaustion_level = new_level;
    }

    pub fn get_skill_modifier(&self, skill: &Skill) -> isize {
        self.get_ability_modifier(&skill.get_ability())
            + (match self.skills.get_proficiency(skill) {
                Some(Proficiency::Proficiency) => self.get_proficiency_bonus(),
                Some(Proficiency::Expertise) => self.get_proficiency_bonus() * 2,
                None => 0,
            }) as isize
    }

    pub fn get_passive_perception(&self) -> usize {
        (10 + self.get_skill_modifier(&Skill::Perception)) as usize
    }

    pub fn get_passive_investigation(&self) -> usize {
        (10 + self.get_skill_modifier(&Skill::Investigation)) as usize
    }

    pub fn get_passive_insight(&self) -> usize {
        (10 + self.get_skill_modifier(&Skill::Insight)) as usize
    }

    pub fn add_class(&mut self, class: Class) {
        self.classes.push(class);
    }

    pub fn render_tui<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(70),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(area);

        let name_block = Paragraph::new(self.name.clone())
            .block(Block::default().title("Name").borders(Borders::ALL));
        f.render_widget(name_block, chunks[0]);

        let block = Block::default().title("Abilities").borders(Borders::ALL);
        f.render_widget(block.clone(), chunks[1]);

        self.abilities.render_tui(f, block.inner(chunks[1]));

        let block = Block::default().title("Body").borders(Borders::ALL);
        f.render_widget(block, chunks[2]);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::class::ClassTemplate;

    use super::*;

    impl Character {
        fn dummy() -> Self {
            Self {
                name: "Dummy".into(),
                alignment: (Conformity::Neutral, Morality::Neutral),
                gender: None,
                abilities: Abilities::default(),
                race: Race::human(),
                classes: vec![],
                personality: Personality {
                    personality_traits: vec![],
                    ideals: vec![],
                    bonds: vec![],
                    flaws: vec![],
                },
                skills: Skills::default(),
                items: Items::default(),
                exhaustion_level: 0,
            }
        }
    }

    #[test]
    fn _should_default_character_creature_type_to_race_creature_type() {
        let character = Character::dummy();

        assert_eq!(character.get_creature_type(), &CreatureType::Humanoid);
    }

    #[test]
    fn _classless_should_have_no_saving_throw_proficiencies() {
        let character = Character::dummy();

        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Strength),
            None
        );
        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Dexterity),
            None
        );
        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Constitution),
            None
        );
        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Intelligence),
            None
        );
        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Wisdom),
            None
        );
        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Charisma),
            None
        );
    }

    #[test]
    fn _monoclasses_should_derive_their_saving_throw_proficiencies_from_it() {
        let mut character = Character::dummy();
        character.add_class(Class::artificer());

        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Strength),
            None
        );
        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Dexterity),
            None
        );
        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Constitution),
            Some(&Proficiency::Proficiency)
        );
        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Intelligence),
            Some(&Proficiency::Proficiency)
        );
        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Wisdom),
            None
        );
        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Charisma),
            None
        );
    }

    #[test]
    fn _multiclasses_should_only_inherit_proficiencies_from_first_class() {
        let mut character = Character::dummy();
        character.add_class(Class::wizard());
        character.add_class(Class::artificer());

        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Strength),
            None
        );
        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Dexterity),
            None
        );
        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Constitution),
            None
        );
        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Intelligence),
            Some(&Proficiency::Proficiency)
        );
        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Wisdom),
            Some(&Proficiency::Proficiency)
        );
        assert_eq!(
            character.get_saving_throw_proficiency(&Ability::Charisma),
            None
        );
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
        character.add_item(Item::new(46));

        assert_eq!(
            character.get_variant_encumbrance(),
            Some(Encumbrance::Encumbered)
        );
    }

    #[test]
    fn _characters_with_more_than_10_times_str_score_in_item_weight_should_be_heavily_encumbered() {
        let mut character = Character::dummy();
        character.add_item(Item::new(91));

        assert_eq!(
            character.get_variant_encumbrance(),
            Some(Encumbrance::HeavilyEncumbered)
        );
    }

    #[test]
    fn _encumbered_characters_should_reduce_their_speed_by_10() {
        let mut character = Character::dummy();
        character.add_item(Item::new(46));

        assert_eq!(character.get_walking_speed(), 20);
    }

    #[test]
    fn _heavily_encumbered_characters_should_reduce_their_speed_by_20() {
        let mut character = Character::dummy();
        character.add_item(Item::new(91));

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
    fn _classless_should_be_level_0() {
        let character = Character::dummy();

        assert_eq!(character.get_level(), 0);
    }

    #[test]
    fn _monoclasses_should_be_the_class_level() {
        let mut character = Character::dummy();
        character.add_class(Class::artificer());

        assert_eq!(character.get_level(), 1);

        let mut artificer = Class::artificer();
        artificer.set_level(20).unwrap();
        character.classes = vec![artificer];

        assert_eq!(character.get_level(), 20);
    }

    #[test]
    fn _multiclasses_should_sum_classes_levels() {
        let mut character = Character::dummy();
        character.add_class(Class::artificer());
        character.add_class(Class::wizard());

        assert_eq!(character.get_level(), 2);
    }

    #[test]
    fn _level_0_character_should_not_have_proficiency_bonus() {
        let character = Character::dummy();

        assert_eq!(character.get_proficiency_bonus(), 0);
    }

    #[test]
    fn _level_1_character_should_have_proficiency_bonus_of_2() {
        let mut character = Character::dummy();
        character.add_class(Class::artificer());

        assert_eq!(character.get_proficiency_bonus(), 2);
    }

    #[test]
    fn _proficiency_bonus_should_go_up_by_1_every_4_level_ups() {
        let mut character = Character::dummy();

        let lvl4 = Class::try_from(ClassTemplate {
            name: "lvl4".into(),
            level: 4,
            saving_throw_proficiencies: HashMap::new(),
        })
        .unwrap();
        character.classes = vec![lvl4];
        assert_eq!(character.get_proficiency_bonus(), 2);

        let lvl5 = Class::try_from(ClassTemplate {
            name: "lvl5".into(),
            level: 5,
            saving_throw_proficiencies: HashMap::new(),
        })
        .unwrap();
        character.classes = vec![lvl5];
        assert_eq!(character.get_proficiency_bonus(), 3);

        let lvl9 = Class::try_from(ClassTemplate {
            name: "lvl9".into(),
            level: 9,
            saving_throw_proficiencies: HashMap::new(),
        })
        .unwrap();
        character.classes = vec![lvl9];
        assert_eq!(character.get_proficiency_bonus(), 4);

        let lvl13 = Class::try_from(ClassTemplate {
            name: "lvl13".into(),
            level: 13,
            saving_throw_proficiencies: HashMap::new(),
        })
        .unwrap();
        character.classes = vec![lvl13];
        assert_eq!(character.get_proficiency_bonus(), 5);

        let lvl17 = Class::try_from(ClassTemplate {
            name: "lvl17".into(),
            level: 17,
            saving_throw_proficiencies: HashMap::new(),
        })
        .unwrap();
        character.classes = vec![lvl17];
        assert_eq!(character.get_proficiency_bonus(), 6);
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
}
