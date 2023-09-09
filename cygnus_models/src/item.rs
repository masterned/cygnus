use std::{error, fmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmorClass {
    Light(usize),
    Medium(usize),
    Heavy(usize),
}

#[derive(Debug, Default)]
pub struct Builder {
    name: Option<String>,
    weight: Option<usize>,
    types: Vec<String>,
    armor_class: Option<ArmorClass>,
}

impl Builder {
    pub fn new() -> Self {
        Builder::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Result<Self, ConstructionError> {
        let name: String = name.into();

        if name.is_empty() {
            return Err(ConstructionError::MissingName);
        }

        self.name = Some(name);

        Ok(self)
    }

    pub fn weight(mut self, weight: usize) -> Result<Self, ConstructionError> {
        self.weight = Some(weight);

        Ok(self)
    }

    pub fn add_type(mut self, new_type: impl Into<String>) -> Result<Self, ConstructionError> {
        self.types.push(new_type.into());

        Ok(self)
    }

    pub fn armor_class(mut self, armor_class: ArmorClass) -> Result<Self, ConstructionError> {
        self.armor_class = Some(armor_class);

        Ok(self)
    }

    pub fn build(self) -> Result<Item, ConstructionError> {
        let name = self.name.ok_or(ConstructionError::MissingName)?;
        let weight = self.weight.unwrap_or(0);
        let types = self.types.clone();
        let armor_class = self.armor_class;

        Ok(Item {
            name,
            weight,
            types,
            armor_class,
        })
    }
}

#[derive(Debug)]
pub enum ConstructionError {
    MissingName,
}

impl fmt::Display for ConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            ConstructionError::MissingName => "Cannot create an Item without a name.",
        };

        write!(f, "{result}")
    }
}

impl error::Error for ConstructionError {}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    name: String,
    weight: usize,
    types: Vec<String>,
    armor_class: Option<ArmorClass>,
}

impl Item {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn get_weight(&self) -> usize {
        self.weight
    }

    pub fn has_type(&self, item_type: impl Into<String>) -> bool {
        self.types.contains(&item_type.into())
    }

    pub fn get_armor_class(&self) -> Option<ArmorClass> {
        self.armor_class
    }
}

#[derive(Clone, Default, Debug)]
pub struct Items(Vec<Item>);

impl Items {
    #[must_use]
    pub fn get_total_weight(&self) -> usize {
        self.0.iter().map(Item::get_weight).sum()
    }

    pub fn add_item(&mut self, item: Item) {
        self.0.push(item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _empty_items_should_have_no_weight() {
        let items = Items::default();

        assert_eq!(items.get_total_weight(), 0);
    }

    #[test]
    fn _should_accumulate_total_weight() {
        let items = Items(vec![
            Item {
                name: String::from("one"),
                weight: 1,
                types: vec![],
                armor_class: None,
            },
            Item {
                name: String::from("two"),
                weight: 2,
                types: vec![],
                armor_class: None,
            },
            Item {
                name: String::from("three"),
                weight: 3,
                types: vec![],
                armor_class: None,
            },
        ]);

        assert_eq!(items.get_total_weight(), 6);
    }
}
