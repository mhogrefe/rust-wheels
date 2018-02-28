use iterators::primitive_ints::{random_range, RandomRange};
use malachite_base::misc::Walkable;
use rand::{IsaacRng, Rand, Rng, SeedableRng};
use std::fmt::Display;
use std::iter::Peekable;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct RangeIncreasing<T: Walkable> {
    i: T,
    b: T,
    done: bool,
}

impl<T: Clone + Walkable> Iterator for RangeIncreasing<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.done {
            None
        } else {
            self.done = self.i == self.b;
            let ret = self.i.clone();
            if !self.done {
                self.i.increment();
            }
            Some(ret)
        }
    }
}

pub fn range_increasing<T: Display + Walkable>(a: T, b: T) -> RangeIncreasing<T> {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RangeIncreasing {
        i: a,
        b,
        done: false,
    }
}

#[derive(Clone)]
pub struct RangeDecreasing<T: Walkable> {
    a: T,
    i: T,
    done: bool,
}

impl<T: Clone + Walkable> Iterator for RangeDecreasing<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.done {
            None
        } else {
            self.done = self.i == self.a;
            let ret = self.i.clone();
            if !self.done {
                self.i.decrement();
            }
            Some(ret)
        }
    }
}

pub fn range_decreasing<T: Display + Walkable>(a: T, b: T) -> RangeDecreasing<T> {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RangeDecreasing {
        a,
        i: b,
        done: false,
    }
}

pub struct Random<T: Rand> {
    pub(crate) rng: Box<IsaacRng>,
    boo: PhantomData<*const T>,
}

impl<T: Rand> Iterator for Random<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        Some(self.rng.gen::<T>())
    }
}

pub fn random<T: Rand>(seed: &[u32]) -> Random<T> {
    Random {
        rng: Box::new(IsaacRng::from_seed(seed)),
        boo: PhantomData,
    }
}

pub struct RandomFromVector<T> {
    xs: Vec<T>,
    range: RandomRange<u64>,
}

impl<T: Clone> Iterator for RandomFromVector<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.range.next().map(|i| self.xs[i as usize].clone())
    }
}

pub fn random_from_vector<T>(seed: &[u32], xs: Vec<T>) -> RandomFromVector<T> {
    if xs.is_empty() {
        panic!("Cannot randomly generate values from an empty Vec.");
    }
    let limit = &xs.len() - 1;
    RandomFromVector {
        xs,
        range: random_range(seed, 0, limit as u64),
    }
}

pub struct CachedIterator<I: Iterator>
where
    I::Item: Clone,
{
    xs: Peekable<I>,
    cache: Vec<I::Item>,
    done: bool,
}

impl<I: Iterator> Iterator for CachedIterator<I>
where
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.done {
            None
        } else {
            let ox = self.xs.next();
            if let Some(ref x) = ox {
                self.cache.push(x.clone());
                if self.xs.peek().is_none() {
                    self.done = true;
                }
            } else {
                self.done = true;
            }
            ox
        }
    }
}

impl<I: Iterator> CachedIterator<I>
where
    I::Item: Clone,
{
    pub fn get(&mut self, index: usize) -> Option<I::Item> {
        let old_len = self.cache.len();
        if index < old_len {
            Some(self.cache[index].clone())
        } else {
            if self.done {
                return None;
            }
            for _ in old_len..(index + 1) {
                self.next()?;
            }
            Some(self.cache.last().unwrap().clone())
        }
    }

    pub fn currently_known_size(&self) -> Option<usize> {
        if self.done {
            Some(self.cache.len())
        } else {
            None
        }
    }
}

impl<I: Iterator> CachedIterator<I>
where
    I::Item: Clone,
{
    pub fn new(xs: I) -> CachedIterator<I> {
        CachedIterator {
            xs: xs.peekable(),
            cache: Vec::new(),
            done: false,
        }
    }
}
