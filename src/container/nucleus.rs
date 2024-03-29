use crate::{board::Board, property::Antiness};
use crate::ordered::OrdIx2;
use crate::property::Direction;
use sorted_vec::SortedSet;

use super::{ContainerTrait, Contents};

#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd, Ord)]
pub struct NucleusParticle {
    antiness: Antiness,
    protons: SortedSet<OrdIx2>,
    neutrons: SortedSet<OrdIx2>,
}

impl ContainerTrait for NucleusParticle {
    fn contents(&self) -> Contents {
        let mut res: Vec<_> = self.protons.iter().map(|&i| i.into()).collect();
        res.extend(self.neutrons.iter().map(|&i| i.into()));
        res.into()
    }

    fn charge(&self) -> u32 {
        let protons: u32 = self.protons.len().try_into().expect("too many protons");
        protons * 3
    }
}

impl NucleusParticle {
    pub fn new(antiness: Antiness, protons: SortedSet<OrdIx2>, neutrons: SortedSet<OrdIx2>) -> Self {
        Self { antiness, protons, neutrons }
    }

    pub fn one_move(&self, direction: Direction) -> Board {
        todo!()
    }
}
