use super::ObstacleTrait;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Default, PartialOrd, Ord)]
pub struct Hole;

impl ObstacleTrait for Hole {}
