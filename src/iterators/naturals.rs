use malachite_base::num::arithmetic::traits::{
    IsPowerOf2, ModPowerOf2, PowerOf2, SaturatingSubAssign, ShrRound,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_base::num::conversion::traits::VecFromOtherTypeSlice;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::arithmetic::add::limbs_slice_add_limb_in_place;
use malachite_nz::natural::Natural;
use rand::distributions::{IndependentSample, Range};
use rand::{IsaacRng, Rand, Rng, SeedableRng};
use std::cmp::max;

use iterators::common::scramble;
use iterators::integers_geometric::{
    positive_u32s_geometric, range_up_geometric_u32, u32s_geometric, PositiveU32sGeometric,
    RangeUpGeometricU32, U32sGeometric,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

pub(crate) fn random_natural_below_old<R: Rng>(rng: &mut R, n: &Natural) -> Natural {
    assert_ne!(*n, 0, "Cannot generate a Natural below 0");
    if n.is_power_of_2() {
        random_natural_up_to_bits_old(rng, n.significant_bits() - 1)
    } else {
        let bits = n.significant_bits();
        // Loop loops <= 2 times on average.
        loop {
            let m = random_natural_up_to_bits_old(rng, bits);
            if m < *n {
                return m;
            }
        }
    }
}

fn limbs_random_up_to_bits_old<T: PrimitiveUnsigned + Rand, R: Rng>(
    rng: &mut R,
    bits: u64,
) -> Vec<T> {
    assert_ne!(bits, 0);
    let remainder_bits = bits.mod_power_of_2(T::LOG_WIDTH);
    let limb_count = bits.shr_round(T::LOG_WIDTH, RoundingMode::Ceiling);
    let mut xs: Vec<T> = Vec::with_capacity(usize::exact_from(limb_count));
    for _ in 0..limb_count {
        xs.push(rng.gen());
    }
    if remainder_bits != 0 {
        xs.last_mut()
            .unwrap()
            .mod_power_of_2_assign(remainder_bits);
    }
    xs
}

#[cfg(feature = "32_bit_limbs")]
fn random_natural_up_to_bits_old<R: Rng>(rng: &mut R, bits: u64) -> Natural {
    if bits == 0 {
        Natural::ZERO
    } else {
        Natural::from_owned_limbs_asc(limbs_random_up_to_bits_old(rng, bits))
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
fn random_natural_up_to_bits_old<R: Rng>(rng: &mut R, bits: u64) -> Natural {
    if bits == 0 {
        Natural::ZERO
    } else {
        let xs: Vec<u32> = limbs_random_up_to_bits_old(rng, bits);
        Natural::from_owned_limbs_asc(u64::vec_from_other_type_slice(&xs))
    }
}

pub(crate) fn random_natural_with_bits_old<R: Rng>(rng: &mut R, bits: u64) -> Natural {
    let mut n = random_natural_up_to_bits_old(rng, bits);
    if bits != 0 {
        n.set_bit(bits - 1);
    }
    n
}

fn special_random_natural_below_old<R: Rng>(rng: &mut R, n: &Natural) -> Natural {
    assert_ne!(*n, 0, "Cannot generate a Natural below 0");
    if n.is_power_of_2() {
        special_random_natural_up_to_bits_old(rng, n.significant_bits() - 1)
    } else {
        let bits = n.significant_bits();
        // Loop loops <= 2 times on average.
        loop {
            let m = special_random_natural_up_to_bits_old(rng, bits);
            if m < *n {
                return m;
            }
        }
    }
}

pub(crate) fn limbs_special_random_up_to_bits_old<T: PrimitiveUnsigned, R: Rng>(
    rng: &mut R,
    bits: u64,
) -> Vec<T> {
    assert_ne!(bits, 0);
    let remainder_bits = bits.mod_power_of_2(T::LOG_WIDTH);
    let limb_count = bits.shr_round(T::LOG_WIDTH, RoundingMode::Ceiling);
    // Initialize the value to all binary 1s; later we'll remove chunks to create blocks of 0s.
    let mut limbs = vec![T::MAX; usize::exact_from(limb_count)];
    // max_chunk_size may be as low as max(1, bits / 4) or as high as bits. The actual chunk size
    // will be between 1 and max_chunk_size, inclusive.
    let max_chunk_size = max(1, bits / (rng.gen_range(0, 4) + 1));
    let chunk_size_range = Range::new(1, max_chunk_size + 1);
    // Start i at a random position in the highest limb.
    let mut i = (limb_count << T::LOG_WIDTH) - rng.gen_range(0, T::WIDTH) + 1;
    loop {
        let mut chunk_size = chunk_size_range.ind_sample(rng);
        i.saturating_sub_assign(chunk_size);
        if i == 0 {
            break;
        }
        let j = usize::exact_from(i >> T::LOG_WIDTH);
        if j < limbs.len() {
            limbs[j].clear_bit(i & T::WIDTH_MASK);
        }
        chunk_size = chunk_size_range.ind_sample(rng);
        i.saturating_sub_assign(chunk_size);
        limbs_slice_add_limb_in_place(
            &mut limbs[usize::exact_from(i >> T::LOG_WIDTH)..],
            T::power_of_2(i & T::WIDTH_MASK),
        );
        if i == 0 {
            break;
        }
    }
    if remainder_bits != 0 {
        limbs
            .last_mut()
            .unwrap()
            .mod_power_of_2_assign(remainder_bits);
    }
    limbs
}

#[cfg(feature = "32_bit_limbs")]
fn special_random_natural_up_to_bits_old<R: Rng>(rng: &mut R, bits: u64) -> Natural {
    if bits == 0 {
        Natural::ZERO
    } else {
        Natural::from_owned_limbs_asc(limbs_special_random_up_to_bits_old(rng, bits))
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
fn special_random_natural_up_to_bits_old<R: Rng>(rng: &mut R, bits: u64) -> Natural {
    if bits == 0 {
        Natural::ZERO
    } else {
        let xs: Vec<u32> = limbs_special_random_up_to_bits_old(rng, bits);
        Natural::from_owned_limbs_asc(u64::vec_from_other_type_slice(&xs))
    }
}

pub(crate) fn special_random_natural_with_bits_old<R: Rng>(rng: &mut R, bits: u64) -> Natural {
    let mut n = special_random_natural_up_to_bits_old(rng, bits);
    if bits != 0 {
        n.set_bit(bits - 1);
    }
    n
}

pub struct RandomPositiveNaturals {
    rng: Box<IsaacRng>,
    bit_sizes: PositiveU32sGeometric,
}

impl Iterator for RandomPositiveNaturals {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        Some(random_natural_with_bits_old(
            &mut self.rng,
            u64::from(self.bit_sizes.next().unwrap()),
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
        Some(random_natural_with_bits_old(
            &mut self.rng,
            u64::from(self.bit_sizes.next().unwrap()),
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
        Some(special_random_natural_with_bits_old(
            &mut self.rng,
            u64::from(self.bit_sizes.next().unwrap()),
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
        Some(special_random_natural_with_bits_old(
            &mut self.rng,
            u64::from(self.bit_sizes.next().unwrap()),
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
        Some(random_natural_below_old(&mut self.rng, &self.diameter_plus_one) + &self.a)
    }
}

pub fn random_range_natural(seed: &[u32], a: Natural, b: Natural) -> RandomRangeNatural {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RandomRangeNatural {
        rng: Box::new(IsaacRng::from_seed(seed)),
        diameter_plus_one: b - &a + Natural::ONE,
        a,
    }
}

pub struct SpecialRandomRangeNatural {
    rng: Box<IsaacRng>,
    diameter_plus_one: Natural,
    a: Natural,
}

impl Iterator for SpecialRandomRangeNatural {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        Some(special_random_natural_below_old(&mut self.rng, &self.diameter_plus_one) + &self.a)
    }
}

pub fn special_random_range_natural(
    seed: &[u32],
    a: Natural,
    b: Natural,
) -> SpecialRandomRangeNatural {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    SpecialRandomRangeNatural {
        rng: Box::new(IsaacRng::from_seed(seed)),
        diameter_plus_one: b - &a + Natural::ONE,
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
            random_natural_below_old(&mut self.rng, &self.offset_limit) + &self.a
        } else {
            // Generates values between 2^(n - 1) and 2^n - 1, inclusive.
            random_natural_with_bits_old(&mut self.rng, bit_size)
        })
    }
}

pub fn random_range_up_natural(seed: &[u32], scale: u32, a: Natural) -> RandomRangeUpNatural {
    let a_bit_size = a.significant_bits();
    // let n = a.significant_bits().
    // There are 2^(n - 1) values with exactly n bits, the smallest being 2^(n - 1);
    // a - 2^(n - 1) are smaller than a, so 2^(n - 1) - (a - 2^(n - 1)) = 2^n - a are at least a.
    let offset_limit = Natural::power_of_2(a_bit_size) - &a;
    RandomRangeUpNatural {
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bits"))),
        bit_sizes: range_up_geometric_u32(
            &scramble(seed, "bitsizes"),
            scale,
            u32::exact_from(a_bit_size),
        ),
        a,
        a_bit_size,
        offset_limit,
    }
}

pub struct SpecialRandomRangeUpNatural {
    rng: Box<IsaacRng>,
    bit_sizes: RangeUpGeometricU32,
    a: Natural,
    a_bit_size: u64,
    offset_limit: Natural,
}

impl Iterator for SpecialRandomRangeUpNatural {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        let bit_size = u64::from(self.bit_sizes.next().unwrap());
        Some(if bit_size == self.a_bit_size {
            // Generates values between a and 2^n - 1, inclusive.
            special_random_natural_below_old(&mut self.rng, &self.offset_limit) + &self.a
        } else {
            // Generates values between 2^(n - 1) and 2^n - 1, inclusive.
            special_random_natural_with_bits_old(&mut self.rng, bit_size)
        })
    }
}

pub fn special_random_range_up_natural(
    seed: &[u32],
    scale: u32,
    a: Natural,
) -> SpecialRandomRangeUpNatural {
    let a_bit_size = a.significant_bits();
    // let n = a.significant_bits().
    // There are 2^(n - 1) values with exactly n bits, the smallest being 2^(n - 1);
    // a - 2^(n - 1) are smaller than a, so 2^(n - 1) - (a - 2^(n - 1)) = 2^n - a are at least a.
    let offset_limit = Natural::power_of_2(a_bit_size) - &a;
    SpecialRandomRangeUpNatural {
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "bits"))),
        bit_sizes: range_up_geometric_u32(
            &scramble(seed, "bitsizes"),
            scale,
            u32::exact_from(a_bit_size),
        ),
        a,
        a_bit_size,
        offset_limit,
    }
}
