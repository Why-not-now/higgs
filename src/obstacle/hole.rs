use super::ObstacleTrait;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Default)]
pub struct Hole;

impl ObstacleTrait for Hole {}
