extern crate itertools;

// Can uncomment this to use itertools "unique" adaptor which should behave the same
// I implemented the "unique" extension to "Iterator" trait as a learning exercise
// use itertools::Itertools;

use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug)]
struct Unique<I>
where
    I: Iterator,
    I::Item: Eq + Hash,
{
    iter: I,
    seen_mults: HashSet<I::Item>,
}

impl<I> Iterator for Unique<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(v) => {
                    if self.seen_mults.contains(&v) {
                        continue;
                    }
                    self.seen_mults.insert(v.clone());
                    return Some(v);
                }
            }
        }
    }
}

trait UniqueExt: Iterator {
    fn unique(self) -> Unique<Self>
    where
        Self::Item: Hash + Eq + Clone,
        Self: Sized,
    {
        Unique {
            iter: self,
            seen_mults: HashSet::new(),
        }
    }
}

impl<I: Iterator> UniqueExt for I {}

fn unique_multiples<'a>(limit: u32, factors: &'a [u32]) -> impl Iterator<Item = u32> + 'a {
    factors
        .iter()
        .flat_map(move |f| (*f..limit).step_by(*f as usize))
        .unique()
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    unique_multiples(limit, factors).sum()
}
