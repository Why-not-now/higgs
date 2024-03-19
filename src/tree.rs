use slotmap::{Key, SlotMap};
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;

pub type TreeMap<K, T> = SlotMap<K, Node<K, T>>;

#[derive(Debug, Clone)]
pub struct KeyError<'a, K: Key, V>(&'a SlotMap<K, V>, K);

impl<'a, K: Key, V: Debug> Error for KeyError<'a, K, V> {}

impl<'a, K: Key, V> Display for KeyError<'a, K, V> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "key does not exists in tree")
    }
}

pub type KeyResult<'a, T, K, V> = Result<V, KeyError<'a, K, Node<K, T>>>;

#[derive(Debug)]
pub struct Tree<K: Key, T> {
    nodes: TreeMap<K, T>,
    root: K,
}

impl<K: Key, T: Clone> Clone for Tree<K, T> {
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone(),
            root: self.root,
        }
    }
}

impl<K: Key, T> Tree<K, T> {
    pub fn new(value: T) -> Self {
        let mut nodes: SlotMap<K, Node<K, T>> = SlotMap::with_key();
        let root = nodes.insert(Node {
            value,
            parents: vec![],
        });
        Self { nodes, root }
    }

    pub fn root(&self) -> K {
        self.root
    }

    pub fn get_value(&self, key: K) -> KeyResult<T, K, &T> {
        Ok(&self.get(key).ok_or(KeyError(&self.nodes, key))?.value)
    }

    pub fn parents_of(&self, key: K) -> KeyResult<T, K, &Vec<K>> {
        Ok(&self
            .nodes
            .get(key)
            .ok_or(KeyError(&self.nodes, key))?
            .parents)
    }

    pub fn insert_child_of(&mut self, parent: K, value: T) -> KeyResult<T, K, K> {
        if !self.nodes.contains_key(parent) {
            return Err(KeyError(&self.nodes, parent));
        }
        Ok(self.nodes.insert(Node {
            value,
            parents: vec![parent],
        }))
    }

    pub fn connect(&mut self, parent: K, child: K) -> KeyResult<T, K, ()> {
        if !self.nodes.contains_key(parent) {
            return Err(KeyError(&self.nodes, parent));
        }
        if !self.nodes.contains_key(child) {
            return Err(KeyError(&self.nodes, child));
        }
        self.nodes.get_mut(child).unwrap().parents.push(parent);
        Ok(())
    }
}

impl<K, T> Default for Tree<K, T>
where
    K: Key,
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<K, T> Deref for Tree<K, T>
where
    K: Key,
{
    type Target = TreeMap<K, T>;

    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

#[derive(Debug, Clone)]
pub struct Node<K: Key, T> {
    value: T,
    parents: Vec<K>,
}
