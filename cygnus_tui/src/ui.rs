use cygnus_models::{ability::Ability, character::Character};
use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Paragraph,
    },
};

use crate::app::App;

fn ability_widget(character: &Character, ability: Ability) -> Paragraph {
    let modifier = character.get_ability_modifier(ability);
    Paragraph::new(format!(
        "{}{}",
        if modifier < 0 { "-" } else { "+" },
        modifier
    ))
    .alignment(Alignment::Center)
    .block(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(Title::from(ability.to_string()).alignment(Alignment::Center))
            .title(
                Title::from(character.get_ability_score(ability).to_string())
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            ),
    )
}

fn render_ability_widget<B: Backend>(
    frame: &mut Frame<'_, B>,
    character: &Character,
    ability: Ability,
    rect: Rect,
) {
    let ability = ability_widget(character, ability);

    frame.render_widget(ability, rect);
}

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Ratio(1, 6),
            Constraint::Ratio(1, 6),
            Constraint::Ratio(1, 6),
            Constraint::Ratio(1, 6),
            Constraint::Ratio(1, 6),
            Constraint::Ratio(1, 6),
        ])
        .split(frame.size());

    let character_ref = app
        .character
        .as_ref()
        .expect("Can't render a `Character` if it doesn't exist.");

    Ability::all()
        .iter()
        .enumerate()
        .for_each(|(i, &ability)| render_ability_widget(frame, character_ref, ability, layout[i]));
}
