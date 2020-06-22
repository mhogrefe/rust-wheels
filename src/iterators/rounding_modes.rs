use malachite_base::rounding_modes::RoundingMode;

use iterators::general::{random_from_vector, RandomFromVector};

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
