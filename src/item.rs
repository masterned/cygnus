#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    name: String,
    weight: usize,
    types: Vec<String>,
}

impl Item {
    #[must_use]
    pub fn new(name: impl Into<String>, weight: usize, types: Vec<String>) -> Self {
        Item {
            name: name.into(),
            weight,
            types,
        }
    }

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
}

#[derive(Default, Debug)]
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
            },
            Item {
                name: String::from("two"),
                weight: 2,
                types: vec![],
            },
            Item {
                name: String::from("three"),
                weight: 3,
                types: vec![],
            },
        ]);

        assert_eq!(items.get_total_weight(), 6);
    }
}
