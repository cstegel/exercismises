extern crate integer_sqrt;
extern crate itertools;

use integer_sqrt::IntegerSquareRoot;
use itertools::Itertools;

// use std::collections::HashSet;
use std::iter::once;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn factors(num: u64) -> impl Iterator<Item = u64> {
    let sqrt = num.integer_sqrt();

    // let mut factors: HashSet<_> = HashSet::new();
    // for x in 1..=sqrt {
    //     if num % x != 0 || x == num {
    //         continue;
    //     }
    //
    //     factors.insert(x);
    // }
    // factors.iter().cloned()
    (1..=sqrt)
        .filter(move |x| num % x == 0)
        .filter(move |x| *x != num)
        .flat_map(move |x| once(x).chain(once(num / x)))
        .unique()
}

/// Calculate the aliquot sum
/// This is the sum of all unique factors of a number excluding
/// the number itself
pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    match factors(num).sum::<u64>() {
        s if s < num => Some(Classification::Deficient),
        s if s > num => Some(Classification::Abundant),
        _ => Some(Classification::Perfect),
    }
}
