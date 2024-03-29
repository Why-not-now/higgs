use std::collections::BTreeMap;

use sorted_vec::SortedVec;

use crate::ordered::OrdIx2;

use self::nucleus::NucleusParticle;

mod nucleus;

pub type Contents = SortedVec<Component>;
pub type ContentsLUT = BTreeMap<Component, Contents>;
pub type ContainerLUT = BTreeMap<Contents, Container>;

pub trait ContainerTrait {
    fn contents(&self) -> Contents;
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
