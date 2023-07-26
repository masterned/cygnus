pub struct Item {
    weight: usize,
}

impl Item {
    pub fn new(weight: usize) -> Self {
        Item { weight }
    }

    pub fn get_weight(&self) -> usize {
        self.weight
    }
}

#[derive(Default)]
pub struct Items(Vec<Item>);

impl Items {
    pub fn get_total_weight(&self) -> usize {
        self.0.iter().map(|item| item.get_weight()).sum()
    }

    pub fn add_item(&mut self, item: Item) {
        self.0.push(item)
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
            Item { weight: 1 },
            Item { weight: 2 },
            Item { weight: 3 },
        ]);

        assert_eq!(items.get_total_weight(), 6);
    }
}
