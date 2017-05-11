use iterators::general::{exhaustive_from_vector, ExhaustiveFromVector};

pub fn exhaustive_bools() -> ExhaustiveFromVector<bool> {
    exhaustive_from_vector(vec![false, true])
}
