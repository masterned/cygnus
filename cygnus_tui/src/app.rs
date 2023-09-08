use std::error;

use cygnus_models::{
    ability::{Abilities, AbilitiesTemplate},
    character::{self, Character, Conformity, Gender, Morality},
    class,
    personality::Personality,
    race,
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,

    pub character: Option<Character>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            character: None,
        }
    }
}

impl App {
    fn create_character(&mut self) -> Result<(), Box<dyn error::Error>> {
        let personality = Personality::default()
            .add_trait("I always have a plan for what to do when things go wrong.");

        let race = race::Builder::new().name("Haskellian").build()?;

        let artificer = class::Builder::new()
            .name("Artificer")?
            .level(12)?
            .build()?;

        let character = character::Builder::new()
            .name("𝛴𝜄𝛾𝜈𝜐𝜍")?
            .alignment(Conformity::Lawful, Morality::Neutral)?
            .gender(Gender::Male)?
            .personality(personality)?
            .race(race)?
            .ability_scores(Abilities::from(AbilitiesTemplate {
                strength: Some(10),
                dexterity: Some(15),
                constitution: Some(10),
                intelligence: Some(15),
                wisdom: Some(10),
                charisma: Some(10),
            }))?
            .add_class(artificer)?
            .build()?;

        self.character = Some(character);

        Ok(())
    }
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let mut app = Self::default();

        app.create_character()
            .unwrap_or_else(|err| panic!("Failed to create Character: {err}"));

        app
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
