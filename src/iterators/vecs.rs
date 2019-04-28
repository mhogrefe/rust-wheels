use iterators::common::scramble;
use iterators::dependent_pairs::exhaustive_dependent_pairs_infinite_log;
use iterators::general::CachedIterator;
use iterators::integers_geometric::{
    range_up_geometric_u32, u32s_geometric, RangeUpGeometricU32, U32sGeometric,
};
use iterators::primitive_ints::exhaustive_positive;
use iterators::tuples::{exhaustive_pairs, ZOrderTupleIndices};
use malachite_base::num::integers::PrimitiveInteger;
use malachite_base::num::traits::Parity;
use malachite_base::num::unsigneds::PrimitiveUnsigned;
use malachite_nz::natural::random::special_random_natural_up_to_bits::*;
use malachite_nz::natural::random::special_random_natural_with_bits::*;
use rand::{IsaacRng, Rng, SeedableRng};
use std::iter::repeat;
use std::marker::PhantomData;

pub enum ExhaustiveFixedSizeVecsFromSingle<I: Iterator>
where
    I::Item: Clone,
{
    Zero(bool),
    One(I),
    MoreThanOne(
        bool,
        CachedIterator<I>,
        ZOrderTupleIndices,
        bool,
        Option<ZOrderTupleIndices>,
    ),
}

impl<I: Iterator> Iterator for ExhaustiveFixedSizeVecsFromSingle<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        match *self {
            ExhaustiveFixedSizeVecsFromSingle::Zero(ref mut first) => {
                if *first {
                    *first = false;
                    Some(Vec::new())
                } else {
                    None
                }
            }
            ExhaustiveFixedSizeVecsFromSingle::One(ref mut xs) => xs.next().map(|x| vec![x]),
            ExhaustiveFixedSizeVecsFromSingle::MoreThanOne(
                ref mut done,
                ref mut xs,
                ref mut i,
                ref mut stop_checking_size,
                ref mut max_indices,
            ) => {
                let mut result = Vec::with_capacity(i.size());
                'outer: loop {
                    if *done {
                        return None;
                    }
                    for j in 0..i.size() {
                        match xs.get(i.0[j] as usize) {
                            Some(x) => result.push(x),
                            None => {
                                if max_indices.as_ref() == Some(i) {
                                    return None;
                                }
                                i.increment();
                                result.clear();
                                continue 'outer;
                            }
                        }
                    }
                    if !*stop_checking_size {
                        if let Some(size) = xs.currently_known_size() {
                            let size = size as u64;
                            let mut max_vec = Vec::new();
                            max_vec.resize(i.size(), size - 1);
                            *max_indices = Some(ZOrderTupleIndices(max_vec));
                            *stop_checking_size = true;
                        }
                    }
                    if max_indices.as_ref() == Some(i) {
                        *done = true;
                    } else {
                        i.increment();
                    }
                    return Some(result);
                }
            }
        }
    }
}

//TODO test
pub fn exhaustive_fixed_size_vecs_from_single<I: Iterator>(
    size: u64,
    xs: I,
) -> ExhaustiveFixedSizeVecsFromSingle<I>
where
    I::Item: Clone,
{
    match size {
        0 => ExhaustiveFixedSizeVecsFromSingle::Zero(true),
        1 => ExhaustiveFixedSizeVecsFromSingle::One(xs),
        _ => ExhaustiveFixedSizeVecsFromSingle::MoreThanOne(
            false,
            CachedIterator::new(xs),
            ZOrderTupleIndices::new(size),
            false,
            None,
        ),
    }
}

fn exhaustive_vecs_more_than_one<'a, I: Clone + Iterator + 'a>(
    xs: I,
) -> Box<Iterator<Item = Vec<I::Item>> + 'a>
where
    I::Item: Clone,
{
    let f = move |size: &u64| {
        exhaustive_fixed_size_vecs_from_single(*size, xs.clone())
            .map(Option::Some)
            .chain(repeat(Option::None))
    };
    Box::new(exhaustive_dependent_pairs_infinite_log(exhaustive_positive(), f).flat_map(|(_, v)| v))
}

pub enum ExhaustiveVecs<'a, I: Iterator>
where
    I::Item: Clone,
{
    Zero(bool),
    One(I::Item, Vec<I::Item>),
    MoreThanOne(bool, Box<Iterator<Item = Vec<I::Item>> + 'a>),
}

impl<'a, I: Iterator> Iterator for ExhaustiveVecs<'a, I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        match *self {
            ExhaustiveVecs::Zero(ref mut first) => {
                if *first {
                    *first = false;
                    Some(Vec::new())
                } else {
                    None
                }
            }
            ExhaustiveVecs::One(ref x, ref mut xs) => {
                let copy = xs.clone();
                xs.push(x.clone());
                Some(copy)
            }
            ExhaustiveVecs::MoreThanOne(ref mut first, ref mut xs) => {
                if *first {
                    *first = false;
                    Some(Vec::new())
                } else {
                    xs.next()
                }
            }
        }
    }
}

//TODO test
pub fn exhaustive_vecs<'a, I: Clone + Iterator + 'a>(xs: I) -> ExhaustiveVecs<'a, I>
where
    I::Item: Clone,
{
    let mut xs_cloned = xs.clone();
    let first = match xs_cloned.next() {
        None => return ExhaustiveVecs::Zero(true),
        Some(x) => x,
    };
    if xs_cloned.next().is_none() {
        ExhaustiveVecs::One(first, Vec::new())
    } else {
        ExhaustiveVecs::MoreThanOne(true, exhaustive_vecs_more_than_one(xs))
    }
}

pub fn exhaustive_vecs_min_length<'a, I: 'static + Clone + Iterator + 'a>(
    min_len: u64,
    xs: I,
) -> Box<Iterator<Item = Vec<I::Item>>>
where
    I::Item: Clone,
{
    match min_len {
        0 => Box::new(exhaustive_vecs(xs)),
        1 => Box::new(exhaustive_vecs(xs).skip(1)),
        _ => Box::new(
            exhaustive_pairs(
                exhaustive_fixed_size_vecs_from_single(min_len, xs.clone()),
                exhaustive_vecs(xs),
            )
            .map(|(mut xs, mut ys)| {
                xs.append(&mut ys);
                xs
            }),
        ),
    }
}

pub struct RandomVecs<I>
where
    I: Iterator,
{
    lengths: U32sGeometric,
    xs: I,
}

impl<I> Iterator for RandomVecs<I>
where
    I: Iterator,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        Some(
            (&mut self.xs)
                .take(self.lengths.next().unwrap() as usize)
                .collect(),
        )
    }
}

//TODO test
pub fn random_vecs<I>(seed: &[u32], scale: u32, xs_gen: &Fn(&[u32]) -> I) -> RandomVecs<I>
where
    I: Iterator,
{
    RandomVecs {
        lengths: u32s_geometric(&scramble(seed, "lengths"), scale),
        xs: xs_gen(&scramble(seed, "xs")),
    }
}

pub struct RandomVecsMinLength<I>
where
    I: Iterator,
{
    lengths: RangeUpGeometricU32,
    xs: I,
}

impl<I> Iterator for RandomVecsMinLength<I>
where
    I: Iterator,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        Some(
            (&mut self.xs)
                .take(self.lengths.next().unwrap() as usize)
                .collect(),
        )
    }
}

pub fn random_vecs_min_length<I>(
    seed: &[u32],
    scale: u32,
    min_length: u64,
    xs_gen: &Fn(&[u32]) -> I,
) -> RandomVecsMinLength<I>
where
    I: Iterator,
{
    RandomVecsMinLength {
        lengths: range_up_geometric_u32(&scramble(seed, "lengths"), scale, min_length as u32),
        xs: xs_gen(&scramble(seed, "xs")),
    }
}

pub struct SpecialRandomUnsignedVecs<T: PrimitiveUnsigned> {
    lengths: U32sGeometric,
    rng: Box<IsaacRng>,
    boo: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> Iterator for SpecialRandomUnsignedVecs<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        let len = self.lengths.next().unwrap();
        if len == 0 {
            return Some(Vec::new());
        }
        let mut limbs =
            limbs_special_random_up_to_bits(&mut self.rng, u64::from(len << T::LOG_WIDTH));
        //TODO make this more generic
        if T::WIDTH == u64::WIDTH && (limbs.len() as u64).odd() {
            limbs.push(0);
        }
        let mut result = vec![T::ZERO; limbs.len() << u32::LOG_WIDTH >> T::LOG_WIDTH];
        T::copy_from_u32_slice(&mut result, &limbs);
        Some(result)
    }
}

//TODO test
pub fn special_random_unsigned_vecs<T: PrimitiveUnsigned>(
    seed: &[u32],
    scale: u32,
) -> SpecialRandomUnsignedVecs<T> {
    SpecialRandomUnsignedVecs {
        lengths: u32s_geometric(&scramble(seed, "lengths"), scale),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "xs"))),
        boo: PhantomData,
    }
}

pub struct SpecialRandomUnsignedVecsMinLength<T: PrimitiveUnsigned> {
    lengths: RangeUpGeometricU32,
    rng: Box<IsaacRng>,
    boo: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> Iterator for SpecialRandomUnsignedVecsMinLength<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        let len = self.lengths.next().unwrap();
        if len == 0 {
            return Some(Vec::new());
        }
        let mut limbs =
            limbs_special_random_up_to_bits(&mut self.rng, u64::from(len << T::LOG_WIDTH));
        //TODO make this more generic
        if T::WIDTH == u64::WIDTH && (limbs.len() as u64).odd() {
            limbs.push(0);
        }
        let mut result = vec![T::ZERO; limbs.len() << u32::LOG_WIDTH >> T::LOG_WIDTH];
        T::copy_from_u32_slice(&mut result, &limbs);
        Some(result)
    }
}

//TODO test
pub fn special_random_unsigned_vecs_min_length<T: PrimitiveUnsigned>(
    seed: &[u32],
    scale: u32,
    min_length: u64,
) -> SpecialRandomUnsignedVecsMinLength<T> {
    SpecialRandomUnsignedVecsMinLength {
        lengths: range_up_geometric_u32(&scramble(seed, "lengths"), scale, min_length as u32),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "xs"))),
        boo: PhantomData,
    }
}

pub struct SpecialRandomBoolVecs {
    lengths: U32sGeometric,
    rng: Box<IsaacRng>,
}

impl Iterator for SpecialRandomBoolVecs {
    type Item = Vec<bool>;

    fn next(&mut self) -> Option<Vec<bool>> {
        let n = special_random_natural_with_bits(
            &mut self.rng,
            u64::from(self.lengths.next().unwrap()),
        );
        let mut bits = n.to_bits_asc();
        // The last element of bits is always true; flip it to false 50% of the time.
        if !bits.is_empty() && self.rng.gen() {
            *bits.last_mut().unwrap() = false;
        }
        Some(bits)
    }
}

//TODO test
pub fn special_random_bool_vecs(seed: &[u32], scale: u32) -> SpecialRandomBoolVecs {
    SpecialRandomBoolVecs {
        lengths: u32s_geometric(&scramble(seed, "lengths"), scale),
        rng: Box::new(IsaacRng::from_seed(&scramble(seed, "xs"))),
    }
}
