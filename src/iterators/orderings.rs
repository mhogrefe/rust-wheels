use std;
use std::cmp::Ordering;

use iterators::general::{random_from_vector, RandomFromVector};

pub fn orderings_increasing() -> std::vec::IntoIter<Ordering> {
    vec![Ordering::Less, Ordering::Equal, Ordering::Greater].into_iter()
}

pub fn exhaustive_orderings() -> std::vec::IntoIter<Ordering> {
    vec![Ordering::Equal, Ordering::Less, Ordering::Greater].into_iter()
}

pub fn random_orderings(seed: &[u32]) -> RandomFromVector<Ordering> {
    random_from_vector(
        seed,
        vec![Ordering::Equal, Ordering::Less, Ordering::Greater],
    )
}
