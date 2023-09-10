use std::error;

use cygnus_models::{
    ability::{self, Abilities, AbilitiesTemplate},
    character::{self, Character, Conformity, Gender, Morality},
    class::{self, HPIncreases},
    item::{self, ArmorClass},
    personality::Personality,
    race,
    skill::Skill,
    slot::Slot,
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

        let race = race::Builder::new()
            .name("Haskellian")
            .add_ability(ability::Identifier::Intelligence, 2)
            .add_ability(ability::Identifier::Dexterity, 1)
            .build()?;

        let hp_increases = HPIncreases::try_from(vec![8, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5])?;

        let artificer = class::Builder::new()
            .name("Artificer")?
            .level(12)?
            .hp_increases(hp_increases)?
            .add_saving_throw_proficiency(ability::Identifier::Intelligence)?
            .add_saving_throw_proficiency(ability::Identifier::Constitution)?
            .build()?;

        let mut character = character::Builder::new()
            .name("𝛴𝜄𝛾𝜈𝜐𝜍")?
            .alignment(Conformity::Lawful, Morality::Neutral)?
            .gender(Gender::Male)?
            .personality(personality)?
            .race(race)?
            .base_ability_scores(Abilities::from(AbilitiesTemplate {
                strength: 10,
                dexterity: 15,
                constitution: 10,
                intelligence: 15,
                wisdom: 10,
                charisma: 10,
            }))?
            .add_class(artificer)?
            .add_skill_proficiency(Skill::Arcana)?
            .add_skill_proficiency(Skill::Insight)?
            .add_skill_proficiency(Skill::Investigation)?
            .add_skill_proficiency(Skill::Perception)?
            .add_skill_proficiency(Skill::Stealth)?
            .add_equipment_slot("armor", Slot::new(|item| item.has_type("armor")))?
            .add_equipment_slot("cloak", Slot::new(|item| item.has_type("cloak")))?
            .add_equipment_slot("left hand", Slot::new(|item| item.has_type("hand")))?
            .build()?;

        let mithral_plate = item::Builder::new()
            .name("Mithral Plate")?
            .weight(65)?
            .armor_class(ArmorClass::Heavy(18))?
            .add_type("armor")?
            .build()?;
        character.equip_item(mithral_plate, "armor")?;

        let cloak_of_protection = item::Builder::new()
            .name("Cloak of Protection")?
            .armor_class(ArmorClass::Heavy(1))?
            .add_type("cloak")?
            .build()?;
        character.equip_item(cloak_of_protection, "cloak")?;

        let shield = item::Builder::new()
            .name("Shield")?
            .armor_class(ArmorClass::Heavy(2))?
            .weight(6)?
            .add_type("hand")?
            .build()?;
        character.equip_item(shield, "left hand")?;

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
