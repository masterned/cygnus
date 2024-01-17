use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Duration {
    Instantaneous,
    Rounds(usize),
    Minutes(usize),
    Hours(usize),
    Years(usize),
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Duration::Instantaneous => String::from("inst"),
                Duration::Rounds(r) => format!("{r} rnds."),
                Duration::Minutes(m) => format!("{m} mins."),
                Duration::Hours(h) => format!("{h} hrs."),
                Duration::Years(y) => format!("{y} yrs."),
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Distance {
    Inches(usize),
    Feet(usize),
}

impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Distance::Inches(i) => format!("{i}\""),
                Distance::Feet(f) => format!("{f}'"),
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Weight {
    Pounds(usize),
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Weight::Pounds(p) => format!("{p} lbs."),
            }
        )
    }
}
