use std::collections::BTreeMap;

use sorted_vec::SortedVec;

use crate::ordered::OrdIx2;

use self::nucleus::NucleusParticle;

mod nucleus;

pub type Contents = SortedVec<Component>;
pub type ContainerLUT = BTreeMap<Component, (Container, Contents)>;

pub trait ContainerTrait {}

#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd, Ord)]
pub enum Container {
    NucleusParticle(NucleusParticle),
}

#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd, Ord)]
pub enum Component {
    Particle(OrdIx2),
    Container(Contents),
}
