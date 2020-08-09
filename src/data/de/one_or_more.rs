use std::{iter, vec};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOrMore<T> {
    One(T),
    More(Vec<T>),
}

impl<T> Default for OneOrMore<T> {
    fn default() -> Self {
        OneOrMore::More(Vec::new())
    }
}

pub enum OneOrMoreIter<T> {
    Once(iter::Once<T>),
    Vec(vec::IntoIter<T>),
}

impl<T> Iterator for OneOrMoreIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            OneOrMoreIter::Once(itr) => itr.next(),
            OneOrMoreIter::Vec(itr) => itr.next(),
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            OneOrMoreIter::Once(itr) => itr.size_hint(),
            OneOrMoreIter::Vec(itr) => itr.size_hint(),
        }
    }
}

impl<T> IntoIterator for OneOrMore<T> {
    type Item = T;
    type IntoIter = OneOrMoreIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            OneOrMore::One(entry) => OneOrMoreIter::Once(iter::once(entry)),
            OneOrMore::More(entries) => OneOrMoreIter::Vec(entries.into_iter()),
        }
    }
}
