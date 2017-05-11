use iterators::general::{ExhaustiveFromVector, exhaustive_from_vector, RandomFromVector,
                         random_from_vector};
use std::cmp::Ordering;

pub fn orderings_increasing() -> ExhaustiveFromVector<Ordering> {
    exhaustive_from_vector(vec![Ordering::Less, Ordering::Equal, Ordering::Greater])
}

pub fn exhaustive_orderings() -> ExhaustiveFromVector<Ordering> {
    exhaustive_from_vector(vec![Ordering::Equal, Ordering::Less, Ordering::Greater])
}

pub fn random_orderings(seed: &[u32]) -> RandomFromVector<Ordering> {
    random_from_vector(seed,
                       vec![Ordering::Equal, Ordering::Less, Ordering::Greater])
}
