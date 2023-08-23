use crate::{
    ability::Ability, dice::Roll, race::DamageType, utils::lower_bound_map::LowerBoundMap,
};

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttackKind {
    Save {
        ability: Ability,
    },
    Melee {
        additional_weapon_damage: LowerBoundMap<usize, (Roll, DamageType)>,
    },
    Ranged,
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
    attack_kind: Option<AttackKind>,
    effect: Effect,
    description: String,
    damage_rolls: LowerBoundMap<usize, Roll>,
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
    pub fn get_attack_kind(&self) -> Option<&AttackKind> {
        self.attack_kind.as_ref()
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

    pub fn get_damage_roll(&self, level: usize) -> Option<&Roll> {
        self.damage_rolls.get(&level)
    }
}

#[derive(Debug, Default)]
pub struct SpellList(Vec<Spell>);

impl From<Vec<Spell>> for SpellList {
    fn from(value: Vec<Spell>) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Spell {
        fn _fire_bolt() -> Self {
            Spell {
                name: "fire bolt".into(),
                level: 0,
                casting_time: CastingTime::Action(1),
                range: Range::Feet(120),
                components: vec![Component::Verbal, Component::Somatic],
                duration: Duration::Instantaneous,
                school: School::Evocation,
                concentration: false,
                attack_kind: Some(AttackKind::Ranged),
                effect: Effect::Fire,
                description: "Say cheese!".into(),
                damage_rolls: LowerBoundMap::from([
                    (0, Roll::new(1, 10, 0)),
                    (5, Roll::new(2, 10, 0)),
                    (11, Roll::new(3, 10, 0)),
                    (17, Roll::new(4, 10, 0)),
                ]),
            }
        }

        fn _fireball() -> Self {
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
                attack_kind: Some(AttackKind::Save {
                    ability: Ability::Dexterity,
                }),
                effect: Effect::Fire,
                description: "EXPLOSION!!!".into(),
                damage_rolls: LowerBoundMap::from([
                    (3, Roll::new(6, 8, 0)),
                    (4, Roll::new(7, 8, 0)),
                    (5, Roll::new(8, 8, 0)),
                    (6, Roll::new(9, 8, 0)),
                    (7, Roll::new(10, 8, 0)),
                    (8, Roll::new(11, 8, 0)),
                    (9, Roll::new(12, 8, 0)),
                ]),
            }
        }
    }
}
