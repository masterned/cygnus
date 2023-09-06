use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use cygnus::{
    ability::{Abilities, AbilitiesTemplate, Ability},
    character::{self, Conformity, Gender, Morality},
    class::{self, HPIncreases},
    feat::Feat,
    item::{self, ArmorClass},
    personality::Personality,
    race::{self, Condition, DamageType, Language},
    slot::Slot,
    spell::SpellList,
    view::tui::render_character,
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

fn ui<B: Backend>(f: &mut Frame<B>) {
    let race = race::Builder::new()
        .name("Haskellian")
        .add_ability(Ability::Intelligence, 2)
        .add_ability(Ability::Dexterity, 1)
        .add_damage_resistance(DamageType::Necrotic)
        .add_condition_immunity(Condition::MagicalSleep)
        .add_language(Language::Common)
        .add_language(Language::Undercommon)
        .add_feat(Feat::new(
            "Trance",
            "Short long rests & two proficiency options each long rest.",
        ))
        .add_feat(Feat::new(
            "Explaining a Monad",
            "Misty Step + damage resistance (Prof mod / day)",
        ))
        .build()
        .expect("You broke your race.");

    let spell_list = SpellList::default();

    let hp_increases = HPIncreases::try_from(vec![8, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5])
        .unwrap_or_else(|e| panic!("Unable to create HP Increases: {e}"));

    let personality = Personality::default()
        .add_trait("I always have a plan for what to do when things go wrong.")
        .add_trait("I am incredibly slow to trust. Those who seem the fairest often have the most to hide.")
        .add_ideal("Honor. I don’t steal from others in the trade. (Lawful)")
        .add_bond("Something important was taken from me, and I aim to steal it back.")
        .add_bond("Someone I loved died because of a mistake I made. That will never happen again.")
        .add_flaw("I turn tail and run when things look bad.");

    let artificer = class::Builder::new()
        .name("Artificer")
        .and_then(|c| c.level(12))
        .and_then(|c| c.add_saving_throw_proficiency(Ability::Constitution))
        .and_then(|c| c.spell_list(spell_list))
        .and_then(|c| c.hp_increases(hp_increases))
        .and_then(|c| c.build())
        .unwrap_or_else(|e| panic!("Unable to build class: {e}"));

    let mut character = character::Builder::new()
        .name("𝛴𝜄𝛾𝜈𝜐𝜍")
        .and_then(|c| c.alignment(Conformity::Lawful, Morality::Neutral))
        .and_then(|c| c.gender(Gender::Male))
        .and_then(|c| c.personality(personality))
        .and_then(|c| c.race(race))
        .and_then(|c| {
            c.ability_scores(Abilities::from(AbilitiesTemplate {
                strength: Some(10),
                dexterity: Some(15),
                constitution: Some(10),
                intelligence: Some(15),
                wisdom: Some(10),
                charisma: Some(10),
            }))
        })
        .and_then(|c| c.add_class(artificer))
        .and_then(|c| c.add_equipment_slot("armor", Slot::new(|item| item.has_type("armor"))))
        .and_then(|c| c.add_equipment_slot("left hand", Slot::new(|_| true)))
        .and_then(|c| c.add_equipment_slot("cloak", Slot::new(|_| true)))
        .and_then(|c| c.add_equipment_slot("armor", Slot::new(|item| item.has_type("armor"))))
        .and_then(|c| c.build())
        .unwrap_or_else(|e| panic!("Unable to build character: {e}"));

    let mithral_plate = item::Builder::new()
        .set_name("Mithral Plate")
        .set_weight(65)
        .add_type("armor")
        .set_armor_class(ArmorClass::Heavy(18))
        .build()
        .expect("There shouldn't be an error here.");
    character
        .equip_item(mithral_plate, "armor")
        .expect("Make sure you have the right slot.");

    let shield = item::Builder::new()
        .set_name("Shield")
        .set_weight(6)
        .set_armor_class(ArmorClass::Heavy(2))
        .build()
        .expect("The shield shouldn't error out either.");
    character
        .equip_item(shield, "left hand")
        .expect("Do you have a left hand?");

    let cloak_of_protection = item::Builder::new()
        .set_name("Cloak of Protection")
        .set_armor_class(ArmorClass::Heavy(1))
        .build()
        .expect("Cloak shouldn't mess up.");
    character
        .equip_item(cloak_of_protection, "cloak")
        .expect("Do you have a cloak slot?");

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
