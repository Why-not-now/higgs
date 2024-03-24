use sorted_vec::SortedVec;

use crate::ordered::OrdIx2;

#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd, Ord)]
pub struct NucleusParticle {
    protons: SortedVec<OrdIx2>,
    neutrons: SortedVec<OrdIx2>,
}

impl NucleusParticle {
    pub fn new(protons: SortedVec<OrdIx2>, neutrons: SortedVec<OrdIx2>) -> Self {
        Self { protons, neutrons }
    }
}
