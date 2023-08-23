#![warn(clippy::pedantic)]
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct LowerBoundMap<K, V>(BTreeMap<K, V>);

impl<K: Ord + Clone, V> LowerBoundMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) {
        self.0.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.0.get(key).or_else(|| {
            let lower_bound = self.0.range(..key.clone()).next_back();
            lower_bound.map(|(_, v)| v)
        })
    }
}

impl<K: Ord + Clone, V, const N: usize> From<[(K, V); N]> for LowerBoundMap<K, V> {
    fn from(value: [(K, V); N]) -> Self {
        Self(BTreeMap::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _should_return_none_on_empty_map() {
        let lbm: LowerBoundMap<i32, usize> = LowerBoundMap::default();

        assert_eq!(lbm.get(&42), None);
    }

    #[test]
    fn _should_return_value_if_key_exists() {
        let lbm = LowerBoundMap::from([
            (12, "A cool number"),
            (42, "The answer to everything"),
            (9001, "IT'S OVER 9000!!!"),
        ]);

        assert_eq!(lbm.get(&12), Some(&"A cool number"));
        assert_eq!(lbm.get(&42), Some(&"The answer to everything"));
        assert_eq!(lbm.get(&9001), Some(&"IT'S OVER 9000!!!"));
    }

    #[test]
    fn _should_return_the_next_value_lower_than_key_if_key_does_not_exist() {
        let lbm = LowerBoundMap::from([(3, "Three"), (7, "Seven"), (12, "Twelve")]);

        assert_eq!(lbm.get(&5), Some(&"Three"));
        assert_eq!(lbm.get(&10), Some(&"Seven"));
        assert_eq!(lbm.get(&490), Some(&"Twelve"));
    }

    #[test]
    fn _should_return_none_if_no_lower_key_exists() {
        let lbm = LowerBoundMap::from([(7, "prime"), (11, "also prime")]);

        assert_eq!(lbm.get(&3), None);
        assert_eq!(lbm.get(&6), None);
    }
}
