use std::{collections::HashMap, error, fmt};

use crate::item::Item;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug, Default)]
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
            .any(item_criteria)
    }

    pub fn get_total_weight(&self) -> usize {
        self.0
            .values()
            .filter_map(|slot| slot.value.as_ref())
            .map(|item| item.get_weight())
            .sum()
    }

    pub fn get_equipped_items(&self) -> Vec<&Item> {
        self.0
            .values()
            .filter_map(|slot| slot.value.as_ref())
            .collect()
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
    use std::error::Error;

    use crate::item;

    use super::*;

    #[test]
    fn _should_equip_if_valid() -> Result<(), Box<dyn Error>> {
        let mut vc = Slot::new(|item: &Item| item.has_type("armor"));

        let item = item::Builder::new()
            .name("one")?
            .add_type("armor")?
            .build()?;
        vc.equip(item.clone())?;

        assert_eq!(vc.value, Some(item));

        Ok(())
    }

    #[test]
    fn _should_return_err_if_equip_not_valid() -> Result<(), Box<dyn Error>> {
        let mut vc = Slot::new(|item: &Item| item.has_type("armor"));

        let not_armor = item::Builder::new().name("not armor")?.build()?;
        let result = vc.equip(not_armor);

        assert!(
            matches!(result, Err(SlotError::Invalid)),
            "Should return Err"
        );

        Ok(())
    }

    #[test]
    fn _should_prevent_equipping_multiple_things_to_same_slot() -> Result<(), Box<dyn Error>> {
        let mut vc = Slot::new(|_: &Item| true);

        let item1 = item::Builder::new().name("item 1")?.build()?;
        let _ = vc.equip(item1.clone());

        let item2 = item::Builder::new().name("item 2")?.build()?;
        let result = vc.equip(item2);

        assert!(matches!(result, Err(SlotError::Full)));

        Ok(())
    }

    #[test]
    fn _should_prevent_removing_from_empty_slot() {
        let mut vc = Slot::new(|_: &Item| false);

        assert!(matches!(vc.unequip(), Err(SlotError::Empty)));
    }

    #[test]
    fn _should_return_stored_thing_on_unequip() -> Result<(), Box<dyn Error>> {
        let mut vc = Slot::new(|_: &Item| true);

        let item = item::Builder::new().name("dummy")?.build()?;
        let _ = vc.equip(item.clone());

        assert_eq!(vc.unequip()?, item);

        Ok(())
    }

    #[test]
    fn _should_remove_value_from_slot_on_unequip() -> Result<(), Box<dyn Error>> {
        let mut vc = Slot::new(|_: &Item| true);

        let item = item::Builder::new().name("dummy")?.build()?;
        vc.value = Some(item);
        let _ = vc.unequip()?;

        assert_eq!(vc.value, None);

        Ok(())
    }

    mod slots {
        use std::error::Error;

        use crate::item::ArmorClass;

        use super::*;

        #[test]
        fn _should_allow_equipping_to_multiple_slots() -> Result<(), Box<dyn Error>> {
            let mut equipment = ItemSlots::default();
            equipment.add_slot("armor", Slot::new(|item| item.has_type("armor")));
            equipment.add_slot("right hand", Slot::new(|item| item.has_type("weapon")));

            let chain_mail = item::Builder::new()
                .name("Chain Mail")?
                .weight(55)?
                .add_type("armor")?
                .armor_class(ArmorClass::Heavy(16))?
                .build()?;
            equipment.equip(chain_mail, "armor")?;

            let rapier = item::Builder::new()
                .name("Rapier")?
                .weight(2)?
                .add_type("weapon")?
                .build()?;
            equipment.equip(rapier, "right hand")?;

            Ok(())
        }

        #[test]
        fn _should_return_whether_contains_thing_of_given_type() -> Result<(), Box<dyn Error>> {
            let mut equipment = ItemSlots::default();
            equipment.add_slot("armor", Slot::new(|item| item.has_type("armor")));
            equipment.add_slot("right hand", Slot::new(|item| item.has_type("weapon")));

            let armor_criteria = |item: &Item| item.has_type("armor");

            let rapier = item::Builder::new()
                .name("Rapier")?
                .weight(2)?
                .add_type("weapon")?
                .build()?;
            equipment.equip(rapier, "right hand")?;
            assert!(!equipment.has_item_equipped_matching_criteria(armor_criteria));

            let chain_mail = item::Builder::new()
                .name("Chain Mail")?
                .weight(55)?
                .add_type("armor")?
                .armor_class(ArmorClass::Heavy(16))?
                .build()?;
            equipment.equip(chain_mail, "armor")?;
            assert!(equipment.has_item_equipped_matching_criteria(armor_criteria));

            Ok(())
        }

        #[test]
        fn _should_return_the_total_weight_of_equipped_items() -> Result<(), Box<dyn Error>> {
            let mut equipment = ItemSlots::default();
            equipment.add_slot("armor", Slot::new(|_: &Item| true));
            equipment.add_slot("right hand", Slot::new(|_: &Item| true));

            let chain_mail = item::Builder::new()
                .name("Chain Mail")?
                .weight(55)?
                .add_type("armor")?
                .armor_class(ArmorClass::Heavy(16))?
                .build()?;
            equipment.equip(chain_mail, "armor")?;

            let rapier = item::Builder::new()
                .name("Rapier")?
                .weight(2)?
                .add_type("weapon")?
                .build()?;
            equipment.equip(rapier, "right hand")?;

            assert_eq!(equipment.get_total_weight(), 57);

            Ok(())
        }
    }
}
