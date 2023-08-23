use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Die {
    sides: usize,
    count: usize,
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}d{}", self.count, self.sides)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Roll {
    dice: Vec<Die>,
    modifier: isize,
}

impl Roll {
    #[must_use]
    pub fn new(count: usize, sides: usize, modifier: isize) -> Self {
        Roll {
            dice: vec![Die { sides, count }],
            modifier,
        }
    }

    pub fn add_die(&mut self, count: usize, sides: usize) {
        if let Some(die) = self.dice.iter_mut().find(|die| die.sides == sides) {
            die.count += count;
        } else {
            self.dice.push(Die { sides, count });
        }

        self.dice.sort();
    }
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let modifier = match self.modifier.signum() {
            -1 => format!(" - {}", self.modifier.abs()),
            1 => format!(" + {}", self.modifier),
            _ => String::new(),
        };

        let dice = self
            .dice
            .split_first()
            .map_or(String::new(), |(head, tail)| {
                tail.iter()
                    .fold(head.to_string(), |acc, die| format!("{acc} + {die}"))
            });

        write!(f, "{dice}{modifier}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _should_display_no_mod() {
        let dr = Roll::new(1, 6, 0);

        assert_eq!(dr.to_string(), "1d6");
    }

    #[test]
    fn _should_display_positive_mod() {
        let dr = Roll::new(1, 6, 2);

        assert_eq!(dr.to_string(), "1d6 + 2");
    }

    #[test]
    fn _should_display_negative_mod() {
        let dr = Roll::new(1, 6, -1);

        assert_eq!(dr.to_string(), "1d6 - 1");
    }

    #[test]
    fn _should_add_new_size_die_to_dice_roll() {
        let mut dr = Roll::new(1, 6, 1);
        dr.add_die(2, 10);

        assert_eq!(dr.to_string(), "1d6 + 2d10 + 1");
    }

    #[test]
    fn _should_sort_new_dice() {
        let mut dr = Roll::new(1, 10, 2);
        dr.add_die(2, 4);

        assert_eq!(dr.to_string(), "2d4 + 1d10 + 2");
    }

    #[test]
    fn _should_combine_like_die() {
        let mut dr = Roll::new(1, 6, 3);
        dr.add_die(2, 6);

        assert_eq!(dr.to_string(), "3d6 + 3");
    }
}
