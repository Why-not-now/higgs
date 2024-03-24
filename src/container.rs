use crate::object::Object;

use self::nucleus::NucleusParticle;

mod nucleus;

pub trait ContainerTrait {
    fn components(&self) -> Vec<Object>;
}

#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd, Ord)]
pub enum Container {
    NucleusParticle(NucleusParticle)
}
