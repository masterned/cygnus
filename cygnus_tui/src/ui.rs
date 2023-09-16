use cygnus_models::{ability, character::Character, modifiers::Proficiency, skills};
use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Cell, Paragraph, Row, Table, Wrap,
    },
};

use crate::app::App;

fn ability_widget(character: &Character, ability: ability::Identifier) -> Paragraph {
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
    ability: ability::Identifier,
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

    ability::Identifier::all()
        .iter()
        .enumerate()
        .for_each(|(i, &ability)| {
            render_ability_widget(frame, character, ability, abilities_layout[i])
        });
}

fn render_header<B: Backend>(frame: &mut Frame<'_, B>, character: &Character, area: Rect) {
    let header_layout = Layout::new()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(2, 5),
            Constraint::Ratio(1, 5),
            Constraint::Ratio(2, 5),
        ])
        .split(area);

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
    ))
    .block(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(header_widget, header_layout[0]);
    render_health_block(frame, character, header_layout[2]);
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
    ability: ability::Identifier,
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
    ability_left: ability::Identifier,
    ability_right: ability::Identifier,
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

    ability::Identifier::all()
        .chunks(2)
        .enumerate()
        .for_each(|(i, pair)| {
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

fn render_proficiency_bonus<B: Backend>(
    frame: &mut Frame<'_, B>,
    character: &Character,
    area: Rect,
) {
    let proficiency_bonus = Paragraph::new(format!("{:+}", character.get_proficiency_bonus()))
        .block(
            Block::default()
                .title(
                    Title::from("Proficiency")
                        .alignment(Alignment::Center)
                        .position(Position::Top),
                )
                .title(
                    Title::from("Bonus")
                        .alignment(Alignment::Center)
                        .position(Position::Bottom),
                )
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Center);

    frame.render_widget(proficiency_bonus, area);
}

fn render_walking_speed<B: Backend>(frame: &mut Frame<'_, B>, character: &Character, area: Rect) {
    let walking_speed = Paragraph::new(format!("{} ft.", character.get_walking_speed()))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(
                    Title::from("Walking")
                        .alignment(Alignment::Center)
                        .position(Position::Top),
                )
                .title(
                    Title::from("Speed")
                        .alignment(Alignment::Center)
                        .position(Position::Bottom),
                ),
        )
        .alignment(Alignment::Center);

    frame.render_widget(walking_speed, area);
}

fn render_skills_table<B: Backend>(frame: &mut Frame<'_, B>, character: &Character, area: Rect) {
    let header_cells = ["Prof", "Mod", "Skill", "Bonus"]
        .iter()
        .map(|&h| Cell::from(h));
    let header = Row::new(header_cells).height(1).bottom_margin(1);
    let skills = skills::Identifier::all();
    let rows = skills.iter().map(|&id| {
        let cells = [
            Cell::from(format!(
                "{}",
                match character.get_skill_proficiency(id) {
                    Some(Proficiency::Proficiency) => "x",
                    Some(Proficiency::Expertise) => "*",
                    None => "o",
                }
            )),
            Cell::from(format!("{}", id.get_default_ability().abbr())),
            Cell::from(format!("{id}")),
            Cell::from(format!("{:+}", character.get_skill_modifier(id))),
        ];
        Row::new(cells)
    });
    let table = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(
                    Title::from("Skills")
                        .alignment(Alignment::Center)
                        .position(Position::Bottom),
                ),
        )
        .widths([Constraint::Ratio(1, 4); 4].as_ref());

    frame.render_widget(table, area);
}

fn render_first_row<B: Backend>(frame: &mut Frame<'_, B>, character: &Character, area: Rect) {
    let first_row_layout = Layout::new()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 4); 4])
        .split(area);

    render_proficiency_bonus(frame, character, first_row_layout[0]);
    render_walking_speed(frame, character, first_row_layout[1]);
    render_initiative(frame, character, first_row_layout[2]);
    render_armor_class(frame, character, first_row_layout[3]);
}

fn render_rolls_block<B: Backend>(frame: &mut Frame<'_, B>, character: &Character, area: Rect) {
    let twin_layout = Layout::new()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50); 2])
        .split(area);

    let left_column_layout = Layout::new()
        .constraints([
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 2),
        ])
        .split(twin_layout[0]);
    render_saving_throws_block(frame, character, left_column_layout[0]);
    render_senses_block(frame, character, left_column_layout[1]);
    render_proficiencies_and_languages_block(frame, character, left_column_layout[2]);

    render_skills_table(frame, character, twin_layout[1]);
}

fn render_senses_block<B: Backend>(frame: &mut Frame<'_, B>, character: &Character, area: Rect) {
    let senses_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(
            Title::from("Senses")
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        );
    frame.render_widget(senses_block.clone(), area);

    let senses_layout = Layout::new()
        .constraints([Constraint::Ratio(1, 4); 4])
        .split(senses_block.inner(area));

    let passive_perception_row_layout = Layout::new()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3), Constraint::Min(0)])
        .split(senses_layout[0]);
    frame.render_widget(
        Paragraph::new(format!("{}", character.get_passive_perception()))
            .alignment(Alignment::Center),
        passive_perception_row_layout[0],
    );
    frame.render_widget(
        Paragraph::new("Passive WIS (Perception)"),
        passive_perception_row_layout[1],
    );

    let passive_investigation_row_layout = Layout::new()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3), Constraint::Min(0)])
        .split(senses_layout[1]);
    frame.render_widget(
        Paragraph::new(format!("{}", character.get_passive_investigation()))
            .alignment(Alignment::Center),
        passive_investigation_row_layout[0],
    );
    frame.render_widget(
        Paragraph::new("Passive INT (Investigation)"),
        passive_investigation_row_layout[1],
    );

    let passive_insight_row_layout = Layout::new()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3), Constraint::Min(0)])
        .split(senses_layout[2]);
    frame.render_widget(
        Paragraph::new(format!("{}", character.get_passive_insight())).alignment(Alignment::Center),
        passive_insight_row_layout[0],
    );
    frame.render_widget(
        Paragraph::new("Passive WIS (Insight)"),
        passive_insight_row_layout[1],
    );

    if let Some(darkvision) = character.get_darkvision() {
        frame.render_widget(
            Paragraph::new(format!("Darkvision {} ft.", darkvision)).alignment(Alignment::Center),
            senses_layout[3],
        );
    }
}

fn render_proficiencies_and_languages_block<B: Backend>(
    frame: &mut Frame<'_, B>,
    character: &Character,
    area: Rect,
) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(
            Title::from("Proficiencies & Languages")
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        );
    frame.render_widget(block.clone(), area);

    let layout = Layout::new()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 4); 4])
        .split(block.inner(area));

    frame.render_widget(
        Paragraph::new(character.get_armor_proficiencies_string())
            .block(
                Block::new()
                    .title(Title::from("Armor Proficiencies"))
                    .title_style(Style::default().add_modifier(Modifier::BOLD)),
            )
            .wrap(Wrap { trim: true }),
        layout[0],
    );

    frame.render_widget(
        Paragraph::new(character.get_weapon_proficiencies_string())
            .block(
                Block::new()
                    .title(Title::from("Weapon Proficiencies"))
                    .title_style(Style::default().add_modifier(Modifier::BOLD)),
            )
            .wrap(Wrap { trim: true }),
        layout[1],
    );

    frame.render_widget(
        Paragraph::new(character.get_tool_proficiencies_string())
            .block(
                Block::new()
                    .title(Title::from("Tool Proficiencies"))
                    .title_style(Style::default().add_modifier(Modifier::BOLD)),
            )
            .wrap(Wrap { trim: true }),
        layout[2],
    );

    frame.render_widget(
        Paragraph::new(character.get_languages_string())
            .block(
                Block::new()
                    .title(Title::from("Languages"))
                    .title_style(Style::default().add_modifier(Modifier::BOLD)),
            )
            .wrap(Wrap { trim: true }),
        layout[3],
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
            Constraint::Ratio(1, 16),
            Constraint::Ratio(1, 16),
            Constraint::Min(0),
        ])
        .split(frame.size());

    render_header(frame, character_ref, layout[0]);
    render_first_row(frame, character_ref, layout[1]);
    render_abilities(frame, character_ref, layout[2]);
    render_rolls_block(frame, character_ref, layout[3]);
}
