#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

pub trait Race {
    fn get_size(&self) -> Size;

    fn get_walking_speed(&self) -> usize;
}

pub struct Human;

impl Race for Human {
    fn get_size(&self) -> Size {
        Size::Medium
    }

    fn get_walking_speed(&self) -> usize {
        30
    }
}
