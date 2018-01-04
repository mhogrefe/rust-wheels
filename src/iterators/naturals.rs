use iterators::common::scramble;
use iterators::integers_geometric::{NaturalU32sGeometric, PositiveU32sGeometric,
                                    natural_u32s_geometric, positive_u32s_geometric};
use malachite_base::traits::{One, Zero};
use malachite::natural::Natural;
use malachite::natural::random::random_natural_from_bits::random_natural_from_bits;
use rand::{IsaacRng, SeedableRng};

#[derive(Clone)]
pub struct RangeIncreasingNatural {
    i: Natural,
    b: Natural,
    done: bool,
}

impl Iterator for RangeIncreasingNatural {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        if self.done {
            None
        } else {
            self.done = self.i == self.b;
            let ret = self.i.clone();
            if !self.done {
                self.i += 1
            }
            Some(ret)
        }
    }
}

#[derive(Clone)]
pub struct RangeDecreasingNatural {
    a: Natural,
    i: Natural,
    done: bool,
}

impl Iterator for RangeDecreasingNatural {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        if self.done {
            None
        } else {
            self.done = self.i == self.a;
            let ret = self.i.clone();
            if !self.done {
                self.i -= 1
            }
            Some(ret)
        }
    }
}

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

pub fn range_increasing_natural(a: Natural, b: Natural) -> RangeIncreasingNatural {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RangeIncreasingNatural {
        i: a,
        b: b,
        done: false,
    }
}

pub fn range_decreasing_natural(a: Natural, b: Natural) -> RangeDecreasingNatural {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RangeDecreasingNatural {
        a: a,
        i: b,
        done: false,
    }
}

pub fn range_up_increasing_natural(a: Natural) -> RangeIncreasingUnboundedNatural {
    RangeIncreasingUnboundedNatural(a)
}

pub fn range_down_increasing_natural(a: Natural) -> RangeIncreasingNatural {
    range_increasing_natural(Natural::ZERO, a)
}

pub fn range_down_decreasing_natural(a: Natural) -> RangeDecreasingNatural {
    range_decreasing_natural(Natural::ZERO, a)
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
        Some(random_natural_from_bits(
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
        Some(random_natural_from_bits(
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
