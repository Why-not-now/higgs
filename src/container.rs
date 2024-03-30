use std::collections::BTreeMap;

use sorted_vec::SortedSet;

use crate::board::Board;
use crate::ordered::OrdIx2;
use crate::property::Colour;

use self::nucleus::NucleusParticle;

mod nucleus;

pub type Contents = SortedSet<Component>;
pub type ContentsLUT = BTreeMap<Component, Contents>;
pub type ContainerLUT = BTreeMap<Contents, Container>;

pub trait ContainerTrait {
    fn all_moves(&self, board: Board) -> Vec<Board>;
    fn contents(&self) -> Contents;

    fn charge(&self) -> u32 {
        0
    }

    fn colour(&self) -> Colour {
        Colour::White
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd, Ord)]
pub enum Component {
    Particle(OrdIx2),
    Container(Contents),
}

impl From<OrdIx2> for Component {
    fn from(value: OrdIx2) -> Self {
        Self::Particle(value)
    }
}

impl From<Contents> for Component {
    fn from(value: Contents) -> Self {
        Self::Container(value)
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd, Ord)]
pub enum Container {
    NucleusParticle(NucleusParticle),
}
