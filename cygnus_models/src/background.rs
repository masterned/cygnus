#![warn(clippy::pedantic)]

use std::{error::Error, fmt};

use crate::skills;

#[derive(Clone, Debug, PartialEq)]
pub struct Background {
    name: String,
    description: String,
    feature: Feature,
    proficiencies: Proficiencies,
}

impl Background {
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn get_description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub fn get_feature(&self) -> &Feature {
        &self.feature
    }

    #[must_use]
    pub fn get_proficiencies(&self) -> &Proficiencies {
        &self.proficiencies
    }
}

#[derive(Clone, Debug, Default)]
pub struct Builder {
    name: Option<String>,
    description: Option<String>,
    feature: Option<Feature>,
    proficiencies: Option<Proficiencies>,
}

impl Builder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        let _ = self.name.insert(name.into());

        self
    }

    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        let _ = self.description.insert(description.into());

        self
    }

    #[must_use]
    pub fn feature(mut self, feature: Feature) -> Self {
        let _ = self.feature.insert(feature);

        self
    }

    #[must_use]
    pub fn proficiencies(mut self, proficiencies: Proficiencies) -> Self {
        let _ = self.proficiencies.insert(proficiencies);

        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BuildError {
    MissingField(Vec<String>),
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Build Error: {}",
            match self {
                BuildError::MissingField(fields) => format!(
                    "missing field(s): {}",
                    fields
                        .iter()
                        .map(|s| format!("`{s}`"))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            }
        )
    }
}

impl Error for BuildError {}

impl TryInto<Background> for Builder {
    type Error = BuildError;

    fn try_into(self) -> Result<Background, Self::Error> {
        let mut missing_fields = None;

        if self.name.is_none() {
            missing_fields
                .get_or_insert(vec![])
                .push(String::from("name"));
        }

        if self.description.is_none() {
            missing_fields
                .get_or_insert(vec![])
                .push(String::from("description"));
        }

        if self.feature.is_none() {
            missing_fields
                .get_or_insert(vec![])
                .push(String::from("feature"));
        }

        if self.proficiencies.is_none() {
            missing_fields
                .get_or_insert(vec![])
                .push(String::from("proficiencies"));
        }

        if let Some(missing_fields) = missing_fields {
            return Err(BuildError::MissingField(missing_fields));
        }

        Ok(Background {
            name: self.name.unwrap(),
            description: self.description.unwrap(),
            feature: self.feature.unwrap(),
            proficiencies: self.proficiencies.unwrap(),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Feature {
    name: String,
    description: String,
}

impl Feature {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
        }
    }

    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn get_description(&self) -> &str {
        &self.description
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Proficiencies {
    TwoSkillsTwoTools {
        skills: Vec<skills::Identifier>,
        tools: Vec<String>,
    },
    TwoSkillsTwoLanguages {
        skills: Vec<skills::Identifier>,
        languages: Vec<String>,
    },
    TwoSkillsOneLanguageOneTool {
        skills: Vec<skills::Identifier>,
        lanugage: String,
        tool: String,
    },
}

impl Proficiencies {
    pub fn two_skills_two_tools(
        skill1: skills::Identifier,
        skill2: skills::Identifier,
        tool1: impl Into<String>,
        tool2: impl Into<String>,
    ) -> Self {
        Proficiencies::TwoSkillsTwoTools {
            skills: vec![skill1, skill2],
            tools: vec![tool1.into(), tool2.into()],
        }
    }

    pub fn two_skills_two_languages(
        skill1: skills::Identifier,
        skill2: skills::Identifier,
        language1: impl Into<String>,
        language2: impl Into<String>,
    ) -> Self {
        Proficiencies::TwoSkillsTwoLanguages {
            skills: vec![skill1, skill2],
            languages: vec![language1.into(), language2.into()],
        }
    }

    pub fn two_skills_one_tool_one_language(
        skill1: skills::Identifier,
        skill2: skills::Identifier,
        tool: impl Into<String>,
        language: impl Into<String>,
    ) -> Self {
        Proficiencies::TwoSkillsOneLanguageOneTool {
            skills: vec![skill1, skill2],
            lanugage: language.into(),
            tool: tool.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _should_successfully_build_with_all_fields() -> Result<(), Box<dyn Error>> {
        let test_background: Background = Builder::new()
            .name("Test Background")
            .description("This is a Background used for testing")
            .feature(Feature::new(
                "Test Feature",
                "This is a Feature used for testing.",
            ))
            .proficiencies(Proficiencies::two_skills_two_tools(
                skills::Identifier::Acrobatics,
                skills::Identifier::AnimalHandling,
                "Land Vehicles",
                "Smith Tools",
            ))
            .try_into()?;

        assert_eq!(
            test_background,
            Background {
                name: String::from("Test Background"),
                description: String::from("This is a Background used for testing"),
                feature: Feature {
                    name: String::from("Test Feature"),
                    description: String::from("This is a Feature used for testing.")
                },
                proficiencies: Proficiencies::TwoSkillsTwoTools {
                    skills: vec![
                        skills::Identifier::Acrobatics,
                        skills::Identifier::AnimalHandling
                    ],
                    tools: vec![String::from("Land Vehicles"), String::from("Smith Tools")]
                }
            }
        );

        Ok(())
    }
}
