#[derive(Clone, Debug, Default)]
pub struct Personality {
    pub traits: Vec<String>,
    pub ideals: Vec<String>,
    pub bonds: Vec<String>,
    pub flaws: Vec<String>,
}

impl Personality {
    pub fn add_trait(mut self, personality_trait: impl Into<String>) -> Self {
        self.traits.push(personality_trait.into());

        self
    }

    pub fn add_ideal(mut self, ideal: impl Into<String>) -> Self {
        self.ideals.push(ideal.into());

        self
    }

    pub fn add_bond(mut self, bond: impl Into<String>) -> Self {
        self.bonds.push(bond.into());

        self
    }

    pub fn add_flaw(mut self, flaw: impl Into<String>) -> Self {
        self.flaws.push(flaw.into());

        self
    }
}
