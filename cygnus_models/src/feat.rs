#[derive(Clone, Debug, PartialEq)]
pub struct Feat {
    name: String,
    description: String,
}

impl Feat {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Feat {
            name: name.into(),
            description: description.into(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}
