use std::collections::HashMap;

use crate::{ability::Ability, dice::Roll};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CastingTime {
    Action(usize),
    Reaction(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range {
    Cone,
    Cube,
    Cylinder,
    Feet(usize),
    Line,
    Sphere { distance: usize, radius: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Component {
    Verbal,
    Somatic,
    Material,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Duration {
    Instantaneous,
    Rounds(usize),
    Minutes(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum School {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttackType {
    Ranged,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attack {
    Save(Ability),
    Attack(AttackType),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect {
    Acid,
    Buff,
    Cold,
    Combat,
    Detection,
    Fire,
}

#[derive(Debug)]
pub struct Spell {
    name: String,
    level: usize,
    casting_time: CastingTime,
    range: Range,
    components: Vec<Component>,
    duration: Duration,
    concentration: bool,
    school: School,
    attack: Option<Attack>,
    effect: Effect,
    description: String,
    damages: HashMap<usize, Roll>,
}

impl Spell {
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn get_level(&self) -> usize {
        self.level
    }

    #[must_use]
    pub fn get_casting_time(&self) -> CastingTime {
        self.casting_time
    }

    #[must_use]
    pub fn get_range(&self) -> Range {
        self.range
    }

    #[must_use]
    pub fn get_components(&self) -> &[Component] {
        &self.components
    }

    #[must_use]
    pub fn get_duration(&self) -> Duration {
        self.duration
    }

    #[must_use]
    pub fn get_school(&self) -> School {
        self.school
    }

    #[must_use]
    pub fn get_attack(&self) -> Option<Attack> {
        self.attack
    }

    #[must_use]
    pub fn get_effect(&self) -> Effect {
        self.effect
    }

    #[must_use]
    pub fn is_concentration(&self) -> bool {
        self.concentration
    }

    #[must_use]
    pub fn get_description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub fn get_damage(&self, cast_level: usize) -> Option<&Roll> {
        match self.level {
            0 => self.damages.get(&((cast_level + 1) / 6)),
            lvl => {
                if cast_level < lvl {
                    None
                } else {
                    self.damages.get(&((cast_level - lvl) % self.damages.len()))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Spell {
        fn fire_bolt() -> Self {
            Spell {
                name: "fire bolt".into(),
                level: 0,
                casting_time: CastingTime::Action(1),
                range: Range::Feet(120),
                components: vec![Component::Verbal, Component::Somatic],
                duration: Duration::Instantaneous,
                school: School::Evocation,
                concentration: false,
                attack: Some(Attack::Attack(AttackType::Ranged)),
                effect: Effect::Fire,
                description: "Say cheese!".into(),
                damages: HashMap::from([
                    (0, Roll::new(1, 10, 0)),
                    (1, Roll::new(2, 10, 0)),
                    (2, Roll::new(3, 10, 0)),
                    (3, Roll::new(4, 10, 0)),
                ]),
            }
        }

        fn fireball() -> Self {
            Spell {
                name: "fireball".into(),
                level: 3,
                casting_time: CastingTime::Action(1),
                range: Range::Sphere {
                    distance: 150,
                    radius: 20,
                },
                components: vec![Component::Verbal, Component::Somatic, Component::Material],
                duration: Duration::Instantaneous,
                concentration: false,
                school: School::Evocation,
                attack: Some(Attack::Save(Ability::Dexterity)),
                effect: Effect::Fire,
                description: "EXPLOSION!!!".into(),
                damages: HashMap::from([
                    (0, Roll::new(6, 8, 0)),
                    (1, Roll::new(7, 8, 0)),
                    (2, Roll::new(8, 8, 0)),
                    (3, Roll::new(9, 8, 0)),
                    (4, Roll::new(10, 8, 0)),
                    (5, Roll::new(11, 8, 0)),
                    (6, Roll::new(12, 8, 0)),
                ]),
            }
        }
    }

    #[test]
    fn _cantrip_damage_should_improve_on_casters_level() {
        let fire_bolt = Spell::fire_bolt();

        assert_eq!(fire_bolt.get_damage(4), Some(&Roll::new(1, 10, 0)));

        assert_eq!(fire_bolt.get_damage(5), Some(&Roll::new(2, 10, 0)));

        assert_eq!(fire_bolt.get_damage(10), Some(&Roll::new(2, 10, 0)));

        assert_eq!(fire_bolt.get_damage(11), Some(&Roll::new(3, 10, 0)));

        assert_eq!(fire_bolt.get_damage(16), Some(&Roll::new(3, 10, 0)));

        assert_eq!(fire_bolt.get_damage(17), Some(&Roll::new(4, 10, 0)));
    }

    #[test]
    fn _undercasting_a_leveled_spell_should_return_none_damge() {
        let fireball = Spell::fireball();

        assert_eq!(fireball.get_damage(1), None);
    }

    #[test]
    fn _leveled_spell_should_improve_damage_on_upcasting() {
        let fireball = Spell::fireball();

        assert_eq!(fireball.get_damage(3), Some(&Roll::new(6, 8, 0)));

        assert_eq!(fireball.get_damage(4), Some(&Roll::new(7, 8, 0)));

        assert_eq!(fireball.get_damage(5), Some(&Roll::new(8, 8, 0)));

        assert_eq!(fireball.get_damage(6), Some(&Roll::new(9, 8, 0)));

        assert_eq!(fireball.get_damage(7), Some(&Roll::new(10, 8, 0)));

        assert_eq!(fireball.get_damage(8), Some(&Roll::new(11, 8, 0)));

        assert_eq!(fireball.get_damage(9), Some(&Roll::new(12, 8, 0)));
    }
}
