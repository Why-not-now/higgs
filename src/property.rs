#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Default, PartialOrd, Ord)]
pub enum Antiness {
    #[default]
    Ordinary,
    Anti,
}

pub trait AntiTrait {
    fn anti(&self) -> Antiness;
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Default, PartialOrd, Ord)]
pub enum Colour {
    #[default]
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    AntiRed,
    AntiGreen,
    AntiBlue,
    AntiYellow,
    AntiCyan,
    AntiMagenta,
}

pub trait ColourTrait {
    fn colour(&self) -> Colour;
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, PartialOrd, Ord)]
pub enum Mass {
    Massless,
    Light,
    Medium,
    Heavy,
    Motionless,
    Unstable,
}

pub trait MassTrait {
    fn mass(&self) -> Mass;
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, PartialOrd, Ord)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Default, PartialOrd, Ord)]
pub enum Step {
    #[default]
    Continue,
    Shift,
    Annihilate(usize),
    Remove,
}
