use iterators::primitive_ints::{RandomRange, random_range};
use prim_utils::traits::Walkable;
use rand::{IsaacRng, Rand, Rng, SeedableRng};
use std::marker::PhantomData;

pub struct ExhaustiveFromVector<T> {
    xs: Vec<T>,
    i: usize,
}

impl<T: Clone> Iterator for ExhaustiveFromVector<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.i == self.xs.len() {
            None
        } else {
            let next = self.xs[self.i].clone();
            self.i += 1;
            Some(next)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.xs.len();
        (len, Some(len))
    }
}

pub fn exhaustive_from_vector<T>(xs: Vec<T>) -> ExhaustiveFromVector<T> {
    ExhaustiveFromVector { xs: xs, i: 0 }
}

pub struct RangeIncreasing<T: Walkable> {
    i: T,
    b: T,
    done: bool,
}

impl<T: Walkable> Iterator for RangeIncreasing<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.done {
            None
        } else {
            self.done = self.i == self.b;
            let ret = self.i;
            if !self.done {
                self.i.increment();
            }
            Some(ret)
        }
    }
}

pub fn range_increasing_x<T: Walkable>(a: T, b: T) -> RangeIncreasing<T> {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RangeIncreasing {
        i: a,
        b: b,
        done: false,
    }
}

pub struct RangeDecreasing<T: Walkable> {
    a: T,
    i: T,
    done: bool,
}

impl<T: Walkable> Iterator for RangeDecreasing<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.done {
            None
        } else {
            self.done = self.i == self.a;
            let ret = self.i;
            if !self.done {
                self.i.decrement();
            }
            Some(ret)
        }
    }
}

pub fn range_decreasing_x<T: Walkable>(a: T, b: T) -> RangeDecreasing<T> {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RangeDecreasing {
        a: a,
        i: b,
        done: false,
    }
}

pub struct Random<T: Rand> {
    rng: IsaacRng,
    boo: PhantomData<T>,
}

impl<T: Rand> Iterator for Random<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        Some(self.rng.gen::<T>())
    }
}

pub fn random_x<T: Rand>(seed: &[u32]) -> Random<T> {
    Random {
        rng: SeedableRng::from_seed(seed),
        boo: PhantomData,
    }
}

pub struct RandomFromVector<T> {
    xs: Vec<T>,
    range: RandomRange<usize>,
}

impl<T: Clone> Iterator for RandomFromVector<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.range.next().map(|i| self.xs[i].clone())
    }
}

pub fn random_from_vector<T>(seed: &[u32], xs: Vec<T>) -> RandomFromVector<T> {
    if xs.is_empty() {
        panic!("Cannot randomly generate values from an empty Vec.");
    }
    let limit = &xs.len() - 1;
    RandomFromVector {
        xs: xs,
        range: random_range(seed, 0, limit),
    }
}
