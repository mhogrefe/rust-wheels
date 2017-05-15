use iterators::common::scramble;
use iterators::general::{Random, random_x};
use iterators::naturals::{RandomNaturals, random_naturals, RandomPositiveNaturals,
                          random_positive_naturals};
use itertools::{Interleave, Itertools};
use malachite::integer::Integer;
use malachite::natural::Natural;
use rand::{IsaacRng, SeedableRng};
use std::iter::{Chain, Once, once};

pub struct RangeIncreasingInteger {
    i: Integer,
    b: Integer,
    done: bool,
}

impl Iterator for RangeIncreasingInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
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

pub struct RangeDecreasingInteger {
    a: Integer,
    i: Integer,
    done: bool,
}

impl Iterator for RangeDecreasingInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
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

pub struct RangeIncreasingUnboundedInteger(Integer);

impl Iterator for RangeIncreasingUnboundedInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let ret = self.0.clone();
        self.0 += 1;
        Some(ret)
    }
}

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

pub fn range_increasing_integer(a: Integer, b: Integer) -> RangeIncreasingInteger {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RangeIncreasingInteger {
        i: a,
        b: b,
        done: false,
    }
}

pub fn range_decreasing_integer(a: Integer, b: Integer) -> RangeDecreasingInteger {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RangeDecreasingInteger {
        a: a,
        i: b,
        done: false,
    }
}

pub enum ExhaustiveRangeInteger {
    AllNonNegative(RangeIncreasingInteger),
    AllNonPositive(RangeDecreasingInteger),
    SomeOfEachSign(Chain<Once<Integer>,
                         Interleave<RangeIncreasingInteger, RangeDecreasingInteger>>),
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
        ExhaustiveRangeInteger::AllNonNegative(range_increasing_integer(a, b))
    } else if b <= 0 {
        ExhaustiveRangeInteger::AllNonPositive(range_decreasing_integer(a, b))
    } else {
        ExhaustiveRangeInteger::SomeOfEachSign(
                once(Integer::from(0)).chain(
                    range_increasing_integer(Integer::from(1), b)
                        .interleave(
                            range_decreasing_integer(
                                a,
                                Integer::from(-1)
                            )
                        )
                )
            )
    }
}

pub fn exhaustive_positive_integers() -> RangeIncreasingUnboundedInteger {
    range_up_increasing_integer(Integer::from(1))
}

pub fn exhaustive_natural_integers() -> RangeIncreasingUnboundedInteger {
    range_up_increasing_integer(Integer::from(0))
}

pub fn exhaustive_negative_integers() -> RangeDecreasingUnboundedInteger {
    range_down_decreasing_integer(Integer::from(-1))
}

pub fn exhaustive_nonzero_integers
    ()
    -> Interleave<RangeIncreasingUnboundedInteger, RangeDecreasingUnboundedInteger>
{
    exhaustive_positive_integers().interleave(exhaustive_negative_integers())
}

pub fn exhaustive_integers()
    -> Chain<Once<Integer>,
             Interleave<RangeIncreasingUnboundedInteger, RangeDecreasingUnboundedInteger>>
{
    once(Integer::from(0)).chain(exhaustive_nonzero_integers())
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
    diameter: Integer,
    a: Integer,
}

impl Iterator for RandomRangeInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let mut random = self.diameter.clone();
        random.random_below(&mut self.rng);
        random += &self.a;
        Some(random)
    }
}

pub fn random_range_integer(seed: &[u32], a: Integer, b: Integer) -> RandomRangeInteger {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    let mut diameter = b - &a;
    diameter += 1;
    RandomRangeInteger {
        rng: IsaacRng::from_seed(seed),
        diameter,
        a,
    }
}
