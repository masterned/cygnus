#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiceRoll {
    count: usize,
    sides: usize,
    modifier: isize,
}

impl DiceRoll {
    pub fn new(count: usize, sides: usize, modifier: isize) -> Self {
        DiceRoll {
            count,
            sides,
            modifier,
        }
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    pub fn get_sides(&self) -> usize {
        self.sides
    }

    pub fn get_modifier(&self) -> isize {
        self.modifier
    }
}
