use crate::{ability::Ability, dice::DiceRoll};

pub enum Spell {
    Attack {
        name: String,
        description: String,
        dice_roll: DiceRoll,
    },
    Save {
        name: String,
        description: String,
        ability: Ability,
    },
}
