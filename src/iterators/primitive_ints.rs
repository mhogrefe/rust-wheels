use iterators::general::{Random, random_x, RangeDecreasing, range_decreasing_x, RangeIncreasing,
                         range_increasing_x};
use itertools::{Interleave, Itertools};
use prim_utils::traits::{PrimInt, PrimSignedInt, PrimUnsignedInt};
use rand::distributions::{IndependentSample, Range};
use rand::{IsaacRng, Rand, SeedableRng};
use std::iter::{Chain, Once, once};

pub fn exhaustive_positive_x<T: PrimInt>() -> RangeIncreasing<T> {
    range_increasing_x(T::from_u8(1), T::max_value())
}

pub fn exhaustive_u<T: PrimUnsignedInt>() -> RangeIncreasing<T> {
    range_increasing_x(T::from_u8(0), T::max_value())
}

pub fn exhaustive_negative_i<T: PrimSignedInt>() -> RangeDecreasing<T> {
    range_decreasing_x(T::min_value(), T::from_i8(-1))
}

pub fn exhaustive_natural_i<T: PrimSignedInt>() -> RangeIncreasing<T> {
    range_increasing_x(T::from_u8(0), T::max_value())
}

pub fn exhaustive_nonzero_i<T: PrimSignedInt>
    ()
    -> Interleave<RangeIncreasing<T>, RangeDecreasing<T>>
{
    exhaustive_positive_x().interleave(exhaustive_negative_i())
}

pub fn exhaustive_i<T: PrimSignedInt>
    ()
    -> Chain<Once<T>, Interleave<RangeIncreasing<T>, RangeDecreasing<T>>>
{
    once(T::from_u8(0)).chain(exhaustive_nonzero_i())
}

pub fn range_up_increasing_x<T: PrimInt>(a: T) -> RangeIncreasing<T> {
    range_increasing_x(a, T::max_value())
}

pub fn range_up_decreasing_x<T: PrimInt>(a: T) -> RangeDecreasing<T> {
    range_decreasing_x(a, T::max_value())
}

pub fn range_down_increasing_x<T: PrimInt>(b: T) -> RangeIncreasing<T> {
    range_increasing_x(T::min_value(), b)
}

pub fn range_down_decreasing_x<T: PrimInt>(b: T) -> RangeDecreasing<T> {
    range_decreasing_x(T::min_value(), b)
}

pub fn x_increasing<T: PrimInt>() -> RangeIncreasing<T> {
    range_increasing_x(T::min_value(), T::max_value())
}

pub fn x_decreasing<T: PrimInt>() -> RangeDecreasing<T> {
    range_decreasing_x(T::min_value(), T::max_value())
}

pub enum ExhaustiveRangeI<T: PrimSignedInt> {
    AllNonNegative(RangeIncreasing<T>),
    AllNonPositive(RangeDecreasing<T>),
    SomeOfEachSign(Chain<Once<T>, Interleave<RangeIncreasing<T>, RangeDecreasing<T>>>),
}

impl<T: PrimSignedInt> Iterator for ExhaustiveRangeI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match *self {
            ExhaustiveRangeI::AllNonNegative(ref mut xs) => xs.next(),
            ExhaustiveRangeI::AllNonPositive(ref mut xs) => xs.next(),
            ExhaustiveRangeI::SomeOfEachSign(ref mut xs) => xs.next(),
        }
    }
}

pub fn exhaustive_range_i<T: PrimSignedInt>(a: T, b: T) -> ExhaustiveRangeI<T> {
    let zero = T::from_u8(0);
    if a >= zero {
        ExhaustiveRangeI::AllNonNegative(range_increasing_x(a, b))
    } else if b <= zero {
        ExhaustiveRangeI::AllNonPositive(range_decreasing_x(a, b))
    } else {
        ExhaustiveRangeI::SomeOfEachSign(
                once(zero).chain(
                        range_increasing_x(T::from_u8(1), b).interleave(
                                range_decreasing_x(a, T::from_i8(-1))
                        )
                )
        )
    }
}

pub enum RandomRange<T: Rand> {
    Some(bool, Box<IsaacRng>, Range<T>),
    All(Random<T>),
}

impl<T: PrimInt> Iterator for RandomRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match *self {
            RandomRange::Some(shift, ref mut rng, ref range) => {
                Some(if shift {
                         range.ind_sample(rng) + T::from_u8(1)
                     } else {
                         range.ind_sample(rng)
                     })
            }
            RandomRange::All(ref mut xs) => xs.next(),
        }
    }
}

pub struct RandomPositiveU<T: Rand>(Random<T>);

impl<T: PrimUnsignedInt> Iterator for RandomPositiveU<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let zero = T::from_u8(0);
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

pub struct RandomPositiveI<T: PrimSignedInt>(Random<T>);

impl<T: PrimSignedInt> Iterator for RandomPositiveI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let zero = T::from_u8(0);
        loop {
            let x = self.0.next().map(|x| x & T::max_value());
            if x.is_none() || x.unwrap() != zero {
                return x;
            }
        }
    }
}

pub fn random_positive_i<T: PrimSignedInt>(seed: &[u32]) -> RandomPositiveI<T> {
    RandomPositiveI(random_x(seed))
}

pub struct RandomNegativeI<T: PrimSignedInt>(Random<T>);

impl<T: PrimSignedInt> Iterator for RandomNegativeI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.next().map(|x| !(x & T::max_value()))
    }
}

pub fn random_negative_i<T: PrimSignedInt>(seed: &[u32]) -> RandomNegativeI<T> {
    RandomNegativeI(random_x(seed))
}

pub struct RandomNaturalI<T: PrimSignedInt>(Random<T>);

impl<T: PrimSignedInt> Iterator for RandomNaturalI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.next().map(|x| x & T::max_value())
    }
}

pub fn random_natural_i<T: PrimSignedInt>(seed: &[u32]) -> RandomNaturalI<T> {
    RandomNaturalI(random_x(seed))
}

pub struct RandomNonzeroI<T: PrimSignedInt>(Random<T>);

impl<T: PrimSignedInt> Iterator for RandomNonzeroI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let zero = T::from_u8(0);
        loop {
            let x = self.0.next();
            if x.is_none() || x.unwrap() != zero {
                return x;
            }
        }
    }
}

pub fn random_nonzero_i<T: PrimSignedInt>(seed: &[u32]) -> RandomNonzeroI<T> {
    RandomNonzeroI(random_x(seed))
}

pub fn random_range<T: PrimInt>(seed: &[u32], a: T, b: T) -> RandomRange<T> {
    if a == T::min_value() && b == T::max_value() {
        RandomRange::All(random_x(seed))
    } else if b == T::max_value() {
        RandomRange::Some(true,
                          Box::new(IsaacRng::from_seed(seed)),
                          Range::new(a - T::from_u8(1), b))
    } else {
        RandomRange::Some(false,
                          Box::new(IsaacRng::from_seed(seed)),
                          Range::new(a, b + T::from_u8(1)))
    }
}

pub fn random_range_up<T: PrimInt>(seed: &[u32], a: T) -> RandomRange<T> {
    random_range(seed, a, T::max_value())
}

pub fn random_range_down<T: PrimInt>(seed: &[u32], a: T) -> RandomRange<T> {
    random_range(seed, T::min_value(), a)
}
