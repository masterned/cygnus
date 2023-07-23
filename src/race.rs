use std::{error::Error, fmt};

use crate::{ability::Ability, modifiers::Resistance};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DamageType {
    Necrotic,
    Radiant,
    Poison,
    Force,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub enum AbilityScoreIncrease {
    TwoOne(Ability, Ability),
    OneOneOne(Ability, Ability, Ability),
    AllOne,
    TwoOneOne(Ability, Ability, Ability),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbilityScoreIncreaseError {
    DuplicateAbilities,
}

impl fmt::Display for AbilityScoreIncreaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AbilityScoreIncreaseError::DuplicateAbilities => {
                write!(f, "Attempted to duplicate an ability score.")
            }
        }
    }
}

impl Error for AbilityScoreIncreaseError {}

impl AbilityScoreIncrease {
    pub fn get_delta_of(&self, ability: &Ability) -> usize {
        match self {
            AbilityScoreIncrease::TwoOne(two, one) => {
                if two == ability {
                    2
                } else if one == ability {
                    1
                } else {
                    0
                }
            }
            AbilityScoreIncrease::OneOneOne(a, b, c) => {
                if a == ability || b == ability || c == ability {
                    1
                } else {
                    0
                }
            }
            AbilityScoreIncrease::AllOne => 1,
            AbilityScoreIncrease::TwoOneOne(two, one_a, one_b) => {
                if two == ability {
                    2
                } else if one_a == ability || one_b == ability {
                    1
                } else {
                    0
                }
            }
        }
    }

    pub fn new_two_one(two: Ability, one: Ability) -> Result<Self, AbilityScoreIncreaseError> {
        if two == one {
            return Err(AbilityScoreIncreaseError::DuplicateAbilities);
        }

        Ok(AbilityScoreIncrease::TwoOne(two, one))
    }

    pub fn new_one_one_one(
        a: Ability,
        b: Ability,
        c: Ability,
    ) -> Result<Self, AbilityScoreIncreaseError> {
        if a == b || a == c || b == c {
            return Err(AbilityScoreIncreaseError::DuplicateAbilities);
        }

        Ok(AbilityScoreIncrease::OneOneOne(a, b, c))
    }

    pub fn new_all_one() -> Self {
        AbilityScoreIncrease::AllOne
    }

    pub fn new_two_one_one(
        two: Ability,
        one_a: Ability,
        one_b: Ability,
    ) -> Result<Self, AbilityScoreIncreaseError> {
        if two == one_a || two == one_b || one_a == one_b {
            return Err(AbilityScoreIncreaseError::DuplicateAbilities);
        }

        Ok(AbilityScoreIncrease::TwoOneOne(two, one_a, one_b))
    }
}

pub trait Race {
    fn get_creature_type(&self) -> CreatureType;

    fn get_size(&self) -> Size;

    fn get_walking_speed(&self) -> usize;

    fn get_ability_score_bonus(&self, ability: &Ability) -> usize;

    fn get_damage_resistance(&self, damage_type: &DamageType) -> Option<Resistance>;

    fn get_condition_resistance(&self, contition: &Condition) -> Option<Resistance>;
}

pub struct Human {
    ability_score_increase: AbilityScoreIncrease,
}

impl Default for Human {
    fn default() -> Self {
        Human {
            ability_score_increase: AbilityScoreIncrease::new_all_one(),
        }
    }
}

impl Race for Human {
    fn get_size(&self) -> Size {
        Size::Medium
    }

    fn get_walking_speed(&self) -> usize {
        30
    }

    fn get_ability_score_bonus(&self, ability: &Ability) -> usize {
        self.ability_score_increase.get_delta_of(ability)
    }

    fn get_creature_type(&self) -> CreatureType {
        CreatureType::Humanoid
    }

    fn get_damage_resistance(&self, _damage_type: &DamageType) -> Option<Resistance> {
        None
    }

    fn get_condition_resistance(&self, _condition: &Condition) -> Option<Resistance> {
        None
    }
}

pub struct ShadarKai {
    ability_score_increase: AbilityScoreIncrease,
}

impl ShadarKai {
    pub fn new_two_one(two: Ability, one: Ability) -> Result<Self, Box<dyn Error>> {
        let ability_score_increase = AbilityScoreIncrease::new_two_one(two, one)?;

        Ok(ShadarKai {
            ability_score_increase,
        })
    }

    pub fn new_one_one_one(a: Ability, b: Ability, c: Ability) -> Result<Self, Box<dyn Error>> {
        let ability_score_increase = AbilityScoreIncrease::new_one_one_one(a, b, c)?;

        Ok(ShadarKai {
            ability_score_increase,
        })
    }
}

impl Race for ShadarKai {
    fn get_creature_type(&self) -> CreatureType {
        CreatureType::Humanoid
    }

    fn get_size(&self) -> Size {
        Size::Medium
    }

    fn get_walking_speed(&self) -> usize {
        30
    }

    fn get_ability_score_bonus(&self, ability: &Ability) -> usize {
        self.ability_score_increase.get_delta_of(ability)
    }

    fn get_damage_resistance(&self, damage_type: &DamageType) -> Option<Resistance> {
        match damage_type {
            DamageType::Necrotic => Some(Resistance::Resistant),
            _ => None,
        }
    }

    fn get_condition_resistance(&self, condition: &Condition) -> Option<Resistance> {
        match condition {
            Condition::MagicalSleep => Some(Resistance::Immune),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod ability_score_increase {
        use super::*;

        mod constructors {
            use super::*;

            #[test]
            fn _two_one_should_prevent_duplicates() {
                let asi = AbilityScoreIncrease::new_two_one(Ability::Strength, Ability::Dexterity);

                assert_eq!(
                    asi,
                    Ok(AbilityScoreIncrease::TwoOne(
                        Ability::Strength,
                        Ability::Dexterity
                    ))
                );

                let failed_asi =
                    AbilityScoreIncrease::new_two_one(Ability::Strength, Ability::Strength);

                assert_eq!(
                    failed_asi,
                    Err(AbilityScoreIncreaseError::DuplicateAbilities)
                );
            }

            #[test]
            fn _one_one_one_should_prevent_duplicates() {
                let asi = AbilityScoreIncrease::new_one_one_one(
                    Ability::Strength,
                    Ability::Dexterity,
                    Ability::Constitution,
                );

                assert_eq!(
                    asi,
                    Ok(AbilityScoreIncrease::OneOneOne(
                        Ability::Strength,
                        Ability::Dexterity,
                        Ability::Constitution
                    ))
                );

                let failed_asi = AbilityScoreIncrease::new_one_one_one(
                    Ability::Strength,
                    Ability::Strength,
                    Ability::Strength,
                );

                assert_eq!(
                    failed_asi,
                    Err(AbilityScoreIncreaseError::DuplicateAbilities)
                );

                let failed_asi = AbilityScoreIncrease::new_one_one_one(
                    Ability::Strength,
                    Ability::Strength,
                    Ability::Dexterity,
                );

                assert_eq!(
                    failed_asi,
                    Err(AbilityScoreIncreaseError::DuplicateAbilities)
                );

                let failed_asi = AbilityScoreIncrease::new_one_one_one(
                    Ability::Strength,
                    Ability::Dexterity,
                    Ability::Strength,
                );

                assert_eq!(
                    failed_asi,
                    Err(AbilityScoreIncreaseError::DuplicateAbilities)
                );

                let failed_asi = AbilityScoreIncrease::new_one_one_one(
                    Ability::Dexterity,
                    Ability::Strength,
                    Ability::Strength,
                );

                assert_eq!(
                    failed_asi,
                    Err(AbilityScoreIncreaseError::DuplicateAbilities)
                );
            }

            #[test]
            fn _two_one_one_should_prevent_duplicates() {
                let asi = AbilityScoreIncrease::new_two_one_one(
                    Ability::Strength,
                    Ability::Dexterity,
                    Ability::Constitution,
                );

                assert_eq!(
                    asi,
                    Ok(AbilityScoreIncrease::TwoOneOne(
                        Ability::Strength,
                        Ability::Dexterity,
                        Ability::Constitution
                    ))
                );

                let failed_asi = AbilityScoreIncrease::new_two_one_one(
                    Ability::Strength,
                    Ability::Strength,
                    Ability::Strength,
                );

                assert_eq!(
                    failed_asi,
                    Err(AbilityScoreIncreaseError::DuplicateAbilities)
                );

                let failed_asi = AbilityScoreIncrease::new_two_one_one(
                    Ability::Strength,
                    Ability::Strength,
                    Ability::Dexterity,
                );

                assert_eq!(
                    failed_asi,
                    Err(AbilityScoreIncreaseError::DuplicateAbilities)
                );

                let failed_asi = AbilityScoreIncrease::new_two_one_one(
                    Ability::Strength,
                    Ability::Dexterity,
                    Ability::Strength,
                );

                assert_eq!(
                    failed_asi,
                    Err(AbilityScoreIncreaseError::DuplicateAbilities)
                );

                let failed_asi = AbilityScoreIncrease::new_two_one_one(
                    Ability::Dexterity,
                    Ability::Strength,
                    Ability::Strength,
                );

                assert_eq!(
                    failed_asi,
                    Err(AbilityScoreIncreaseError::DuplicateAbilities)
                );
            }
        }

        mod get_delta_of {
            use super::*;

            #[test]
            fn _all_one_should_always_return_1() {
                let asi = AbilityScoreIncrease::AllOne;

                assert_eq!(asi.get_delta_of(&Ability::Strength), 1);
                assert_eq!(asi.get_delta_of(&Ability::Dexterity), 1);
                assert_eq!(asi.get_delta_of(&Ability::Constitution), 1);
                assert_eq!(asi.get_delta_of(&Ability::Intelligence), 1);
                assert_eq!(asi.get_delta_of(&Ability::Wisdom), 1);
                assert_eq!(asi.get_delta_of(&Ability::Charisma), 1);
            }

            #[test]
            fn _one_one_one_should_return_1_on_three_different_abilities_and_0_on_others() {
                let asi = AbilityScoreIncrease::OneOneOne(
                    Ability::Strength,
                    Ability::Dexterity,
                    Ability::Constitution,
                );

                assert_eq!(asi.get_delta_of(&Ability::Strength), 1);
                assert_eq!(asi.get_delta_of(&Ability::Dexterity), 1);
                assert_eq!(asi.get_delta_of(&Ability::Constitution), 1);
                assert_eq!(asi.get_delta_of(&Ability::Intelligence), 0);
                assert_eq!(asi.get_delta_of(&Ability::Wisdom), 0);
                assert_eq!(asi.get_delta_of(&Ability::Charisma), 0);
            }

            #[test]
            fn _two_one_should_return_2_on_one_ability_1_on_another_and_0_elsewhere() {
                let asi = AbilityScoreIncrease::TwoOne(Ability::Strength, Ability::Dexterity);

                assert_eq!(asi.get_delta_of(&Ability::Strength), 2);
                assert_eq!(asi.get_delta_of(&Ability::Dexterity), 1);
                assert_eq!(asi.get_delta_of(&Ability::Constitution), 0);
                assert_eq!(asi.get_delta_of(&Ability::Intelligence), 0);
                assert_eq!(asi.get_delta_of(&Ability::Wisdom), 0);
                assert_eq!(asi.get_delta_of(&Ability::Charisma), 0);
            }

            #[test]
            fn _two_one_one_should_return_2_on_one_ability_1_on_another_1_on_a_third_and_0_elsewhere(
            ) {
                let asi = AbilityScoreIncrease::TwoOneOne(
                    Ability::Strength,
                    Ability::Dexterity,
                    Ability::Constitution,
                );

                assert_eq!(asi.get_delta_of(&Ability::Strength), 2);
                assert_eq!(asi.get_delta_of(&Ability::Dexterity), 1);
                assert_eq!(asi.get_delta_of(&Ability::Constitution), 1);
                assert_eq!(asi.get_delta_of(&Ability::Intelligence), 0);
                assert_eq!(asi.get_delta_of(&Ability::Wisdom), 0);
                assert_eq!(asi.get_delta_of(&Ability::Charisma), 0);
            }
        }
    }
}
