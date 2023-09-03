use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use cygnus::{
    ability::{Abilities, AbilitiesTemplate, Ability},
    character::{Character, Conformity, Gender, Morality, Personality},
    class::{self, Class, Classes, HPIncreases},
    item::{ArmorClass, Item, Items},
    modifiers::Proficiency,
    race::{self, CreatureType, Language, Race, Size},
    skill::Skills,
    slot::{ItemSlots, Slot},
    spell::SpellList,
    view::tui::render_character,
};
use std::{
    collections::HashMap,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

fn ui<B: Backend>(f: &mut Frame<B>) {
    let mut character = Character {
        name: "𝛴𝜄𝛾𝜈𝜐𝜍".into(),
        alignment: (Conformity::Lawful, Morality::Neutral),
        gender: Some(Gender::Male),
        personality: Personality {
            personality_traits: vec![],
            ideals: vec![],
            bonds: vec![],
            flaws: vec![],
        },
        race: Race::from(race::Template {
            name: "Haskellian".into(),
            creature_type: CreatureType::Humanoid,
            size: Size::Medium,
            walking_speed: 30,
            abilities: Abilities::default(),
            damage_resistances: HashMap::new(),
            condition_resistances: HashMap::new(),
            languages: vec![Language::Common],
        }),
        abilities: Abilities::from(AbilitiesTemplate {
            strength: Some(10),
            dexterity: Some(16),
            constitution: Some(19),
            intelligence: Some(20),
            wisdom: Some(10),
            charisma: Some(10),
        }),
        classes: Classes::default(),
        skills: Skills::default(),
        inventory: Items::default(),
        exhaustion_level: 0,
        damage: 0,
        equipment: ItemSlots::default(),
    };
    character.add_class(
        Class::try_from(class::Template {
            name: "Artificer".into(),
            level: 12,
            saving_throw_proficiencies: HashMap::from([
                (Ability::Constitution, Proficiency::Proficiency),
                (Ability::Intelligence, Proficiency::Proficiency),
            ]),
            spell_list: Some(SpellList::default()),
            hp_increases: HPIncreases::try_from(vec![8, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]).unwrap(),
            feats: vec![],
        })
        .unwrap(),
    );
    character.add_equipment_slot("armor", Slot::new(|item| item.has_type("armor")));
    character.add_equipment_slot("left hand", Slot::new(|_| true));
    character.add_equipment_slot("cloak", Slot::new(|_| true));
    character
        .equip_item(
            Item::new(
                "Mithral Plate",
                65,
                vec!["armor".into()],
                Some(ArmorClass::Heavy(18)),
            ),
            "armor",
        )
        .expect("Make sure you have the right slot.");
    character
        .equip_item(
            Item::new("Shield", 6, vec![], Some(ArmorClass::Heavy(2))),
            "left hand",
        )
        .expect("Do you have a left hand?");
    character
        .equip_item(
            Item::new("Cloak of Protection", 0, vec![], Some(ArmorClass::Heavy(1))),
            "cloak",
        )
        .expect("Did have a cloak slot?");

    render_character(f, f.size(), &character);
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

fn restore_terminal(mut terminal: Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, tick_rate: Duration) -> io::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(ui)?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = setup_terminal()?;

    let tick_rate = Duration::from_millis(250);

    run_app(&mut terminal, tick_rate)?;

    restore_terminal(terminal)
}
