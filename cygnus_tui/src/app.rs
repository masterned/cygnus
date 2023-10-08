use std::error;

use cygnus_models::{
    ability::{self, Abilities, AbilitiesTemplate},
    character::{self, Character, Conformity, Gender, Morality},
    class::{self, HPIncreases},
    item::{self, ArmorClass},
    personality::Personality,
    race::{self, Language},
    senses, skills,
    slot::Slot,
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Clone, Debug, Default)]
pub struct NavMenuState {
    pub is_open: bool,
    pub selected: usize,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    pub character: Option<Character>,

    pub nav_menu_state: NavMenuState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            character: None,
            nav_menu_state: NavMenuState::default(),
        }
    }
}

impl App {
    fn create_character(&mut self) -> Result<(), Box<dyn error::Error>> {
        let personality = Personality::default()
            .add_trait("I always have a plan for what to do when things go wrong.")
            .add_trait("I am always calm, no matter what the situation. I never raise my voice or let my emotions control me.")
            .add_ideal("People. I'm loyal to my friends, not to any ideals, and everyone else can take a trip down the Styx for all I care.")
            .add_bond("Someone I loved died because of a mistake I made. That will never happen again.")
            .add_flaw("I turn tail and run when things look bad.");

        let race = race::Builder::new()
            .name("Haskellian")
            .add_ability(ability::Identifier::Intelligence, 2)
            .add_ability(ability::Identifier::Dexterity, 1)
            .add_language(Language::Common)
            .add_language(Language::Undercommon)
            .build()?;

        let hp_increases = HPIncreases::try_from(vec![8, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5])?;

        let artificer = class::Builder::new()
            .name("Artificer")?
            .level(12)?
            .hp_increases(hp_increases)?
            .add_saving_throw_proficiency(ability::Identifier::Intelligence)?
            .add_saving_throw_proficiency(ability::Identifier::Constitution)?
            .build()?;

        let senses = senses::Builder::new().darkvision(60).build();

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
            .add_skill_proficiency(skills::Identifier::Arcana)?
            .add_skill_proficiency(skills::Identifier::Insight)?
            .add_skill_proficiency(skills::Identifier::Investigation)?
            .add_skill_proficiency(skills::Identifier::Perception)?
            .add_skill_proficiency(skills::Identifier::Stealth)?
            .add_equipment_slot("armor", Slot::new(|item| item.has_type("armor")))?
            .add_equipment_slot("cloak", Slot::new(|item| item.has_type("cloak")))?
            .add_equipment_slot("left hand", Slot::new(|item| item.has_type("hand")))?
            .senses(senses)?
            .add_armor_proficiency("Heavy Armor")?
            .add_armor_proficiency("Light Armor")?
            .add_armor_proficiency("Medium Armor")?
            .add_armor_proficiency("Shields")?
            .add_weapon_proficiency("Firearms")?
            .add_weapon_proficiency("Rapier")?
            .add_weapon_proficiency("Simple Weapons")?
            .add_weapon_proficiency("Whip")?
            .add_tool_proficiency("Alchemist's Supplies")?
            .add_tool_proficiency("Playing Card Set")?
            .add_tool_proficiency("Smith's Tools")?
            .add_tool_proficiency("Thieves' Tools")?
            .add_tool_proficiency("Three-Dragon Ante Set")?
            .add_tool_proficiency("Tinker's Tools")?
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

    pub fn toggle_nav_menu(&mut self) {
        self.nav_menu_state.is_open = !self.nav_menu_state.is_open;
    }

    pub fn nav_down(&mut self) {
        if let Some(res) = self.nav_menu_state.selected.checked_add(1) {
            self.nav_menu_state.selected = res % 10;
        }
    }

    pub fn nav_up(&mut self) {
        if let Some(res) = self
            .nav_menu_state
            .selected
            .checked_add(10)
            .and_then(|a| a.checked_sub(1))
        {
            self.nav_menu_state.selected = res % 10;
        }
    }
}
