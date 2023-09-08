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

fn render_abilities<B: Backend>(frame: &mut Frame<'_, B>, character: &Character, rect: Rect) {
    let abilities_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 6); 6])
        .split(rect);

    Ability::all().iter().enumerate().for_each(|(i, &ability)| {
        render_ability_widget(frame, character, ability, abilities_layout[i])
    });
}

fn render_header<B: Backend>(frame: &mut Frame<'_, B>, character: &Character, rect: Rect) {
    let header_widget = Paragraph::new(format!(
        "{}\n{} {} {}\nLevel {}",
        character.get_name(),
        character
            .get_gender()
            .map(|g| g.to_string())
            .unwrap_or("".into()),
        character.get_race_name(),
        character.get_class_details(),
        character.get_level()
    ));

    frame.render_widget(header_widget, rect);
}

fn render_health_block<B: Backend>(frame: &mut Frame<'_, B>, character: &Character, rect: Rect) {
    let health_block = Block::new()
        .title(
            Title::from("Hit Points")
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(health_block.clone(), rect);

    let health_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(2, 7),
            Constraint::Ratio(1, 7),
            Constraint::Ratio(2, 7),
            Constraint::Ratio(2, 7),
        ])
        .split(health_block.inner(rect));

    frame.render_widget(
        Paragraph::new(character.get_current_hit_points().to_string())
            .alignment(Alignment::Center)
            .block(
                Block::new()
                    .title("Current")
                    .title_alignment(Alignment::Center),
            ),
        health_layout[0],
    );
    frame.render_widget(
        Paragraph::new("/")
            .alignment(Alignment::Center)
            .block(Block::new().title("").title_alignment(Alignment::Center)),
        health_layout[1],
    );
    frame.render_widget(
        Paragraph::new(character.get_hit_points_max().to_string())
            .alignment(Alignment::Center)
            .block(Block::new().title("Max").title_alignment(Alignment::Center)),
        health_layout[2],
    );
    frame.render_widget(
        Paragraph::new("--").alignment(Alignment::Center).block(
            Block::new()
                .title("Temp")
                .title_alignment(Alignment::Center),
        ),
        health_layout[3],
    );
}

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let character_ref = app
        .character
        .as_ref()
        .expect("Can't render a `Character` if it doesn't exist.");

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 10),
            Constraint::Ratio(1, 10),
            Constraint::Ratio(1, 10),
            Constraint::Min(0),
        ])
        .split(frame.size());

    render_header(frame, character_ref, layout[0]);
    render_abilities(frame, character_ref, layout[1]);
    render_health_block(frame, character_ref, layout[2]);
}
