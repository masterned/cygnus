#![warn(clippy::pedantic)]

#[derive(Debug)]
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
    use std::ops::Range;

    #[derive(Debug)]
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
    }

    #[derive(Debug, Default)]
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
            let name = value
                .name
                .ok_or_else(|| Self::Error::MissingField(String::from("name")))?;
            let order = value
                .order
                .ok_or_else(|| Self::Error::MissingField(String::from("order")))?;
            let description = value
                .description
                .ok_or_else(|| Self::Error::MissingField(String::from("description")))?;
            let focus = value
                .focus
                .ok_or_else(|| Self::Error::MissingField(String::from("focus")))?;

            Ok(Discipline {
                name,
                order,
                description,
                focus,
                acts: value.acts,
            })
        }
    }

    pub enum BuildError {
        MissingField(String),
    }

    #[derive(Clone, Debug)]
    pub struct Act {
        name: String,
        description: String,
        cost: Range<usize>,
    }

    impl Act {
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
    }
}
