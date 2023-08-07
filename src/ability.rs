use std::collections::BTreeMap;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl Ability {
    pub fn all() -> Vec<Self> {
        vec![
            Ability::Strength,
            Ability::Dexterity,
            Ability::Constitution,
            Ability::Intelligence,
            Ability::Wisdom,
            Ability::Charisma,
        ]
    }

    pub fn calculate_modifier(ability_score: usize) -> isize {
        ability_score as isize / 2 - 5
    }

    pub fn get_abbreviation(&self) -> &'static str {
        match self {
            Ability::Strength => "STR",
            Ability::Dexterity => "DEX",
            Ability::Constitution => "CON",
            Ability::Intelligence => "INT",
            Ability::Wisdom => "WIS",
            Ability::Charisma => "CHA",
        }
    }

    pub fn render_tui<B: Backend>(&self, f: &mut Frame<B>, area: Rect, score: usize) {
        let ability_block = Paragraph::new(score.to_string())
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(self.get_abbreviation())
                    .borders(Borders::ALL)
                    .title_alignment(Alignment::Center),
            );
        f.render_widget(ability_block, area);
    }
}

pub struct AbilitiesTemplate {
    pub strength: usize,
    pub dexterity: usize,
    pub constitution: usize,
    pub intelligence: usize,
    pub wisdom: usize,
    pub charisma: usize,
}

pub struct Abilities(pub BTreeMap<Ability, usize>);

impl Abilities {
    pub fn empty() -> Self {
        Abilities(BTreeMap::new())
    }

    pub fn get_base_score(&self, ability: &Ability) -> usize {
        *self.0.get(ability).unwrap_or(&0)
    }

    pub fn set_score(&mut self, ability: Ability, score: usize) {
        self.0.insert(ability, score);
    }

    pub fn get_modifier(&self, ability: &Ability) -> isize {
        Ability::calculate_modifier(self.get_base_score(ability))
    }

    pub fn render_tui<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let area = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(2),
                    Constraint::Percentage(16),
                    Constraint::Percentage(16),
                    Constraint::Percentage(16),
                    Constraint::Percentage(16),
                    Constraint::Percentage(16),
                    Constraint::Percentage(16),
                    Constraint::Percentage(2),
                ]
                .as_ref(),
            )
            .split(area);

        self.0
            .iter()
            .enumerate()
            .for_each(|(i, (ability, score))| ability.render_tui(f, area[i + 1], *score));
    }
}

impl Default for Abilities {
    fn default() -> Self {
        Abilities(BTreeMap::from_iter(
            Ability::all().iter().map(|&ability| (ability, 8)),
        ))
    }
}

impl From<AbilitiesTemplate> for Abilities {
    fn from(value: AbilitiesTemplate) -> Self {
        let mut abilities = Abilities::empty();

        abilities.set_score(Ability::Strength, value.strength);
        abilities.set_score(Ability::Dexterity, value.dexterity);
        abilities.set_score(Ability::Constitution, value.constitution);
        abilities.set_score(Ability::Intelligence, value.intelligence);
        abilities.set_score(Ability::Wisdom, value.wisdom);
        abilities.set_score(Ability::Charisma, value.charisma);

        abilities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _ability_score_of_10_should_have_modifier_of_0() {
        assert_eq!(Ability::calculate_modifier(10), 0);
    }

    #[test]
    fn _ability_scores_less_than_10_should_have_negative_modifier() {
        assert_eq!(Ability::calculate_modifier(8), -1);
        assert_eq!(Ability::calculate_modifier(6), -2);
        assert_eq!(Ability::calculate_modifier(4), -3);
        assert_eq!(Ability::calculate_modifier(2), -4);
        assert_eq!(Ability::calculate_modifier(0), -5);
    }

    #[test]
    fn _ability_scores_greater_than_10_should_have_positive_modifiers() {
        assert_eq!(Ability::calculate_modifier(12), 1);
        assert_eq!(Ability::calculate_modifier(14), 2);
        assert_eq!(Ability::calculate_modifier(16), 3);
        assert_eq!(Ability::calculate_modifier(18), 4);
        assert_eq!(Ability::calculate_modifier(20), 5);
    }
}
