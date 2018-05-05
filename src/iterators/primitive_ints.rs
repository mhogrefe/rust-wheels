use iterators::general::{
    random, range_decreasing, range_increasing, Random, RangeDecreasing, RangeIncreasing,
};
use itertools::{Interleave, Itertools};
use malachite_base::misc::{Min, Walkable};
use malachite_base::num::{PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned};
use malachite_nz::natural::random::special_random_natural_up_to_bits::*;
use rand::distributions::{IndependentSample, Range};
use rand::{IsaacRng, Rand, SeedableRng};
use std::fmt::Display;
use std::iter::{once, Chain, Once};

pub fn exhaustive_positive<T: PrimitiveInteger>() -> RangeIncreasing<T> {
    range_increasing(T::ONE, T::MAX)
}

pub fn exhaustive_unsigned<T: PrimitiveUnsigned>() -> RangeIncreasing<T> {
    range_increasing(T::ZERO, T::MAX)
}

pub fn exhaustive_negative_signed<T: PrimitiveSigned>() -> RangeDecreasing<T> {
    range_decreasing(T::MIN, T::NEGATIVE_ONE)
}

pub fn exhaustive_natural_signed<T: PrimitiveSigned>() -> RangeIncreasing<T> {
    range_increasing(T::ZERO, T::MAX)
}

type UpDown<T> = Interleave<RangeIncreasing<T>, RangeDecreasing<T>>;

pub fn exhaustive_nonzero_signed<T: PrimitiveSigned>() -> UpDown<T> {
    exhaustive_positive().interleave(exhaustive_negative_signed())
}

pub fn exhaustive_signed<T: PrimitiveSigned>() -> Chain<Once<T>, UpDown<T>> {
    once(T::ZERO).chain(exhaustive_nonzero_signed())
}

pub fn range_up_increasing<T: PrimitiveInteger>(a: T) -> RangeIncreasing<T> {
    range_increasing(a, T::MAX)
}

pub fn range_up_decreasing<T: PrimitiveInteger>(a: T) -> RangeDecreasing<T> {
    range_decreasing(a, T::MAX)
}

pub fn range_down_increasing<T: Display + Min + Walkable>(b: T) -> RangeIncreasing<T> {
    range_increasing(T::MIN, b)
}

pub fn range_down_decreasing<T: Display + Min + Walkable>(b: T) -> RangeDecreasing<T> {
    range_decreasing(T::MIN, b)
}

pub fn increasing<T: PrimitiveInteger>() -> RangeIncreasing<T> {
    range_increasing(T::MIN, T::MAX)
}

pub fn decreasing<T: PrimitiveInteger>() -> RangeDecreasing<T> {
    range_decreasing(T::MIN, T::MAX)
}

#[derive(Clone)]
pub enum ExhaustiveRangeSigned<T: PrimitiveSigned> {
    AllNonNegative(RangeIncreasing<T>),
    AllNonPositive(RangeDecreasing<T>),
    SomeOfEachSign(Chain<Once<T>, UpDown<T>>),
}

impl<T: PrimitiveSigned> Iterator for ExhaustiveRangeSigned<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match *self {
            ExhaustiveRangeSigned::AllNonNegative(ref mut xs) => xs.next(),
            ExhaustiveRangeSigned::AllNonPositive(ref mut xs) => xs.next(),
            ExhaustiveRangeSigned::SomeOfEachSign(ref mut xs) => xs.next(),
        }
    }
}

pub fn exhaustive_range_signed<T: PrimitiveSigned>(a: T, b: T) -> ExhaustiveRangeSigned<T> {
    let zero = T::ZERO;
    if a >= zero {
        ExhaustiveRangeSigned::AllNonNegative(range_increasing(a, b))
    } else if b <= zero {
        ExhaustiveRangeSigned::AllNonPositive(range_decreasing(a, b))
    } else {
        ExhaustiveRangeSigned::SomeOfEachSign(
            once(zero).chain(
                range_increasing(T::ONE, b).interleave(range_decreasing(a, T::NEGATIVE_ONE)),
            ),
        )
    }
}

pub enum RandomRange<T: Rand> {
    Some(bool, Box<IsaacRng>, Range<T>),
    All(Random<T>),
}

impl<T: PrimitiveInteger> Iterator for RandomRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match *self {
            RandomRange::Some(shift, ref mut rng, ref range) => Some(if shift {
                range.ind_sample(rng) + T::ONE
            } else {
                range.ind_sample(rng)
            }),
            RandomRange::All(ref mut xs) => xs.next(),
        }
    }
}

pub struct RandomPositiveUnsigned<T: Rand>(Random<T>);

impl<T: PrimitiveUnsigned> Iterator for RandomPositiveUnsigned<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        loop {
            let x = self.0.next();
            if x != Some(T::ZERO) {
                return x;
            }
        }
    }
}

pub fn random_positive_unsigned<T: Rand>(seed: &[u32]) -> RandomPositiveUnsigned<T> {
    RandomPositiveUnsigned(random(seed))
}

pub struct RandomPositiveSigned<T: PrimitiveSigned>(Random<T>);

impl<T: PrimitiveSigned> Iterator for RandomPositiveSigned<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        loop {
            let x = self.0.next().map(|x| x & T::MAX);
            if x != Some(T::ZERO) {
                return x;
            }
        }
    }
}

pub fn random_positive_signed<T: PrimitiveSigned>(seed: &[u32]) -> RandomPositiveSigned<T> {
    RandomPositiveSigned(random(seed))
}

pub struct RandomNegativeSigned<T: PrimitiveSigned>(Random<T>);

impl<T: PrimitiveSigned> Iterator for RandomNegativeSigned<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.next().map(|x| !(x & T::MAX))
    }
}

pub fn random_negative_signed<T: PrimitiveSigned>(seed: &[u32]) -> RandomNegativeSigned<T> {
    RandomNegativeSigned(random(seed))
}

pub struct RandomNaturalSigned<T: PrimitiveSigned>(Random<T>);

impl<T: PrimitiveSigned> Iterator for RandomNaturalSigned<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.next().map(|x| x & T::MAX)
    }
}

pub fn random_natural_signed<T: PrimitiveSigned>(seed: &[u32]) -> RandomNaturalSigned<T> {
    RandomNaturalSigned(random(seed))
}

pub struct RandomNonzeroSigned<T: PrimitiveSigned>(Random<T>);

impl<T: PrimitiveSigned> Iterator for RandomNonzeroSigned<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        loop {
            let x = self.0.next();
            if x != Some(T::ZERO) {
                return x;
            }
        }
    }
}

pub fn random_nonzero_signed<T: PrimitiveSigned>(seed: &[u32]) -> RandomNonzeroSigned<T> {
    RandomNonzeroSigned(random(seed))
}

pub fn random_range<T: PrimitiveInteger>(seed: &[u32], a: T, b: T) -> RandomRange<T> {
    if a == T::MIN && b == T::MAX {
        RandomRange::All(random(seed))
    } else if b == T::MAX {
        RandomRange::Some(
            true,
            Box::new(IsaacRng::from_seed(seed)),
            Range::new(a - T::ONE, b),
        )
    } else {
        RandomRange::Some(
            false,
            Box::new(IsaacRng::from_seed(seed)),
            Range::new(a, b + T::ONE),
        )
    }
}

pub fn random_range_up<T: PrimitiveInteger>(seed: &[u32], a: T) -> RandomRange<T> {
    random_range(seed, a, T::MAX)
}

pub fn random_range_down<T: PrimitiveInteger>(seed: &[u32], a: T) -> RandomRange<T> {
    random_range(seed, T::MIN, a)
}

pub struct SpecialRandomUnsigned<T: PrimitiveUnsigned>(Random<T>);

impl<T: PrimitiveUnsigned> Iterator for SpecialRandomUnsigned<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let limbs = limbs_special_random_up_to_bits(&mut self.0.rng, u64::from(T::WIDTH));
        Some(T::from_u32_slice(&limbs))
    }
}

pub fn special_random_unsigned<T: PrimitiveUnsigned>(seed: &[u32]) -> SpecialRandomUnsigned<T> {
    SpecialRandomUnsigned(random(seed))
}

pub struct SpecialRandomPositiveUnsigned<T: PrimitiveUnsigned>(SpecialRandomUnsigned<T>);

impl<T: PrimitiveUnsigned> Iterator for SpecialRandomPositiveUnsigned<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        loop {
            let x = self.0.next();
            if x != Some(T::ZERO) {
                return x;
            }
        }
    }
}

pub fn special_random_positive_unsigned<T: PrimitiveUnsigned>(
    seed: &[u32],
) -> SpecialRandomPositiveUnsigned<T> {
    SpecialRandomPositiveUnsigned(special_random_unsigned(seed))
}

pub struct SpecialRandomNaturalSigned<T: PrimitiveSigned>(
    SpecialRandomUnsigned<T::UnsignedOfEqualWidth>,
);

impl<T: PrimitiveSigned> Iterator for SpecialRandomNaturalSigned<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.next().map(|x| T::from_unsigned_bitwise(x) & T::MAX)
    }
}

pub fn special_random_natural_signed<T: PrimitiveSigned>(
    seed: &[u32],
) -> SpecialRandomNaturalSigned<T> {
    SpecialRandomNaturalSigned(special_random_unsigned(seed))
}

pub struct SpecialRandomPositiveSigned<T: PrimitiveSigned>(
    SpecialRandomPositiveUnsigned<T::UnsignedOfEqualWidth>,
);

impl<T: PrimitiveSigned> Iterator for SpecialRandomPositiveSigned<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.next().map(|x| T::from_unsigned_bitwise(x) & T::MAX)
    }
}

pub fn special_random_positive_signed<T: PrimitiveSigned>(
    seed: &[u32],
) -> SpecialRandomPositiveSigned<T> {
    SpecialRandomPositiveSigned(special_random_positive_unsigned(seed))
}

pub struct SpecialRandomNegativeSigned<T: PrimitiveSigned>(
    SpecialRandomUnsigned<T::UnsignedOfEqualWidth>,
);

impl<T: PrimitiveSigned> Iterator for SpecialRandomNegativeSigned<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.next().map(|x| T::from_unsigned_bitwise(x) | T::MIN)
    }
}

pub fn special_random_negative_signed<T: PrimitiveSigned>(
    seed: &[u32],
) -> SpecialRandomNegativeSigned<T> {
    SpecialRandomNegativeSigned(special_random_unsigned(seed))
}

pub struct SpecialRandomSigned<T: PrimitiveSigned>(SpecialRandomUnsigned<T::UnsignedOfEqualWidth>);

impl<T: PrimitiveSigned> Iterator for SpecialRandomSigned<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.next().map(T::from_unsigned_bitwise)
    }
}

pub fn special_random_signed<T: PrimitiveSigned>(seed: &[u32]) -> SpecialRandomSigned<T> {
    SpecialRandomSigned(special_random_unsigned(seed))
}

pub struct SpecialRandomNonzeroSigned<T: PrimitiveSigned>(SpecialRandomSigned<T>);

impl<T: PrimitiveSigned> Iterator for SpecialRandomNonzeroSigned<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        loop {
            let x = self.0.next();
            if x != Some(T::ZERO) {
                return x;
            }
        }
    }
}

pub fn special_random_nonzero_signed<T: PrimitiveSigned>(
    seed: &[u32],
) -> SpecialRandomNonzeroSigned<T> {
    SpecialRandomNonzeroSigned(special_random_signed(seed))
}
