#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Proficiency {
    Proficiency = 1,
    Expertise = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resistance {
    Vulnerable,
    Resistant,
    Immune,
}

impl Resistance {
    pub fn get_damage_multiplier(&self) -> f32 {
        match self {
            Resistance::Vulnerable => 2.0,
            Resistance::Resistant => 0.5,
            Resistance::Immune => 0.0,
        }
    }
}
