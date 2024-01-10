#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Duration {
    Instantaneous,
    Rounds(usize),
    Minutes(usize),
    Hours(usize),
}
