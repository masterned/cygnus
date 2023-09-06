use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    ability::{Abilities, Ability},
    character::Character,
};

pub fn render_character<B: Backend>(frame: &mut Frame<B>, area: Rect, character: &Character) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(4),
                Constraint::Percentage(50),
                Constraint::Percentage(10),
                Constraint::Percentage(23),
                Constraint::Percentage(23),
            ]
            .as_ref(),
        )
        .split(area);

    render_header(frame, layout[1], character);
    render_abilities(frame, layout[2], &character.get_abilities());
}

fn render_header<B: Backend>(frame: &mut Frame<B>, area: Rect, character: &Character) {
    let name = character.get_name();
    let race_name = character.get_race_name();
    let class_details = character.get_class_details();
    let current_hp = character.get_current_hit_points();
    let hp_max = character.get_hit_points_max();
    let proficiency_bonus = character.get_proficiency_bonus();
    let walking_speed = character.get_walking_speed();
    let initiative = character.get_initiative();
    let armor_class = character.get_armor_class();

    let header_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(14),
                Constraint::Percentage(14),
                Constraint::Percentage(14),
                Constraint::Percentage(14),
                Constraint::Percentage(14),
                Constraint::Percentage(14),
                Constraint::Percentage(14),
            ]
            .as_ref(),
        )
        .split(area);

    frame.render_widget(Paragraph::new(name), header_layout[0]);
    frame.render_widget(
        Paragraph::new(format!("{race_name} {class_details}")),
        header_layout[1],
    );
    frame.render_widget(
        Paragraph::new(format!("{current_hp}/{hp_max}"))
            .block(
                Block::default()
                    .title("HP")
                    .borders(Borders::ALL)
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Center),
        header_layout[2],
    );
    frame.render_widget(
        Paragraph::new(format!("{proficiency_bonus}"))
            .block(
                Block::default()
                    .title("Prof. Bonus")
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Center),
        header_layout[3],
    );
    frame.render_widget(
        Paragraph::new(format!("{walking_speed} ft."))
            .block(
                Block::default()
                    .title("Walking Speed")
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Center),
        header_layout[4],
    );
    frame.render_widget(
        Paragraph::new(format!("{initiative}"))
            .block(
                Block::default()
                    .title("Init")
                    .borders(Borders::ALL)
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Center),
        header_layout[5],
    );
    frame.render_widget(
        Paragraph::new(format!("{armor_class}"))
            .block(
                Block::default()
                    .title("AC")
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Center),
        header_layout[6],
    );
}

fn render_abilities<B: Backend>(frame: &mut Frame<B>, area: Rect, abilities: &Abilities) {
    let abilities_block = Block::default().title("Abilities").borders(Borders::ALL);
    let abilities_inner = abilities_block.inner(area);
    frame.render_widget(abilities_block, area);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([
            Constraint::Percentage(2),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(2),
        ])
        .split(abilities_inner);

    render_ability(
        frame,
        layout[1],
        Ability::Strength.to_string(),
        abilities.get_score(Ability::Strength).unwrap_or(0),
    );
    render_ability(
        frame,
        layout[2],
        Ability::Dexterity.to_string(),
        abilities.get_score(Ability::Dexterity).unwrap_or(0),
    );
    render_ability(
        frame,
        layout[3],
        Ability::Constitution.to_string(),
        abilities.get_score(Ability::Constitution).unwrap_or(0),
    );
    render_ability(
        frame,
        layout[4],
        Ability::Intelligence.to_string(),
        abilities.get_score(Ability::Intelligence).unwrap_or(0),
    );
    render_ability(
        frame,
        layout[5],
        Ability::Wisdom.to_string(),
        abilities.get_score(Ability::Wisdom).unwrap_or(0),
    );
    render_ability(
        frame,
        layout[6],
        Ability::Charisma.to_string(),
        abilities.get_score(Ability::Charisma).unwrap_or(0),
    );
}

fn render_ability<B: Backend>(
    frame: &mut Frame<B>,
    area: Rect,
    title: impl Into<String>,
    score: usize,
) {
    let ability_block = Paragraph::new(score.to_string())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title.into())
                .title_alignment(Alignment::Center),
        )
        .alignment(Alignment::Center);
    frame.render_widget(ability_block, area);
}
