use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Duration {
    Instantaneous,
    Rounds(usize),
    Minutes(usize),
    Hours(usize),
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
            }
        )
    }
}
