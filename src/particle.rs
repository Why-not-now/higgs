use enum_dispatch::enum_dispatch;
use ndarray::Ix2;

use crate::board::Board;

pub use self::{electron::Electron, muon::Muon, neutron::Neutron, tau::Tau};

mod electron;
mod muon;
mod neutron;
mod tau;

#[enum_dispatch]
pub trait ParticleTrait {
    fn charge(&self) -> i32 {
        0
    }
    fn all_moves(&self, board: &Board, pos: Ix2) -> Vec<Board>;
}

#[enum_dispatch(ParticleTrait)]
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Particle {
    Empty,
    Electron,
    Muon,
    Tau,
    Neutron,
}

impl Default for Particle {
    fn default() -> Self {
        Self::Empty(Empty)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Default)]
pub struct Empty;

impl ParticleTrait for Empty {
    fn all_moves(&self, _board: &Board, _pos: Ix2) -> Vec<Board> {
        unimplemented!()
    }
}
