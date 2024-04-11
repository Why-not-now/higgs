use std::collections::BTreeMap;

use enum_dispatch::enum_dispatch;
use sorted_vec::SortedSet;

use crate::board::Board;
use crate::ordered::OrdIx2;
use crate::property::Colour;

use self::nucleus::NucleusParticle;

mod nucleus;

pub type Contents = SortedSet<Component>;
pub type ContainerLUT = BTreeMap<Component, Container>;

pub fn contents_positions(contents: &Contents) -> Vec<OrdIx2> {
    let mut positions = Vec::new();

    for component in contents.iter() {
        component.all_positions_push(&mut positions);
    }

    positions
}

#[enum_dispatch]
pub trait ContainerTrait {
    fn all_moves(&self, board: Board) -> Vec<Board>;
    fn contents(&self) -> &Contents;

    fn charge(&self) -> i32 {
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

impl Component {
    pub fn all_positions(&self) -> Vec<OrdIx2> {
        let mut positions = Vec::new();

        self.all_positions_push(&mut positions);

        positions
    }

    fn all_positions_push<'a>(&'a self, positions: &'a mut Vec<OrdIx2>) -> &Vec<OrdIx2> {
        match self {
            Component::Particle(i) => positions.push(*i),
            Component::Container(c) => {
                for component in c.iter() {
                    component.all_positions_push(positions);
                }
            }
        }

        positions
    }
}

#[enum_dispatch(ContainerTrait)]
#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd, Ord)]
pub enum Container {
    NucleusParticle(NucleusParticle),
}
