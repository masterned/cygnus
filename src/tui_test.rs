use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use cygnus::{
    ability::{Abilities, AbilitiesTemplate, Ability},
    character::{Character, Conformity, Gender, Morality, Personality},
    class::{self, Class, Classes},
    item::Items,
    modifiers::Proficiency,
    race::{self, CreatureType, Language, Race, Size},
    skill::Skills,
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
        items: Items::default(),
        exhaustion_level: 0,
    };
    character.add_class(
        Class::try_from(class::Template {
            name: "Artificer".into(),
            level: 12,
            saving_throw_proficiencies: HashMap::from([
                (Ability::Constitution, Proficiency::Proficiency),
                (Ability::Intelligence, Proficiency::Proficiency),
            ]),
        })
        .unwrap(),
    );

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
