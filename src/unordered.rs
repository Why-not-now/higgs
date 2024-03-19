use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

pub trait HasherU64<T> {
    fn hash(&self, value: &T) -> u64;
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DefaultHasherU64;

impl DefaultHasherU64 {
    fn new() -> Self {
        Self
    }
}

impl<T: Hash> HasherU64<T> for DefaultHasherU64 {
    fn hash(&self, value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Clone, Eq, Debug)]
pub struct UnorderedVec<T, S = DefaultHasherU64> {
    components: Vec<T>,
    hash: u64,
    hasher_u64: S,
}

impl<T: PartialEq, S: PartialEq> PartialEq for UnorderedVec<T, S> {
    fn eq(&self, other: &Self) -> bool {
        if self.hash == other.hash && self.hasher_u64 == other.hasher_u64 {
            unordered_eq(&self.components, &other.components)
        } else {
            false
        }
    }
}

impl<T, S> Hash for UnorderedVec<T, S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

impl<T, S> Deref for UnorderedVec<T, S> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.components
    }
}

impl<T: Hash> UnorderedVec<T, DefaultHasherU64> {
    pub fn new(components: Vec<T>) -> Self {
        let mut hash: u64 = 0;
        let hasher_u64 = DefaultHasherU64::new();
        for component in &components {
            let component_hash = hasher_u64.hash(component);
            hash ^= (component_hash ^ (component_hash << 16) ^ 89869747).wrapping_mul(3644798167);
        }

        Self {
            components,
            hash,
            hasher_u64,
        }
    }
}

impl<T, S: HasherU64<T>> UnorderedVec<T, S> {
    pub fn with_hasher(components: Vec<T>, hasher_u64: S) -> Self {
        let mut hash: u64 = 0;
        for component in &components {
            let component_hash = hasher_u64.hash(component);
            hash ^= (component_hash ^ (component_hash << 16) ^ 89869747).wrapping_mul(3644798167);
        }

        Self {
            components,
            hash,
            hasher_u64,
        }
    }
}

pub fn unordered_eq<T: PartialEq>(unordered_a: &[T], unordered_b: &[T]) -> bool {
    let len_a = unordered_a.len();
    if len_a != unordered_b.len() {
        return false;
    }

    let mut unmatched_indices_b: Vec<usize> = (0..len_a).collect();
    for item in unordered_a {
        'comparison: {
            for index_b in unmatched_indices_b.iter() {
                if item == &unordered_b[*index_b] {
                    unmatched_indices_b.swap_remove(*index_b);
                    break 'comparison;
                }
            }
            return false;
        }
    }

    true
}

impl<T: Hash> From<Vec<T>> for UnorderedVec<T> {
    fn from(value: Vec<T>) -> Self {
        Self::new(value)
    }
}
