use iterators::common::scramble;
use iterators::integers_geometric::{NaturalU32sGeometric, PositiveU32sGeometric,
                                    positive_u32s_geometric, u32s_geometric};
use malachite_base::num::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::natural::random::random_natural_with_bits::random_natural_with_bits;
use malachite_nz::natural::random::special_random_natural_with_bits::*;
use rand::{IsaacRng, SeedableRng};

#[derive(Clone)]
pub struct RangeIncreasingUnboundedNatural(Natural);

impl Iterator for RangeIncreasingUnboundedNatural {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        let ret = self.0.clone();
        self.0 += 1;
        Some(ret)
    }
}

pub fn range_up_increasing_natural(a: Natural) -> RangeIncreasingUnboundedNatural {
    RangeIncreasingUnboundedNatural(a)
}

pub fn exhaustive_positive_naturals() -> RangeIncreasingUnboundedNatural {
    range_up_increasing_natural(Natural::ONE)
}

pub fn exhaustive_naturals() -> RangeIncreasingUnboundedNatural {
    range_up_increasing_natural(Natural::ZERO)
}

pub struct RandomPositiveNaturals {
    rng: IsaacRng,
    bitsizes: PositiveU32sGeometric,
}

impl Iterator for RandomPositiveNaturals {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        Some(random_natural_with_bits(
            &mut self.rng,
            self.bitsizes.next().unwrap().into(),
        ))
    }
}

pub fn random_positive_naturals(seed: &[u32], scale: u32) -> RandomPositiveNaturals {
    RandomPositiveNaturals {
        rng: IsaacRng::from_seed(&scramble(seed, "bits")),
        bitsizes: positive_u32s_geometric(&scramble(seed, "bitsizes"), scale),
    }
}

pub struct RandomNaturals {
    rng: IsaacRng,
    bitsizes: NaturalU32sGeometric,
}

impl Iterator for RandomNaturals {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        Some(random_natural_with_bits(
            &mut self.rng,
            self.bitsizes.next().unwrap().into(),
        ))
    }
}

pub fn random_naturals(seed: &[u32], scale: u32) -> RandomNaturals {
    RandomNaturals {
        rng: IsaacRng::from_seed(&scramble(seed, "bits")),
        bitsizes: u32s_geometric(&scramble(seed, "bitsizes"), scale),
    }
}

pub struct SpecialRandomPositiveNaturals {
    rng: IsaacRng,
    bitsizes: PositiveU32sGeometric,
}

impl Iterator for SpecialRandomPositiveNaturals {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        Some(special_random_natural_with_bits(
            &mut self.rng,
            self.bitsizes.next().unwrap().into(),
        ))
    }
}

pub fn special_random_positive_naturals(seed: &[u32], scale: u32) -> SpecialRandomPositiveNaturals {
    SpecialRandomPositiveNaturals {
        rng: IsaacRng::from_seed(&scramble(seed, "bits")),
        bitsizes: positive_u32s_geometric(&scramble(seed, "bitsizes"), scale),
    }
}

pub struct SpecialRandomNaturals {
    rng: IsaacRng,
    bitsizes: NaturalU32sGeometric,
}

impl Iterator for SpecialRandomNaturals {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        Some(special_random_natural_with_bits(
            &mut self.rng,
            self.bitsizes.next().unwrap().into(),
        ))
    }
}

pub fn special_random_naturals(seed: &[u32], scale: u32) -> SpecialRandomNaturals {
    SpecialRandomNaturals {
        rng: IsaacRng::from_seed(&scramble(seed, "bits")),
        bitsizes: u32s_geometric(&scramble(seed, "bitsizes"), scale),
    }
}
