use crate::container::Container;
use crate::particle::Particle;

#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd, Ord)]
pub enum Object<'a> {
    Particle(Particle),
    Container(&'a Container, Box<Object<'a>>),
}
