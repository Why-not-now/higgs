use super::ObstacleTrait;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Default, PartialOrd, Ord)]
pub struct Block;

impl ObstacleTrait for Block {}
