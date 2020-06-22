use std::iter::Peekable;
use std::marker::PhantomData;

use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use rand::{IsaacRng, Rand, Rng, SeedableRng};

use iterators::primitive_ints::{random_range, RandomRange};

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
        self.range
            .next()
            .map(|i| self.xs[usize::exact_from(i)].clone())
    }
}

pub fn random_from_vector<T>(seed: &[u32], xs: Vec<T>) -> RandomFromVector<T> {
    if xs.is_empty() {
        panic!("Cannot randomly generate values from an empty Vec.");
    }
    let limit = &xs.len() - 1;
    RandomFromVector {
        xs,
        range: random_range(seed, 0, u64::wrapping_from(limit)),
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
