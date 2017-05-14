use itertools::{Interleave, Itertools};
use malachite::integer::Integer;
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

pub struct RangeIncreasingUnboundedInteger {
    i: Integer,
}

impl Iterator for RangeIncreasingUnboundedInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let ret = self.i.clone();
        self.i += 1;
        Some(ret)
    }
}

pub struct RangeDecreasingUnboundedInteger {
    i: Integer,
}

impl Iterator for RangeDecreasingUnboundedInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let ret = self.i.clone();
        self.i -= 1;
        Some(ret)
    }
}

pub fn range_up_increasing_integer(a: Integer) -> RangeIncreasingUnboundedInteger {
    RangeIncreasingUnboundedInteger { i: a }
}

pub fn range_down_decreasing_integer(a: Integer) -> RangeDecreasingUnboundedInteger {
    RangeDecreasingUnboundedInteger { i: a }
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

//TODO test
pub fn exhaustive_positive_integers() -> RangeIncreasingUnboundedInteger {
    range_up_increasing_integer(Integer::from(1))
}

//TODO test
pub fn exhaustive_natural_integers() -> RangeIncreasingUnboundedInteger {
    range_up_increasing_integer(Integer::from(0))
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
