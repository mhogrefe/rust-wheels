use iterators::common::scramble;
use iterators::integers_geometric::{NaturalU32sGeometric, PositiveU32sGeometric,
                                    natural_u32s_geometric, positive_u32s_geometric};
use malachite_base::num::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::natural::random::random_natural_with_bits::random_natural_with_bits;
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
        bitsizes: natural_u32s_geometric(&scramble(seed, "bitsizes"), scale),
    }
}
