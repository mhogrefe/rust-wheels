use std::cmp::Ordering;

use iterators::general::{random_from_vector, RandomFromVector};

pub fn random_orderings(seed: &[u32]) -> RandomFromVector<Ordering> {
    random_from_vector(
        seed,
        vec![Ordering::Equal, Ordering::Less, Ordering::Greater],
    )
}
