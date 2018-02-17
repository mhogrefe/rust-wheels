use iterators::general::{random_x, range_decreasing_x, range_increasing_x, Random,
                         RangeDecreasing, RangeIncreasing};
use itertools::{Interleave, Itertools};
use malachite_base::misc::{Min, Walkable};
use malachite_base::num::{PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned};
use rand::distributions::{IndependentSample, Range};
use rand::{IsaacRng, Rand, SeedableRng};
use std::iter::{once, Chain, Once};
use std::fmt::Display;

pub fn exhaustive_positive_x<T: PrimitiveInteger>() -> RangeIncreasing<T> {
    range_increasing_x(T::ONE, T::MAX)
}

pub fn exhaustive_u<T: PrimitiveUnsigned>() -> RangeIncreasing<T> {
    range_increasing_x(T::ZERO, T::MAX)
}

pub fn exhaustive_negative_i<T: PrimitiveSigned>() -> RangeDecreasing<T> {
    range_decreasing_x(T::MIN, T::NEGATIVE_ONE)
}

pub fn exhaustive_natural_i<T: PrimitiveSigned>() -> RangeIncreasing<T> {
    range_increasing_x(T::ZERO, T::MAX)
}

type UpDown<T> = Interleave<RangeIncreasing<T>, RangeDecreasing<T>>;

pub fn exhaustive_nonzero_i<T: PrimitiveSigned>() -> UpDown<T> {
    exhaustive_positive_x().interleave(exhaustive_negative_i())
}

pub fn exhaustive_i<T: PrimitiveSigned>() -> Chain<Once<T>, UpDown<T>> {
    once(T::ZERO).chain(exhaustive_nonzero_i())
}

pub fn range_up_increasing_x<T: PrimitiveInteger>(a: T) -> RangeIncreasing<T> {
    range_increasing_x(a, T::MAX)
}

pub fn range_up_decreasing_x<T: PrimitiveInteger>(a: T) -> RangeDecreasing<T> {
    range_decreasing_x(a, T::MAX)
}

pub fn range_down_increasing_x<T: Display + Min + Walkable>(b: T) -> RangeIncreasing<T> {
    range_increasing_x(T::MIN, b)
}

pub fn range_down_decreasing_x<T: Display + Min + Walkable>(b: T) -> RangeDecreasing<T> {
    range_decreasing_x(T::MIN, b)
}

pub fn x_increasing<T: PrimitiveInteger>() -> RangeIncreasing<T> {
    range_increasing_x(T::MIN, T::MAX)
}

pub fn x_decreasing<T: PrimitiveInteger>() -> RangeDecreasing<T> {
    range_decreasing_x(T::MIN, T::MAX)
}

#[derive(Clone)]
pub enum ExhaustiveRangeI<T: PrimitiveSigned> {
    AllNonNegative(RangeIncreasing<T>),
    AllNonPositive(RangeDecreasing<T>),
    SomeOfEachSign(Chain<Once<T>, UpDown<T>>),
}

impl<T: PrimitiveSigned> Iterator for ExhaustiveRangeI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match *self {
            ExhaustiveRangeI::AllNonNegative(ref mut xs) => xs.next(),
            ExhaustiveRangeI::AllNonPositive(ref mut xs) => xs.next(),
            ExhaustiveRangeI::SomeOfEachSign(ref mut xs) => xs.next(),
        }
    }
}

pub fn exhaustive_range_i<T: PrimitiveSigned>(a: T, b: T) -> ExhaustiveRangeI<T> {
    let zero = T::ZERO;
    if a >= zero {
        ExhaustiveRangeI::AllNonNegative(range_increasing_x(a, b))
    } else if b <= zero {
        ExhaustiveRangeI::AllNonPositive(range_decreasing_x(a, b))
    } else {
        ExhaustiveRangeI::SomeOfEachSign(once(zero).chain(
            range_increasing_x(T::ONE, b).interleave(range_decreasing_x(a, T::NEGATIVE_ONE)),
        ))
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

pub struct RandomPositiveU<T: Rand>(Random<T>);

impl<T: PrimitiveUnsigned> Iterator for RandomPositiveU<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let zero = T::ZERO;
        loop {
            let x = self.0.next();
            if x.is_none() || x.unwrap() != zero {
                return x;
            }
        }
    }
}

pub fn random_positive_u<T: Rand>(seed: &[u32]) -> RandomPositiveU<T> {
    RandomPositiveU(random_x(seed))
}

pub struct RandomPositiveI<T: PrimitiveSigned>(Random<T>);

impl<T: PrimitiveSigned> Iterator for RandomPositiveI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let zero = T::ZERO;
        loop {
            let x = self.0.next().map(|x| x & T::MAX);
            if x.is_none() || x.unwrap() != zero {
                return x;
            }
        }
    }
}

pub fn random_positive_i<T: PrimitiveSigned>(seed: &[u32]) -> RandomPositiveI<T> {
    RandomPositiveI(random_x(seed))
}

pub struct RandomNegativeI<T: PrimitiveSigned>(Random<T>);

impl<T: PrimitiveSigned> Iterator for RandomNegativeI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.next().map(|x| !(x & T::MAX))
    }
}

pub fn random_negative_i<T: PrimitiveSigned>(seed: &[u32]) -> RandomNegativeI<T> {
    RandomNegativeI(random_x(seed))
}

pub struct RandomNaturalI<T: PrimitiveSigned>(Random<T>);

impl<T: PrimitiveSigned> Iterator for RandomNaturalI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.next().map(|x| x & T::MAX)
    }
}

pub fn random_natural_i<T: PrimitiveSigned>(seed: &[u32]) -> RandomNaturalI<T> {
    RandomNaturalI(random_x(seed))
}

pub struct RandomNonzeroI<T: PrimitiveSigned>(Random<T>);

impl<T: PrimitiveSigned> Iterator for RandomNonzeroI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let zero = T::ZERO;
        loop {
            let x = self.0.next();
            if x.is_none() || x.unwrap() != zero {
                return x;
            }
        }
    }
}

pub fn random_nonzero_i<T: PrimitiveSigned>(seed: &[u32]) -> RandomNonzeroI<T> {
    RandomNonzeroI(random_x(seed))
}

pub fn random_range<T: PrimitiveInteger>(seed: &[u32], a: T, b: T) -> RandomRange<T> {
    if a == T::MIN && b == T::MAX {
        RandomRange::All(random_x(seed))
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
