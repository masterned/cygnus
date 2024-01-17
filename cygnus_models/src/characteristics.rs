#![warn(clippy::pedantic)]

use std::{error::Error, fmt};

use crate::{
    character::{Alignment, Gender},
    race::Size,
    units::{Distance, Duration, Weight},
};

pub struct Characteristics {
    alignment: Alignment,
    gender: Option<Gender>,
    size: Size,
    eye_color: String,
    height: Vec<Distance>,
    faith: Option<String>,
    hair_color: String,
    skin_tone: String,
    age: Duration,
    weight: Weight,
}

impl Characteristics {
    #[must_use]
    pub fn get_alignment(&self) -> Alignment {
        self.alignment
    }

    #[must_use]
    pub fn get_gender(&self) -> Option<Gender> {
        self.gender
    }

    #[must_use]
    pub fn get_size(&self) -> Size {
        self.size
    }

    #[must_use]
    pub fn get_eye_color(&self) -> &str {
        &self.eye_color
    }

    #[must_use]
    pub fn get_height(&self) -> &[Distance] {
        &self.height
    }

    #[must_use]
    pub fn get_faith(&self) -> Option<&str> {
        self.faith.as_deref()
    }

    #[must_use]
    pub fn get_hair_color(&self) -> &str {
        &self.hair_color
    }

    #[must_use]
    pub fn get_skin_tone(&self) -> &str {
        &self.skin_tone
    }

    #[must_use]
    pub fn get_age(&self) -> Duration {
        self.age
    }

    #[must_use]
    pub fn weight(&self) -> Weight {
        self.weight
    }
}

#[derive(Clone, Debug, Default)]
pub struct Builder {
    alignment: Option<Alignment>,
    gender: Option<Gender>,
    size: Option<Size>,
    eye_color: Option<String>,
    height: Option<Vec<Distance>>,
    faith: Option<String>,
    hair_color: Option<String>,
    skin_tone: Option<String>,
    age: Option<Duration>,
    weight: Option<Weight>,
}

impl Builder {
    #[must_use]
    pub fn new() -> Self {
        Builder::default()
    }

    #[must_use]
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        let _ = self.alignment.insert(alignment);

        self
    }

    #[must_use]
    pub fn gender(mut self, gender: Gender) -> Self {
        let _ = self.gender.insert(gender);

        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        let _ = self.size.insert(size);

        self
    }

    #[must_use]
    pub fn eye_color(mut self, eye_color: impl Into<String>) -> Self {
        let _ = self.eye_color.insert(eye_color.into());

        self
    }

    #[must_use]
    pub fn height(mut self, height: Vec<Distance>) -> Self {
        let _ = self.height.insert(height);

        self
    }
    #[must_use]
    pub fn faith(mut self, faith: impl Into<String>) -> Self {
        let _ = self.faith.insert(faith.into());

        self
    }

    #[must_use]
    pub fn hair_color(mut self, hair_color: impl Into<String>) -> Self {
        let _ = self.hair_color.insert(hair_color.into());

        self
    }

    #[must_use]
    pub fn skin_tone(mut self, skin_tone: impl Into<String>) -> Self {
        let _ = self.skin_tone.insert(skin_tone.into());

        self
    }

    #[must_use]
    pub fn age(mut self, age: Duration) -> Self {
        let _ = self.age.insert(age);

        self
    }

    #[must_use]
    pub fn weight(mut self, weight: Weight) -> Self {
        let _ = self.weight.insert(weight);

        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BuildError {
    MissingFields(Vec<String>),
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Unable to build Characteristics:\n\t{}",
            match self {
                BuildError::MissingFields(missing_fields) => missing_fields
                    .iter()
                    .map(|f| format!("`{f}`"))
                    .collect::<Vec<_>>()
                    .join(", "),
            }
        )
    }
}

impl Error for BuildError {}

impl TryFrom<Builder> for Characteristics {
    type Error = BuildError;

    fn try_from(value: Builder) -> Result<Self, Self::Error> {
        let mut missing_fields = None;

        if value.alignment.is_none() {
            missing_fields
                .get_or_insert(vec![])
                .push(String::from("alignment"));
        }

        if value.size.is_none() {
            missing_fields
                .get_or_insert(vec![])
                .push(String::from("size"));
        }

        if value.eye_color.is_none() {
            missing_fields
                .get_or_insert(vec![])
                .push(String::from("eye_color"));
        }

        if value.height.is_none() {
            missing_fields
                .get_or_insert(vec![])
                .push(String::from("height"));
        }

        if value.hair_color.is_none() {
            missing_fields
                .get_or_insert(vec![])
                .push(String::from("hair_color"));
        }

        if value.skin_tone.is_none() {
            missing_fields
                .get_or_insert(vec![])
                .push(String::from("skin_tone"));
        }

        if value.age.is_none() {
            missing_fields
                .get_or_insert(vec![])
                .push(String::from("age"));
        }

        if value.weight.is_none() {
            missing_fields
                .get_or_insert(vec![])
                .push(String::from("weight"));
        }

        if let Some(missing_fields) = missing_fields {
            return Err(BuildError::MissingFields(missing_fields));
        }

        Ok(Characteristics {
            alignment: value.alignment.unwrap(),
            gender: value.gender,
            size: value.size.unwrap(),
            eye_color: value.eye_color.unwrap(),
            height: value.height.unwrap(),
            faith: value.faith,
            hair_color: value.hair_color.unwrap(),
            skin_tone: value.skin_tone.unwrap(),
            age: value.age.unwrap(),
            weight: value.weight.unwrap(),
        })
    }
}
