use iterators::general::{RandomFromVector, random_from_vector};
use malachite_base::round::RoundingMode;
use std;

pub fn exhaustive_rounding_modes() -> std::vec::IntoIter<RoundingMode> {
    vec![
        RoundingMode::Down,
        RoundingMode::Up,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
        RoundingMode::Nearest,
        RoundingMode::Exact,
    ].into_iter()
}

pub fn random_rounding_modes(seed: &[u32]) -> RandomFromVector<RoundingMode> {
    random_from_vector(
        seed,
        vec![
            RoundingMode::Down,
            RoundingMode::Up,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
            RoundingMode::Nearest,
            RoundingMode::Exact,
        ],
    )
}
