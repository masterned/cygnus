use std::{collections::HashMap, error, fmt};

use crate::item::Item;

#[derive(Debug)]
pub struct Slot<T, F>
where
    F: Fn(&T) -> bool,
{
    value: Option<T>,
    validator: F,
}

impl<T, F> Slot<T, F>
where
    F: Fn(&T) -> bool,
{
    pub fn new(validator: F) -> Self {
        Self {
            value: None,
            validator,
        }
    }

    pub fn equip(&mut self, value: T) -> SlotResult<()> {
        if self.value.is_some() {
            return Err(SlotError::Full);
        }

        if !(self.validator)(&value) {
            return Err(SlotError::Invalid);
        }

        self.value = Some(value);

        Ok(())
    }

    pub fn unequip(&mut self) -> SlotResult<T> {
        self.value.take().ok_or(SlotError::Empty)
    }
}

#[derive(Debug)]
pub enum SlotError {
    Full,
    Empty,
    Invalid,
}

impl fmt::Display for SlotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            SlotError::Full => "Slot already contains something.".to_owned(),
            SlotError::Empty => "Cannot remove something from empty Slot.".to_owned(),
            SlotError::Invalid => "Attempted to equip invalid value.".into(),
        };

        write!(f, "{result}")
    }
}

impl error::Error for SlotError {}

pub type SlotResult<T> = Result<T, SlotError>;

type ItemSlot = Slot<Item, fn(&Item) -> bool>;

#[derive(Debug, Default)]
pub struct ItemSlots(HashMap<String, ItemSlot>);

impl ItemSlots {
    pub fn add_slot(&mut self, slot_name: impl Into<String>, slot: Slot<Item, fn(&Item) -> bool>) {
        self.0.insert(slot_name.into(), slot);
    }

    pub fn equip(&mut self, item: Item, slot_name: impl Into<String>) -> SlotsResult<()> {
        let slot_name = slot_name.into();

        self.0
            .get_mut(&slot_name)
            .ok_or(SlotsError::NotExists { slot: slot_name })
            .and_then(|slot| slot.equip(item).map_err(Into::<SlotsError>::into))
    }

    pub fn unequip(&mut self, slot_name: impl Into<String>) -> SlotsResult<Item> {
        let slot_name = slot_name.into();

        self.0
            .get_mut(&slot_name)
            .ok_or(SlotsError::NotExists { slot: slot_name })
            .and_then(|slot| slot.unequip().map_err(Into::<SlotsError>::into))
    }

    pub fn has_item_equipped_matching_criteria(&self, item_criteria: fn(&Item) -> bool) -> bool {
        self.0
            .values()
            .filter_map(|slot| slot.value.as_ref())
            .any(|item| item_criteria(item))
    }

    pub fn get_total_weight(&self) -> usize {
        self.0
            .values()
            .filter_map(|slot| slot.value.as_ref())
            .map(|item| item.get_weight())
            .sum()
    }
}

#[derive(Debug)]
pub enum SlotsError {
    NotExists { slot: String },
    SlotProblem(SlotError),
}

impl From<SlotError> for SlotsError {
    fn from(value: SlotError) -> Self {
        SlotsError::SlotProblem(value)
    }
}

impl fmt::Display for SlotsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            SlotsError::NotExists { slot } => format!("{slot} slot does not exist."),
            SlotsError::SlotProblem(e) => e.to_string(),
        };

        write!(f, "{result}")
    }
}

impl error::Error for SlotsError {}

pub type SlotsResult<T> = Result<T, SlotsError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _should_equip_if_valid() -> SlotResult<()> {
        let mut vc = Slot::new(|item: &Item| item.has_type("armor"));

        vc.equip(Item::new("one", 1, vec!["armor".into()]))?;

        assert_eq!(vc.value, Some(Item::new("one", 1, vec!["armor".into()])));

        Ok(())
    }

    #[test]
    fn _should_return_err_if_equip_not_valid() {
        let mut vc = Slot::new(|item: &Item| item.has_type("armor"));
        let result = vc.equip(Item::new("not armor", 42, vec![]));

        assert!(
            matches!(result, Err(SlotError::Invalid)),
            "Should return Err"
        );
    }

    #[test]
    fn _should_prevent_equipping_multiple_things_to_same_slot() {
        let mut vc = Slot::new(|_: &Item| true);

        let _ = vc.equip(Item::new("armor 1", 35, vec!["armor".into()]));

        let result = vc.equip(Item::new("armor 2", 35, vec![String::from("armor")]));

        assert!(matches!(result, Err(SlotError::Full)));
    }

    #[test]
    fn _should_prevent_removing_from_empty_slot() {
        let mut vc = Slot::new(|_: &Item| false);

        assert!(matches!(vc.unequip(), Err(SlotError::Empty)));
    }

    #[test]
    fn _should_return_stored_thing_on_unequip() -> SlotResult<()> {
        let mut vc = Slot::new(|_: &Item| true);
        let _ = vc.equip(Item::new("dummy", 0, vec![]));

        assert_eq!(vc.unequip()?, Item::new("dummy", 0, vec![]));

        Ok(())
    }

    #[test]
    fn _should_remove_value_from_slot_on_unequip() -> SlotResult<()> {
        let mut vc = Slot::new(|_: &Item| true);
        vc.value = Some(Item::new("dummy", 0, vec![]));
        let _ = vc.unequip()?;

        assert_eq!(vc.value, None);

        Ok(())
    }

    mod slots {
        use super::*;

        #[test]
        fn _should_allow_equipping_to_multiple_slots() -> SlotsResult<()> {
            let mut equipment = ItemSlots::default();
            equipment.add_slot("armor", Slot::new(|item| item.has_type("armor")));
            equipment.add_slot("right hand", Slot::new(|item| item.has_type("weapon")));

            equipment.equip(Item::new("Chain Mail", 55, vec!["armor".into()]), "armor")?;
            equipment.equip(Item::new("Rapier", 2, vec!["weapon".into()]), "right hand")?;

            Ok(())
        }

        #[test]
        fn _should_return_whether_contains_thing_of_given_type() -> SlotsResult<()> {
            let mut equipment = ItemSlots::default();
            equipment.add_slot("armor", Slot::new(|item| item.has_type("armor")));
            equipment.add_slot("right hand", Slot::new(|item| item.has_type("weapon")));

            let armor_criteria = |item: &Item| item.has_type("armor");

            equipment.equip(Item::new("Rapier", 2, vec!["weapon".into()]), "right hand")?;
            assert!(!equipment.has_item_equipped_matching_criteria(armor_criteria));

            equipment.equip(Item::new("Chain Mail", 55, vec!["armor".into()]), "armor")?;
            assert!(equipment.has_item_equipped_matching_criteria(armor_criteria));

            Ok(())
        }

        #[test]
        fn _should_return_the_total_weight_of_equipped_items() {
            let mut equipment = ItemSlots::default();
            equipment.add_slot("armor", Slot::new(|_: &Item| true));
            equipment.add_slot("right hand", Slot::new(|_: &Item| true));

            let _ = equipment.equip(Item::new("Chain Mail", 55, vec!["armor".into()]), "armor");
            let _ = equipment.equip(Item::new("Rapier", 2, vec!["weapon".into()]), "right hand");

            assert_eq!(equipment.get_total_weight(), 57);
        }
    }
}
