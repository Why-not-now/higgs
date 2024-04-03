use crate::ordered::OrdIx2;
use crate::property::Direction;
use crate::{board::Board, property::Antiness};
use sorted_vec::SortedSet;

use super::{ContainerTrait, Contents};

#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd, Ord)]
pub struct NucleusParticle {
    antiness: Antiness,
    protons: SortedSet<OrdIx2>,
    neutrons: SortedSet<OrdIx2>,
    contents: SortedSet<OrdIx2>,
}

impl ContainerTrait for NucleusParticle {
    fn all_moves(&self, board: Board) -> Vec<Board> {
        todo!()
    }

    fn contents(&self) -> Contents {
        self.contents
            .iter()
            .map(|&i| i.into())
            .collect::<Vec<_>>()
            .into()
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
        let mut contents: Vec<_> = protons.to_vec();
        contents.extend(neutrons.iter());
        let contents = contents.into();
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
