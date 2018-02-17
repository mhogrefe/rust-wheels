use iterators::common::scramble;
use iterators::general::{random_x, range_decreasing_x, range_increasing_x, Random,
                         RangeDecreasing, RangeIncreasing};
use iterators::naturals::{random_naturals, random_positive_naturals, RandomNaturals,
                          RandomPositiveNaturals};
use itertools::{Interleave, Itertools};
use malachite_base::num::{NegativeOne, One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::random::random_natural_below;
use rand::{IsaacRng, SeedableRng};
use std::iter::{once, Chain, Once};

#[derive(Clone)]
pub struct RangeIncreasingUnboundedInteger(Integer);

impl Iterator for RangeIncreasingUnboundedInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let ret = self.0.clone();
        self.0 += 1;
        Some(ret)
    }
}

#[derive(Clone)]
pub struct RangeDecreasingUnboundedInteger(Integer);

impl Iterator for RangeDecreasingUnboundedInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let ret = self.0.clone();
        self.0 -= 1;
        Some(ret)
    }
}

pub fn range_up_increasing_integer(a: Integer) -> RangeIncreasingUnboundedInteger {
    RangeIncreasingUnboundedInteger(a)
}

pub fn range_down_decreasing_integer(a: Integer) -> RangeDecreasingUnboundedInteger {
    RangeDecreasingUnboundedInteger(a)
}

type T = Interleave<RangeIncreasing<Integer>, RangeDecreasing<Integer>>;
#[derive(Clone)]
pub enum ExhaustiveRangeInteger {
    AllNonNegative(RangeIncreasing<Integer>),
    AllNonPositive(RangeDecreasing<Integer>),
    SomeOfEachSign(Chain<Once<Integer>, T>),
}

impl Iterator for ExhaustiveRangeInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        match *self {
            ExhaustiveRangeInteger::AllNonNegative(ref mut xs) => xs.next(),
            ExhaustiveRangeInteger::AllNonPositive(ref mut xs) => xs.next(),
            ExhaustiveRangeInteger::SomeOfEachSign(ref mut xs) => xs.next(),
        }
    }
}

pub fn exhaustive_range_integer(a: Integer, b: Integer) -> ExhaustiveRangeInteger {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    if a >= 0 {
        ExhaustiveRangeInteger::AllNonNegative(range_increasing_x(a, b))
    } else if b <= 0 {
        ExhaustiveRangeInteger::AllNonPositive(range_decreasing_x(a, b))
    } else {
        ExhaustiveRangeInteger::SomeOfEachSign(
            once(Integer::ZERO).chain(
                range_increasing_x(Integer::ONE, b)
                    .interleave(range_decreasing_x(a, Integer::NEGATIVE_ONE)),
            ),
        )
    }
}

pub fn exhaustive_positive_integers() -> RangeIncreasingUnboundedInteger {
    range_up_increasing_integer(Integer::ONE)
}

pub fn exhaustive_natural_integers() -> RangeIncreasingUnboundedInteger {
    range_up_increasing_integer(Integer::ZERO)
}

pub fn exhaustive_negative_integers() -> RangeDecreasingUnboundedInteger {
    range_down_decreasing_integer(Integer::NEGATIVE_ONE)
}

pub fn exhaustive_nonzero_integers(
) -> Interleave<RangeIncreasingUnboundedInteger, RangeDecreasingUnboundedInteger> {
    exhaustive_positive_integers().interleave(exhaustive_negative_integers())
}

pub fn exhaustive_integers() -> Chain<
    Once<Integer>,
    Interleave<RangeIncreasingUnboundedInteger, RangeDecreasingUnboundedInteger>,
> {
    once(Integer::ZERO).chain(exhaustive_nonzero_integers())
}

pub struct RandomPositiveIntegers(RandomPositiveNaturals);

impl Iterator for RandomPositiveIntegers {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        self.0.next().map(Natural::into_integer)
    }
}

pub fn random_positive_integers(seed: &[u32], scale: u32) -> RandomPositiveIntegers {
    RandomPositiveIntegers(random_positive_naturals(seed, scale))
}

pub struct RandomNaturalIntegers(RandomNaturals);

impl Iterator for RandomNaturalIntegers {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        self.0.next().map(Natural::into_integer)
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
        signs: random_x(&scramble(seed, "signs")),
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
        signs: random_x(&scramble(seed, "signs")),
        abs: random_natural_integers(&scramble(seed, "abs"), scale),
    }
}

pub struct RandomRangeInteger {
    rng: IsaacRng,
    diameter: Natural,
    a: Integer,
}

impl Iterator for RandomRangeInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        Some(random_natural_below(&mut self.rng, &(&self.diameter + 1)).into_integer() + &self.a)
    }
}

pub fn random_range_integer(seed: &[u32], a: Integer, b: Integer) -> RandomRangeInteger {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RandomRangeInteger {
        rng: IsaacRng::from_seed(seed),
        diameter: (b - &a).into_natural().unwrap(),
        a,
    }
}
