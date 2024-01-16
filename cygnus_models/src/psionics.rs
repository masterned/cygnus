#![warn(clippy::pedantic)]

use std::{error::Error, fmt};

use self::discipline::{Act, Discipline};

struct Mystic {
    psi_points: usize,
    psi_point_max: usize,
    talents: Vec<Talent>,
    disciplines: Vec<Discipline>,
    focus: Option<Discipline>,
}

impl Psionics for Mystic {
    fn get_talents(&self) -> &[Talent] {
        &self.talents
    }

    fn get_disciplines(&self) -> &[Discipline] {
        &self.disciplines
    }

    fn get_psi_points(&self) -> usize {
        self.psi_points
    }

    fn get_psi_points_mut(&mut self) -> &mut usize {
        &mut self.psi_points
    }

    fn get_focus(&self) -> Option<&Discipline> {
        self.focus.as_ref()
    }

    fn get_focus_mut(&mut self) -> &mut Option<Discipline> {
        &mut self.focus
    }

    fn get_psi_point_max(&self) -> usize {
        self.psi_point_max
    }
}

trait Psionics {
    fn get_talents(&self) -> &[Talent];

    fn get_disciplines(&self) -> &[Discipline];

    fn get_psi_points(&self) -> usize;

    fn get_psi_points_mut(&mut self) -> &mut usize;

    fn get_psi_point_max(&self) -> usize;

    fn reset_psi_points(&mut self) {
        *self.get_psi_points_mut() = self.get_psi_point_max();
    }

    fn get_focus(&self) -> Option<&Discipline>;

    fn get_focus_mut(&mut self) -> &mut Option<Discipline>;

    fn focus_on(&mut self, d: &Discipline) {
        let _ = self.get_focus_mut().insert(d.clone());
    }

    fn has_act(&self, a: &Act) -> bool {
        self.get_disciplines().iter().any(|d| d.has_act(a))
    }

    fn perform_act(&mut self, a: &Act, charge_mod: usize) -> Result<(), ActError> {
        if !self.has_act(a) {
            return Err(ActError::ActNotPracticed);
        }

        let psi_points_mut = self.get_psi_points_mut();

        let act_cost = a.get_cost();

        if *psi_points_mut < act_cost.start {
            return Err(ActError::NotEnoughPoints);
        }

        if act_cost.start + charge_mod >= act_cost.end {
            return Err(ActError::Overcharged);
        }

        *psi_points_mut -= act_cost.start + charge_mod;

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActError {
    NotEnoughPoints,
    ActNotPracticed,
    Overcharged,
}

impl fmt::Display for ActError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cannot perform Act: {}",
            match self {
                Self::NotEnoughPoints => "not enough psi points",
                Self::ActNotPracticed => "user doesn't know act",
                Self::Overcharged => "act unable to handle psi points",
            }
        )
    }
}

impl Error for ActError {}

#[derive(Clone, Debug)]
pub struct Talent {
    name: String,
    description: String,
}

impl Talent {
    #[must_use]
    pub fn new(name: &str, description: &str) -> Self {
        Talent {
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

pub mod discipline {
    use super::{fmt, Error};

    use std::ops::Range;

    use crate::units::Duration;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Discipline {
        name: String,
        order: String,
        description: String,
        focus: String,
        acts: Vec<Act>,
    }

    impl Discipline {
        #[must_use]
        pub fn get_name(&self) -> &str {
            &self.name
        }

        #[must_use]
        pub fn get_order(&self) -> &str {
            &self.order
        }

        #[must_use]
        pub fn get_description(&self) -> &str {
            &self.description
        }

        #[must_use]
        pub fn get_focus(&self) -> &str {
            &self.focus
        }

        #[must_use]
        pub fn get_acts(&self) -> &[Act] {
            &self.acts
        }

        #[must_use]
        pub fn has_act(&self, a: &Act) -> bool {
            self.acts.contains(a)
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct Builder {
        name: Option<String>,
        order: Option<String>,
        description: Option<String>,
        focus: Option<String>,
        acts: Vec<Act>,
    }

    impl Builder {
        #[must_use]
        pub fn new() -> Self {
            Builder::default()
        }

        #[must_use]
        pub fn name(mut self, name: &str) -> Self {
            let _ = self.name.insert(name.into());

            self
        }

        #[must_use]
        pub fn order(mut self, order: &str) -> Self {
            let _ = self.order.insert(order.into());

            self
        }

        #[must_use]
        pub fn description(mut self, description: &str) -> Self {
            let _ = self.description.insert(description.into());

            self
        }

        #[must_use]
        pub fn focus(mut self, focus: &str) -> Self {
            let _ = self.focus.insert(focus.into());

            self
        }

        #[must_use]
        pub fn add_act(mut self, act: &Act) -> Self {
            self.acts.push(act.clone());

            self
        }
    }

    impl TryFrom<Builder> for Discipline {
        type Error = BuildError;

        fn try_from(value: Builder) -> Result<Self, Self::Error> {
            let mut missing_fields = None;

            if value.name.is_none() {
                missing_fields.get_or_insert(vec![]).push("name");
            }
            if value.order.is_none() {
                missing_fields.get_or_insert(vec![]).push("order");
            }
            if value.description.is_none() {
                missing_fields.get_or_insert(vec![]).push("description");
            }
            if value.focus.is_none() {
                missing_fields.get_or_insert(vec![]).push("focus");
            }
            if let Some(missing_fields) = missing_fields {
                return Err(BuildError::new_missing_fields_error(&missing_fields));
            }

            Ok(Discipline {
                name: value.name.unwrap(),
                order: value.order.unwrap(),
                description: value.description.unwrap(),
                focus: value.focus.unwrap(),
                acts: value.acts,
            })
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum BuildError {
        MissingField(Vec<String>),
    }

    impl BuildError {
        pub fn new_missing_fields_error(missing_fields: &[impl Into<String> + Clone]) -> Self {
            Self::MissingField(missing_fields.iter().map(|f| (*f).clone().into()).collect())
        }
    }

    impl fmt::Display for BuildError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Unable to build Discipline:\n{}",
                match self {
                    Self::MissingField(fields) => format!(
                        "\tmissing field(s): {}",
                        fields
                            .iter()
                            .map(|f| format!("`{f}`"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                }
            )
        }
    }

    impl Error for BuildError {}

    #[derive(Clone, Debug, PartialEq)]
    pub struct Act {
        name: String,
        description: String,
        cost: Range<usize>,
        duration: Option<Duration>,
    }

    impl Act {
        #[must_use]
        pub fn new(
            name: impl Into<String>,
            description: impl Into<String>,
            cost: Range<usize>,
            duration: Option<Duration>,
        ) -> Self {
            Act {
                name: name.into(),
                description: description.into(),
                cost,
                duration,
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

        #[must_use]
        pub fn get_cost(&self) -> &Range<usize> {
            &self.cost
        }

        #[must_use]
        pub fn get_duration(&self) -> &Option<Duration> {
            &self.duration
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        mod builder {
            use super::*;

            #[test]
            fn _should_succeed_when_all_fields_filled() -> Result<(), Box<dyn Error>> {
                let builder = Builder::new()
                    .name("Psychic Phantoms")
                    .order("Awakened")
                    .description("Your power reaches into the creature's mind and creates false perceptions.")
                    .focus("While focused on this discipline, you have advantage on all Charisma (Deception) checks.")
                    .add_act(&Act {
                        name: String::from("Distracting Figment"),
                        description: String::from("As an action, choose one creature you can see within 60 feet of you. The target must make an Intelligence saving throw. On a failed save, it takes 1d10 psychic damage per psi point spent and thinks it perceives a threatening creature just out of its sight; until the end of your next turn, it can’t use reactions, and melee attack rolls against it have advantage. On a successful save, it takes half as much damage."),
                        cost: 1..8,
                        duration: None }
                    );

                let built_discipline: Discipline = builder.try_into()?;

                assert_eq!(built_discipline,
                    Discipline {
                        name: String::from("Psychic Phantoms"),
                        order: String::from("Awakened"),
                        description: String::from("Your power reaches into the creature's mind and creates false perceptions."),
                        focus: String::from("While focused on this discipline, you have advantage on all Charisma (Deception) checks."),
                        acts: vec![
                            Act {
                                name: String::from("Distracting Figment"),
                                description: String::from("As an action, choose one creature you can see within 60 feet of you. The target must make an Intelligence saving throw. On a failed save, it takes 1d10 psychic damage per psi point spent and thinks it perceives a threatening creature just out of its sight; until the end of your next turn, it can’t use reactions, and melee attack rolls against it have advantage. On a successful save, it takes half as much damage."),
                                cost: 1..8,
                                duration: None
                            }
                        ]
                    }
                );

                Ok(())
            }

            #[test]
            fn _should_return_error_when_missing_field() {
                let empty_buider = Builder::new();
                assert_eq!(
                    Discipline::try_from(empty_buider),
                    Err(BuildError::new_missing_fields_error(&[
                        "name",
                        "order",
                        "description",
                        "focus"
                    ]))
                );

                let named_builder = Builder::new().name("Test");
                assert_eq!(
                    Discipline::try_from(named_builder),
                    Err(BuildError::new_missing_fields_error(&[
                        "order",
                        "description",
                        "focus"
                    ]))
                );

                let named_ordered_builder = Builder::new().name("Test").order("Test");
                assert_eq!(
                    Discipline::try_from(named_ordered_builder),
                    Err(BuildError::new_missing_fields_error(&[
                        "description",
                        "focus"
                    ]))
                );
            }

            mod build_error {
                use super::*;

                #[test]
                fn _should_handle_multiple_missing_fields() {
                    let build_error =
                        BuildError::new_missing_fields_error(&["name", "order", "description"]);

                    assert_eq!(build_error.to_string(), "Unable to build Discipline:\n\tmissing field(s): `name`, `order`, `description`");
                }
            }
        }
    }
}
