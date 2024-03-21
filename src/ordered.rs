use std::ops::{Deref, DerefMut};
use std::cmp::Ordering;

use ndarray::Ix2 as UnOrdIx2;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct OrdIx2(UnOrdIx2);

impl From<UnOrdIx2> for OrdIx2 {
    fn from(value: UnOrdIx2) -> Self {
        OrdIx2(value)
    }
}

impl Deref for OrdIx2 {
    type Target = UnOrdIx2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OrdIx2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Ord for OrdIx2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match (*self)[0].cmp(&(*other)[0]) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => (*self)[1].cmp(&(*other)[1]),
        }
    }
}

impl PartialOrd for OrdIx2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
