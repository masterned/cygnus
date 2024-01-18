#![warn(clippy::pedantic)]

use cygnus_models::{
    ability, background,
    character::Character,
    modifiers::Proficiency,
    psionics::discipline::{self, Act, Discipline},
    skills,
    units::Duration,
};
use ratatui::{
    prelude::*,
    widgets::{
        block::{Block, BorderType, Position, Title},
        Borders, Cell, Clear, List, ListItem, ListState, Paragraph, Row, Table, Wrap,
    },
};

use crate::{
    app::App,
    widgets::{BackgroundWidget, CharacteristicsWidget, DisciplineWidget, PersonalityWidget},
};

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

fn render_ability_widget(
    frame: &mut Frame,
    character: &Character,
    ability: ability::Identifier,
    rect: Rect,
) {
    let ability = ability_widget(character, ability);

    frame.render_widget(ability, rect);
}

fn render_abilities(frame: &mut Frame, character: &Character, rect: Rect) {
    let abilities_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 6); 6])
        .split(rect);

    ability::Identifier::all()
        .iter()
        .enumerate()
        .for_each(|(i, &ability)| {
            render_ability_widget(frame, character, ability, abilities_layout[i]);
        });
}

fn render_header(frame: &mut Frame, character: &Character, area: Rect) {
    let header_layout =
        Layout::new(Direction::Vertical, [Constraint::Ratio(1, 2); 2].as_ref()).split(area);

    let first_row_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Ratio(2, 5),
            Constraint::Ratio(1, 5),
            Constraint::Ratio(2, 5),
        ],
    )
    .split(header_layout[0]);

    let name_card_widget = Paragraph::new(format!(
        "{}\n{} {} {}\nLevel {}",
        character.get_name(),
        character
            .get_gender()
            .map_or(String::new(), |g| g.to_string()),
        character.get_race_name(),
        character.get_class_details(),
        character.get_level()
    ))
    .block(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    // header first row
    frame.render_widget(name_card_widget, first_row_layout[0]);
    render_health_block(frame, character, first_row_layout[2]);

    render_second_row(frame, character, header_layout[1]);
}

fn render_health_block(frame: &mut Frame, character: &Character, rect: Rect) {
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

fn render_saving_throw_pair(
    frame: &mut Frame,
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

fn render_saving_throw_row(
    frame: &mut Frame,
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

fn render_saving_throws_block(frame: &mut Frame, character: &Character, rect: Rect) {
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
            render_saving_throw_row(frame, character, pair[0], pair[1], saving_throw_rows[i]);
        });
}

fn render_armor_class(frame: &mut Frame, character: &Character, rect: Rect) {
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

fn render_initiative(frame: &mut Frame, character: &Character, rect: Rect) {
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

fn render_proficiency_bonus(frame: &mut Frame, character: &Character, area: Rect) {
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

fn render_walking_speed(frame: &mut Frame, character: &Character, area: Rect) {
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

fn render_skills_table(frame: &mut Frame, character: &Character, area: Rect) {
    let header_cells = ["Prof", "Mod", "Skill", "Bonus"]
        .iter()
        .map(|&h| Cell::from(h));
    let header = Row::new(header_cells).height(1).bottom_margin(1);
    let skills = skills::Identifier::all();
    let rows = skills.iter().map(|&id| {
        let cells = [
            Cell::from(match character.get_skill_proficiency(id) {
                Some(Proficiency::Proficiency) => "x",
                Some(Proficiency::Expertise) => "*",
                None => "o",
            }),
            Cell::from(id.get_default_ability().abbr().to_string()),
            Cell::from(format!("{id}")),
            Cell::from(format!("{:+}", character.get_skill_modifier(id))),
        ];
        Row::new(cells)
    });
    let table = Table::new(rows, [Constraint::Ratio(1, 4); 4].as_ref())
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
        );

    frame.render_widget(table, area);
}

fn render_second_row(frame: &mut Frame, character: &Character, area: Rect) {
    let second_row_layout =
        Layout::new(Direction::Horizontal, [Constraint::Ratio(1, 4); 4]).split(area);

    render_proficiency_bonus(frame, character, second_row_layout[0]);
    render_walking_speed(frame, character, second_row_layout[1]);
    render_initiative(frame, character, second_row_layout[2]);
    render_armor_class(frame, character, second_row_layout[3]);
}

fn render_senses_block(frame: &mut Frame, character: &Character, area: Rect) {
    let senses_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(
            Title::from("Senses")
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        );
    frame.render_widget(senses_block.clone(), area);

    let senses_layout = Layout::default()
        .constraints([Constraint::Ratio(1, 4); 4])
        .split(senses_block.inner(area));

    let passive_perception_row_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Ratio(1, 3), Constraint::Min(0)],
    )
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

    let passive_investigation_row_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Ratio(1, 3), Constraint::Min(0)],
    )
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

    let passive_insight_row_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Ratio(1, 3), Constraint::Min(0)],
    )
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
            Paragraph::new(format!("Darkvision {darkvision} ft.")).alignment(Alignment::Center),
            senses_layout[3],
        );
    }
}

fn render_proficiencies_and_languages_block(frame: &mut Frame, character: &Character, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(
            Title::from("Proficiencies & Languages")
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        );
    frame.render_widget(block.clone(), area);

    let layout =
        Layout::new(Direction::Vertical, [Constraint::Ratio(1, 4); 4]).split(block.inner(area));

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

fn render_inventory_table(frame: &mut Frame, _character: &Character, area: Rect) {
    let header_cells = [
        "Equipped",
        "Attuned",
        "Name",
        "Types",
        "Weight",
        "Qty",
        "Cost (GP)",
    ]
    .iter()
    .map(|&h| Cell::from(h));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = [
        Row::new([
            Cell::from("*"),
            Cell::from("*"),
            Cell::from("Amulet of Health"),
            Cell::from("Wonderous Item"),
            Cell::from("--"),
            Cell::from("--"),
            Cell::from("--"),
        ]),
        Row::new([
            Cell::from("*"),
            Cell::from("*"),
            Cell::from("Cloak of Protection"),
            Cell::from("Wonderous Item"),
            Cell::from("--"),
            Cell::from("--"),
            Cell::from("--"),
        ]),
        Row::new([
            Cell::from("-"),
            Cell::from("-"),
            Cell::from("Clothes, Traveler's"),
            Cell::from("Gear, Adventuring Gear"),
            Cell::from("4 lb."),
            Cell::from("1"),
            Cell::from("2"),
        ]),
        Row::new([
            Cell::from("*"),
            Cell::from("-"),
            Cell::from("Studded Leather"),
            Cell::from("Light Armor, Studded Leather"),
            Cell::from("13 lb."),
            Cell::from("--"),
            Cell::from("45"),
        ]),
    ];

    let table = Table::new(
        rows,
        [
            Constraint::Max(8),
            Constraint::Max(7),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Max(6),
            Constraint::Max(3),
            Constraint::Max(9),
        ]
        .as_ref(),
    )
    .header(header)
    .block(
        Block::new()
            .title(
                Title::from("Inventory")
                    .alignment(Alignment::Center)
                    .position(Position::Top),
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .column_spacing(1);

    frame.render_widget(table, area);
}

fn render_description_page(frame: &mut Frame, character: &Character, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Title::from("Description").alignment(Alignment::Center));

    let layout = Layout::default()
        .constraints(
            [
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 2),
                Constraint::Ratio(1, 4),
            ]
            .as_ref(),
        )
        .split(block.clone().inner(area));

    frame.render_widget(block, area);

    let background: BackgroundWidget = <background::Builder as TryInto<background::Background>>::try_into(background::Builder::new()
        .name("Urban Bounty Hunter")
        .description("You did bounty hunter stuff in an urban setting.")
        .feature(
            background::Feature::new(
                "Ear to the Ground",
                "You are in frequent contact with people in the segment of society that your chosen quarries move through. These people might be associated with the criminal underworld, the rough-and-tumble folk of the streets, or members of high society. This connection comes in the form of a contact in any city you visit, a person who provides information about the people and places of the local area."
            )
        )
        .proficiencies(
            background::Proficiencies::two_skills_two_tools(
                skills::Identifier::Deception,
                skills::Identifier::Persuasion,
                "Dice Set",
                "Card Deck"
            )
        ))
        .unwrap()
        .into();
    frame.render_widget(background, layout[0]);

    let characteristics_block = Block::new()
        .title(Title::from("Characteristics"))
        .borders(Borders::TOP);

    let characteristics_layout =
        Layout::new(Direction::Horizontal, [Constraint::Ratio(1, 2); 2].as_ref())
            .split(characteristics_block.clone().inner(layout[1]));

    frame.render_widget(characteristics_block, layout[1]);

    let characteristics_widget: CharacteristicsWidget =
        character.get_characteristics().clone().into();
    frame.render_widget(characteristics_widget, characteristics_layout[0]);

    let personality_widget: PersonalityWidget = character.get_personality().clone().into();
    frame.render_widget(personality_widget, characteristics_layout[1]);

    let appearance_block = Block::new()
        .borders(Borders::TOP)
        .title(Title::from("Appearance"));
    frame.render_widget(appearance_block, layout[2]);
}

enum PageLink {
    AbilitiesSavesSenses,
    Skills,
    Actions,
    Inventory,
    Spells,
    FeaturesTraits,
    ProficienciesLanguages,
    Description,
    Notes,
    ExtrasCreatures,
}

impl From<usize> for PageLink {
    fn from(value: usize) -> Self {
        match value {
            1 => PageLink::Skills,
            2 => PageLink::Actions,
            3 => PageLink::Inventory,
            4 => PageLink::Spells,
            5 => PageLink::FeaturesTraits,
            6 => PageLink::ProficienciesLanguages,
            7 => PageLink::Description,
            8 => PageLink::Notes,
            9 => PageLink::ExtrasCreatures,
            _ => PageLink::AbilitiesSavesSenses,
        }
    }
}

/// Renders the user interface widgets.
///
/// # Panics
///
/// If the `Character` model has not been created yet.
///
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let character = app
        .character
        .as_mut()
        .expect("Can't render a `Character` if it doesn't exist.");

    let document_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 5),
                Constraint::Min(0),
                Constraint::Ratio(1, 10),
            ]
            .as_ref(),
        )
        .split(frame.size());

    render_header(frame, character, document_layout[0]);

    match app.nav_menu_state.selected.into() {
        PageLink::AbilitiesSavesSenses => {
            let body_layout = Layout::new(
                Direction::Vertical,
                [
                    Constraint::Max(4),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ]
                .as_ref(),
            )
            .split(document_layout[1]);

            render_abilities(frame, character, body_layout[0]);
            render_saving_throws_block(frame, character, body_layout[1]);
            render_senses_block(frame, character, body_layout[2]);
        }
        PageLink::Actions => {
            let act = Act::new("Phantom Foe", "As an action, choose one creature you can see within 60 feet of you. The target must make an Intelligence saving throw. On a failed save, it perceives a horrid creature adjacent to it until your concentration ends. During this time, the target can’t take reactions, and it takes 1d8 psychic damage at the start of each of its turns. The target can repeat the saving throw at the end of each of its turns, ending the effect on itself on a success. You can increase the damage by 1d8 for each additional psi point spent on the ability.",3..4,Some(Duration::Minutes(1)));

            let d_w = DisciplineWidget::from(Discipline::try_from(
                discipline::Builder::new()
                    .name("Phychic Phantoms")
                    .order("Awakened")
                    .description( "Your power reaches into a creature’s mind and causes it false perceptions.",
                    ).focus("While focused on this discipline, you have advantage on Charisma (Deception) checks.")
                    .add_act(&act),
            ).unwrap());

            frame.render_widget(d_w, document_layout[1]);
        }
        PageLink::Skills => {
            let body_layout = Layout::new(Direction::Vertical, [Constraint::Min(0)].as_ref())
                .split(document_layout[1]);

            render_skills_table(frame, character, body_layout[0]);
        }
        PageLink::Inventory => {
            let body_layout = Layout::default()
                .constraints([Constraint::Min(0)].as_ref())
                .split(document_layout[1]);

            render_inventory_table(frame, character, body_layout[0]);
        }
        PageLink::ProficienciesLanguages => {
            let body_layout = Layout::default()
                .constraints([Constraint::Min(0)].as_ref())
                .split(document_layout[1]);

            render_proficiencies_and_languages_block(frame, character, body_layout[0]);
        }
        PageLink::Description => render_description_page(frame, character, document_layout[1]),
        _ => {}
    }

    if app.nav_menu_state.is_open {
        let items: Vec<ListItem> = [
            "Abilities, Saves, Senses",
            "Skills",
            "Actions",
            "Inventory",
            "Spells",
            "Features & Traits",
            "Proficiencies & Languages",
            "Description",
            "Notes",
            "Extras: Creatures",
        ]
        .iter()
        .map(|&s| ListItem::new(s))
        .collect();
        let list = List::new(items)
            .block(
                Block::new()
                    .title(Title::from("Pages").alignment(Alignment::Center))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .highlight_symbol(">> ");
        let area = centered_rect(60, 80, document_layout[1]);
        let mut list_state = ListState::default().with_selected(Some(app.nav_menu_state.selected));
        frame.render_widget(Clear, area);
        frame.render_stateful_widget(list, area, &mut list_state);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
