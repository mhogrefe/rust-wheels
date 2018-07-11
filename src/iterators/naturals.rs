use iterators::common::scramble;
use iterators::integers_geometric::{
    positive_u32s_geometric, range_up_geometric_u32, u32s_geometric, PositiveU32sGeometric,
    RangeUpGeometricU32, U32sGeometric,
};
use malachite_base::num::{One, SignificantBits, Zero};
use malachite_nz::natural::random::random_natural_below::random_natural_below;
use malachite_nz::natural::random::random_natural_with_bits::random_natural_with_bits;
use malachite_nz::natural::random::special_random_natural_with_bits::*;
use malachite_nz::natural::Natural;
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
    rng: Box<IsaacRng>,
    bit_sizes: PositiveU32sGeometric,
}

impl Iterator for RandomPositiveNaturals {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        Some(random_natural_with_bits(
            &mut self.rng,
            self.bit_sizes.next().unwrap().into(),
        ))
    }
}

pub fn random_positive_naturals(seed: &[u32], scale: u32) -> RandomPositiveNaturals {
    RandomPositiveNaturals {
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bits"))),
        bit_sizes: positive_u32s_geometric(&scramble(seed, "bitsizes"), scale),
    }
}

pub struct RandomNaturals {
    rng: Box<IsaacRng>,
    bit_sizes: U32sGeometric,
}

impl Iterator for RandomNaturals {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        Some(random_natural_with_bits(
            &mut self.rng,
            self.bit_sizes.next().unwrap().into(),
        ))
    }
}

pub fn random_naturals(seed: &[u32], scale: u32) -> RandomNaturals {
    RandomNaturals {
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bits"))),
        bit_sizes: u32s_geometric(&scramble(seed, "bitsizes"), scale),
    }
}

pub struct SpecialRandomPositiveNaturals {
    rng: Box<IsaacRng>,
    bit_sizes: PositiveU32sGeometric,
}

impl Iterator for SpecialRandomPositiveNaturals {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        Some(special_random_natural_with_bits(
            &mut self.rng,
            self.bit_sizes.next().unwrap().into(),
        ))
    }
}

pub fn special_random_positive_naturals(seed: &[u32], scale: u32) -> SpecialRandomPositiveNaturals {
    SpecialRandomPositiveNaturals {
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bits"))),
        bit_sizes: positive_u32s_geometric(&scramble(seed, "bitsizes"), scale),
    }
}

pub struct SpecialRandomNaturals {
    rng: Box<IsaacRng>,
    bit_sizes: U32sGeometric,
}

impl Iterator for SpecialRandomNaturals {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        Some(special_random_natural_with_bits(
            &mut self.rng,
            self.bit_sizes.next().unwrap().into(),
        ))
    }
}

pub fn special_random_naturals(seed: &[u32], scale: u32) -> SpecialRandomNaturals {
    SpecialRandomNaturals {
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bits"))),
        bit_sizes: u32s_geometric(&scramble(seed, "bitsizes"), scale),
    }
}

pub struct RandomRangeNatural {
    rng: Box<IsaacRng>,
    diameter_plus_one: Natural,
    a: Natural,
}

impl Iterator for RandomRangeNatural {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        Some(random_natural_below(&mut self.rng, &self.diameter_plus_one) + &self.a)
    }
}

pub fn random_range_natural(seed: &[u32], a: Natural, b: Natural) -> RandomRangeNatural {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RandomRangeNatural {
        rng: Box::new(IsaacRng::from_seed(seed)),
        diameter_plus_one: b - &a + 1,
        a,
    }
}

pub struct RandomRangeUpNatural {
    rng: Box<IsaacRng>,
    bit_sizes: RangeUpGeometricU32,
    a: Natural,
    a_bit_size: u64,
    offset_limit: Natural,
}

impl Iterator for RandomRangeUpNatural {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        let bit_size = u64::from(self.bit_sizes.next().unwrap());
        Some(if bit_size == self.a_bit_size {
            // Generates values between a and 2^n - 1, inclusive.
            random_natural_below(&mut self.rng, &self.offset_limit) + &self.a
        } else {
            // Generates values between 2^(n - 1) and 2^n - 1, inclusive.
            random_natural_with_bits(&mut self.rng, bit_size)
        })
    }
}

pub fn random_range_up_natural(seed: &[u32], scale: u32, a: Natural) -> RandomRangeUpNatural {
    let a_bit_size = a.significant_bits();
    // let n = a.significant_bits().
    // There are 2^(n - 1) values with exactly n bits, the smallest being 2^(n - 1);
    // a - 2^(n - 1) are smaller than a, so 2^(n - 1) - (a - 2^(n - 1)) = 2^n - a are at least a.
    let offset_limit = (Natural::ONE << (a_bit_size as u32)) - &a;
    RandomRangeUpNatural {
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bits"))),
        bit_sizes: range_up_geometric_u32(&scramble(seed, "bitsizes"), scale, a_bit_size as u32),
        a,
        a_bit_size,
        offset_limit,
    }
}
