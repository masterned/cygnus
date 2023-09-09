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
    Paragraph::new(format!("{modifier:+}"))
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

fn render_saving_throw_pair<B: Backend>(
    frame: &mut Frame<'_, B>,
    character: &Character,
    ability: Ability,
    rect: Rect,
) {
    let saving_throw_pair_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2); 2])
        .split(rect);

    frame.render_widget(
        Paragraph::new(ability.abbr()).alignment(Alignment::Center),
        saving_throw_pair_layout[0],
    );

    let saving_throw_mod = character.get_saving_throw_mod(ability);

    frame.render_widget(
        Paragraph::new(format!("{saving_throw_mod:+}")).alignment(Alignment::Center),
        saving_throw_pair_layout[1],
    );
}

fn render_saving_throw_row<B: Backend>(
    frame: &mut Frame<'_, B>,
    character: &Character,
    ability_left: Ability,
    ability_right: Ability,
    rect: Rect,
) {
    let saving_throw_row_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2); 2])
        .split(rect);

    render_saving_throw_pair(frame, character, ability_left, saving_throw_row_layout[0]);
    render_saving_throw_pair(frame, character, ability_right, saving_throw_row_layout[1]);
}

fn render_saving_throws_block<B: Backend>(
    frame: &mut Frame<'_, B>,
    character: &Character,
    rect: Rect,
) {
    let saving_throws_block = Block::new()
        .title(
            Title::from("Saving Throws")
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(saving_throws_block.clone(), rect);

    let saving_throw_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 3); 3])
        .split(saving_throws_block.inner(rect));

    Ability::all().chunks(2).enumerate().for_each(|(i, pair)| {
        render_saving_throw_row(frame, character, pair[0], pair[1], saving_throw_rows[i])
    });
}

fn render_armor_class<B: Backend>(frame: &mut Frame<'_, B>, character: &Character, rect: Rect) {
    let armor_class = Paragraph::new(format!("{}", character.get_armor_class()))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(
                    Title::from("Armor")
                        .alignment(Alignment::Center)
                        .position(Position::Top),
                )
                .title(
                    Title::from("Class")
                        .alignment(Alignment::Center)
                        .position(Position::Bottom),
                ),
        )
        .alignment(Alignment::Center);

    frame.render_widget(armor_class, rect);
}

fn render_initiative<B: Backend>(frame: &mut Frame<'_, B>, character: &Character, rect: Rect) {
    let initiative = Paragraph::new(format!("{:+}", character.get_initiative()))
        .block(
            Block::default()
                .title(
                    Title::from("Initiative")
                        .alignment(Alignment::Center)
                        .position(Position::Top),
                )
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Center);

    frame.render_widget(initiative, rect);
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
            Constraint::Ratio(2, 10),
            Constraint::Ratio(1, 10),
            Constraint::Min(0),
        ])
        .split(frame.size());

    render_header(frame, character_ref, layout[0]);
    render_abilities(frame, character_ref, layout[1]);
    render_health_block(frame, character_ref, layout[2]);
    render_saving_throws_block(frame, character_ref, layout[3]);

    let stat_row_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 4); 4])
        .split(layout[4]);

    render_initiative(frame, character_ref, stat_row_layout[2]);
    render_armor_class(frame, character_ref, stat_row_layout[3]);
}
