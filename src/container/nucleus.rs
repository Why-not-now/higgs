use sorted_vec::SortedVec;
use crate::board::Board;
use crate::ordered::OrdIx2;
use crate::property::Direction;

use super::{ContainerTrait, Contents};

#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd, Ord)]
pub struct NucleusParticle {
    protons: SortedVec<OrdIx2>,
    neutrons: SortedVec<OrdIx2>,
}

impl ContainerTrait for NucleusParticle {
    fn contents(&self) -> Contents {
        let mut res : Vec<_>= self.protons.iter().map(|&i| i.into()).collect();
        res.extend(self.neutrons.iter().map(|&i| i.into()));
        res.into()
    }
}

impl NucleusParticle {
    pub fn new(protons: SortedVec<OrdIx2>, neutrons: SortedVec<OrdIx2>) -> Self {
        Self { protons, neutrons }
    }

    pub fn one_move(&self, direction: Direction) -> Board {
        
        todo!()
    }
}
