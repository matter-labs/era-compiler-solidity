//!
//! Implementation of [`IEnvironment`] as a stack of stacks. Stacks are backed by vectors.
//!

use std::fmt::Debug;

use super::IEnvironment;

/// An entry in a lookup data structure.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Entry<K, V>
where
    K: Clone + Debug + PartialEq + Eq,
    V: Clone + Debug + PartialEq + Eq,
{
    pub name: K,
    pub value: V,
}

type Table<K, V> = Vec<Entry<K, V>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Environment<K, V>
where
    K: Clone + std::fmt::Debug + PartialEq + Eq,
    V: Clone + std::fmt::Debug + PartialEq + Eq,
{
    tables: Vec<Table<K, V>>,
}

impl<K, V> IEnvironment<K, V> for Environment<K, V>
where
    K: Clone + std::fmt::Debug + PartialEq + Eq,
    V: Clone + std::fmt::Debug + PartialEq + Eq,
{
    fn add(&mut self, name: &K, value: &V) {
        self.tables.last_mut().unwrap().push(Entry {
            name: name.clone(),
            value: value.clone(),
        })
    }

    fn enter(&mut self) {
        self.tables.push(vec![])
    }

    fn leave(&mut self) {
        let _ = self.tables.pop();
    }

    fn get(&self, name: &K) -> Option<V> {
        for frame in self.tables.iter().rev() {
            if let Some(result) = frame.iter().find(|entry| &entry.name == name) {
                return Some(result.value.clone());
            }
        }
        None
    }
}

impl<K, V> Environment<K, V>
where
    K: Clone + std::fmt::Debug + PartialEq + Eq,
    V: Clone + std::fmt::Debug + PartialEq + Eq,
{
    pub fn new() -> Self {
        Self {
            tables: vec![vec![]],
        }
    }
}
