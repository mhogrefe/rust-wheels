use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::ModPowerOfTwoNeg;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use rand::{IsaacRng, Rng, SeedableRng};

use iterators::common::scramble;
use iterators::general::{random, Random};
use iterators::integers_geometric::{range_up_geometric_u32, RangeUpGeometricU32};
use iterators::naturals::{
    random_natural_below_old, random_natural_with_bits_old, random_naturals,
    random_positive_naturals, special_random_naturals, special_random_positive_naturals,
    RandomNaturals, RandomPositiveNaturals, SpecialRandomNaturals, SpecialRandomPositiveNaturals,
};

pub struct RandomPositiveIntegers(RandomPositiveNaturals);

impl Iterator for RandomPositiveIntegers {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        self.0.next().map(Integer::from)
    }
}

pub fn random_positive_integers(seed: &[u32], scale: u32) -> RandomPositiveIntegers {
    RandomPositiveIntegers(random_positive_naturals(seed, scale))
}

pub struct RandomNaturalIntegers(RandomNaturals);

impl Iterator for RandomNaturalIntegers {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        self.0.next().map(Integer::from)
    }
}

pub fn random_natural_integers(seed: &[u32], scale: u32) -> RandomNaturalIntegers {
    RandomNaturalIntegers(random_naturals(seed, scale))
}

pub struct RandomNegativeIntegers(RandomPositiveIntegers);

impl Iterator for RandomNegativeIntegers {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        self.0.next().map(|i| -i)
    }
}

pub fn random_negative_integers(seed: &[u32], scale: u32) -> RandomNegativeIntegers {
    RandomNegativeIntegers(random_positive_integers(seed, scale))
}

pub struct RandomNonzeroIntegers {
    signs: Random<bool>,
    abs: RandomPositiveIntegers,
}

impl Iterator for RandomNonzeroIntegers {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        if self.signs.next().unwrap() {
            self.abs.next()
        } else {
            self.abs.next().map(|i| -i)
        }
    }
}

pub fn random_nonzero_integers(seed: &[u32], scale: u32) -> RandomNonzeroIntegers {
    RandomNonzeroIntegers {
        signs: random(&scramble(seed, "signs")),
        abs: random_positive_integers(&scramble(seed, "abs"), scale),
    }
}

pub struct RandomIntegers {
    signs: Random<bool>,
    abs: RandomNaturalIntegers,
}

impl Iterator for RandomIntegers {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        if self.signs.next().unwrap() {
            self.abs.next()
        } else {
            self.abs.next().map(|i| -i)
        }
    }
}

pub fn random_integers(seed: &[u32], scale: u32) -> RandomIntegers {
    RandomIntegers {
        signs: random(&scramble(seed, "signs")),
        abs: random_natural_integers(&scramble(seed, "abs"), scale),
    }
}

pub struct SpecialRandomPositiveIntegers(SpecialRandomPositiveNaturals);

impl Iterator for SpecialRandomPositiveIntegers {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        self.0.next().map(Integer::from)
    }
}

pub fn special_random_positive_integers(seed: &[u32], scale: u32) -> SpecialRandomPositiveIntegers {
    SpecialRandomPositiveIntegers(special_random_positive_naturals(seed, scale))
}

pub struct SpecialRandomNaturalIntegers(SpecialRandomNaturals);

impl Iterator for SpecialRandomNaturalIntegers {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        self.0.next().map(Integer::from)
    }
}

pub fn special_random_natural_integers(seed: &[u32], scale: u32) -> SpecialRandomNaturalIntegers {
    SpecialRandomNaturalIntegers(special_random_naturals(seed, scale))
}

pub struct SpecialRandomNegativeIntegers(SpecialRandomPositiveIntegers);

impl Iterator for SpecialRandomNegativeIntegers {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        self.0.next().map(|i| -i)
    }
}

pub fn special_random_negative_integers(seed: &[u32], scale: u32) -> SpecialRandomNegativeIntegers {
    SpecialRandomNegativeIntegers(special_random_positive_integers(seed, scale))
}

pub struct SpecialRandomNonzeroIntegers {
    signs: Random<bool>,
    abs: SpecialRandomPositiveIntegers,
}

impl Iterator for SpecialRandomNonzeroIntegers {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        if self.signs.next().unwrap() {
            self.abs.next()
        } else {
            self.abs.next().map(|i| -i)
        }
    }
}

pub fn special_random_nonzero_integers(seed: &[u32], scale: u32) -> SpecialRandomNonzeroIntegers {
    SpecialRandomNonzeroIntegers {
        signs: random(&scramble(seed, "signs")),
        abs: special_random_positive_integers(&scramble(seed, "abs"), scale),
    }
}

pub struct SpecialRandomIntegers {
    signs: Random<bool>,
    abs: SpecialRandomNaturalIntegers,
}

impl Iterator for SpecialRandomIntegers {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        if self.signs.next().unwrap() {
            self.abs.next()
        } else {
            self.abs.next().map(|i| -i)
        }
    }
}

pub fn special_random_integers(seed: &[u32], scale: u32) -> SpecialRandomIntegers {
    SpecialRandomIntegers {
        signs: random(&scramble(seed, "signs")),
        abs: special_random_natural_integers(&scramble(seed, "abs"), scale),
    }
}

pub struct RandomRangeInteger {
    rng: Box<IsaacRng>,
    diameter_plus_one: Natural,
    a: Integer,
}

impl Iterator for RandomRangeInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        Some(
            Integer::from(random_natural_below_old(
                &mut self.rng,
                &self.diameter_plus_one,
            )) + &self.a,
        )
    }
}

pub fn random_range_integer(seed: &[u32], a: Integer, b: Integer) -> RandomRangeInteger {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RandomRangeInteger {
        rng: Box::new(IsaacRng::from_seed(seed)),
        diameter_plus_one: Natural::exact_from(b - &a) + Natural::ONE,
        a,
    }
}

pub struct RandomRangeUpInteger {
    rng: Box<IsaacRng>,
    bit_sizes: RangeUpGeometricU32,
    a: Integer,
    a_bit_size: u64,
    offset_limit: Option<Natural>,
}

impl Iterator for RandomRangeUpInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let bit_size = u64::from(self.bit_sizes.next().unwrap());
        Some(if bit_size == 0 {
            Integer::ZERO
        } else {
            match bit_size.cmp(&self.a_bit_size) {
                Ordering::Less => {
                    // a < 0
                    // Generates values between 2^(n - 1) and 2^n - 1, inclusive, or
                    // between -(2^n - 1) and -2^(n - 1), inclusive.
                    let abs_result = random_natural_with_bits_old(&mut self.rng, bit_size);
                    if self.rng.gen() {
                        Integer::from(abs_result)
                    } else {
                        -abs_result
                    }
                }
                Ordering::Greater => {
                    // Generates values between 2^(n - 1) and 2^n - 1, inclusive.
                    Integer::from(random_natural_with_bits_old(&mut self.rng, bit_size))
                }
                Ordering::Equal => {
                    if let Some(ref offset_limit) = self.offset_limit {
                        // a >= 0
                        // Generates values between a and 2^n - 1, inclusive.
                        Integer::from(random_natural_below_old(&mut self.rng, offset_limit))
                            + &self.a
                    } else {
                        // a < 0
                        // Generates values between 2^(n - 1) and 2^n - 1, inclusive, or
                        // between a and -2^(n - 1), inclusive.
                        //
                        // Loop loops <= 2 times on average.
                        loop {
                            let abs_result = random_natural_with_bits_old(&mut self.rng, bit_size);
                            let result = if self.rng.gen() {
                                Integer::from(abs_result)
                            } else {
                                -abs_result
                            };
                            if result >= self.a {
                                return Some(result);
                            }
                        }
                    }
                }
            }
        })
    }
}

pub fn random_range_up_integer(seed: &[u32], scale: u32, a: Integer) -> RandomRangeUpInteger {
    let a_bit_size = a.significant_bits();
    let min_bit_size = if a < 0 {
        0
    } else {
        u32::exact_from(a_bit_size)
    };
    let offset_limit = if a < 0 {
        None
    } else {
        Some((&a).unsigned_abs_ref().mod_power_of_two_neg(a_bit_size))
    };
    RandomRangeUpInteger {
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bits"))),
        bit_sizes: range_up_geometric_u32(&scramble(seed, "bitsizes"), scale, min_bit_size),
        a,
        a_bit_size,
        offset_limit,
    }
}

pub struct RandomRangeDownInteger(RandomRangeUpInteger);

impl Iterator for RandomRangeDownInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        self.0.next().map(|i| -i)
    }
}

pub fn random_range_down_integer(seed: &[u32], scale: u32, a: Integer) -> RandomRangeDownInteger {
    RandomRangeDownInteger(random_range_up_integer(seed, scale, -a))
}
