use sorted_vec::SortedSet;

use crate::ordered::OrdIx2;
use crate::property::Direction;
use crate::{board::Board, property::Antiness};

use super::{Component, ContainerTrait, Contents};

#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd, Ord)]
pub struct NucleusParticle {
    antiness: Antiness,
    protons: SortedSet<OrdIx2>,
    neutrons: SortedSet<OrdIx2>,
    contents: SortedSet<Component>,
}

impl ContainerTrait for NucleusParticle {
    fn all_moves(&self, board: Board) -> Vec<Board> {
        todo!()
    }

    fn contents(&self) -> &Contents {
        &self.contents
    }

    fn charge(&self) -> i32 {
        let protons: i32 = self.protons.len().try_into().expect("too many protons");
        protons
            * match self.antiness {
                Antiness::Ordinary => 3,
                Antiness::Anti => -3,
            }
    }
}

impl NucleusParticle {
    pub fn new(
        antiness: Antiness,
        protons: SortedSet<OrdIx2>,
        neutrons: SortedSet<OrdIx2>,
    ) -> Self {
        let mut contents: Vec<_> = protons.iter().map(|&i| Component::Particle(i)).collect();
        contents.extend(neutrons.iter().map(|&i| Component::Particle(i)));
        let contents = Contents::from(contents);
        Self {
            antiness,
            protons,
            neutrons,
            contents,
        }
    }

    pub fn one_move(&self, direction: Direction) -> Board {
        todo!()
    }
}
