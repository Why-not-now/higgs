use enum_dispatch::enum_dispatch;

use self::{block::Block, hole::Hole};

pub mod block;
pub mod hole;

#[enum_dispatch]
pub trait ObstacleTrait {}

#[enum_dispatch(ObstacleTrait)]
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Obstacle {
    Empty,
    Block,
    Hole,
}

impl Default for Obstacle {
    fn default() -> Self {
        Self::Empty(Empty)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Default)]
pub struct Empty;

impl ObstacleTrait for Empty {}
