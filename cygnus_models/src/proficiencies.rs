use crate::race::Language;

#[derive(Clone, Debug, Default)]
pub struct Proficiencies {
    armor: Vec<String>,
    weapons: Vec<String>,
    tools: Vec<String>,
    languages: Vec<Language>,
}

impl Proficiencies {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_armor_proficiency(&mut self, armor_class: impl Into<String>) -> &Self {
        self.armor.push(armor_class.into());

        self
    }

    pub fn get_armor_proficiencies(&self) -> &[String] {
        &self.armor
    }

    pub fn get_armor_proficiencies_string(&self) -> String {
        self.armor.join(", ")
    }

    pub fn add_weapon_proficiency(&mut self, weapon: impl Into<String>) -> &Self {
        self.weapons.push(weapon.into());

        self
    }

    pub fn get_weapon_proficiencies_string(&self) -> String {
        self.weapons.join(", ")
    }

    pub fn get_weapon_proficiencies(&self) -> &[String] {
        &self.weapons
    }

    pub fn add_tool_proficiency(&mut self, tool: impl Into<String>) -> &Self {
        self.tools.push(tool.into());

        self
    }

    pub fn get_tool_proficiencies_string(&self) -> String {
        self.tools.join(", ")
    }

    pub fn get_tool_proficiencies(&self) -> &[String] {
        &self.tools
    }

    pub fn add_language(&mut self, language: Language) -> &Self {
        self.languages.push(language);

        self
    }

    pub fn get_languages(&self) -> &[Language] {
        &self.languages
    }
}
