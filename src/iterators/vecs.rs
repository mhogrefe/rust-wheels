use iterators::common::scramble;
use iterators::dependent_pairs::exhaustive_dependent_pairs_infinite_log;
use iterators::general::CachedIterator;
use iterators::integers_geometric::{NaturalU32sGeometric, natural_u32s_geometric};
use iterators::primitive_ints::exhaustive_positive_x;
use iterators::tuples::ZOrderTupleIndices;
use std::iter::repeat;

pub enum ExhaustiveFixedSizeVecsFromSingle<I: Iterator>
    where I::Item: Clone
{
    Zero(bool),
    One(I),
    MoreThanOne(bool, CachedIterator<I>, ZOrderTupleIndices, bool, Option<ZOrderTupleIndices>),
}

impl<I: Iterator> Iterator for ExhaustiveFixedSizeVecsFromSingle<I>
    where I::Item: Clone
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
            ExhaustiveFixedSizeVecsFromSingle::MoreThanOne(ref mut done,
                                                           ref mut xs,
                                                           ref mut i,
                                                           ref mut stop_checking_size,
                                                           ref mut max_indices) => {
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
pub fn exhaustive_fixed_size_vecs_from_single<I: Iterator>
    (size: usize,
     xs: I)
     -> ExhaustiveFixedSizeVecsFromSingle<I>
    where I::Item: Clone
{
    match size {
        0 => ExhaustiveFixedSizeVecsFromSingle::Zero(true),
        1 => ExhaustiveFixedSizeVecsFromSingle::One(xs),
        _ => {
            ExhaustiveFixedSizeVecsFromSingle::MoreThanOne(false,
                                                           CachedIterator::new(xs),
                                                           ZOrderTupleIndices::new(size),
                                                           false,
                                                           None)
        }
    }
}

fn exhaustive_vecs_more_than_one<'a, I: Clone + Iterator + 'a>
    (xs: I)
     -> Box<Iterator<Item = Vec<I::Item>> + 'a>
    where I::Item: Clone
{
    let f = move |size: &usize| {
        exhaustive_fixed_size_vecs_from_single(*size, xs.clone())
            .map(Option::Some)
            .chain(repeat(Option::None))
    };
    Box::new(exhaustive_dependent_pairs_infinite_log(exhaustive_positive_x::<usize>(), f)
                 .map(|(_, v)| v)
                 .filter(|v| v.is_some())
                 .map(|v| v.unwrap()))
}

pub enum ExhaustiveVecs<'a, I: Iterator>
    where I::Item: Clone
{
    Zero(bool),
    One(I::Item, Vec<I::Item>),
    MoreThanOne(bool, Box<Iterator<Item = Vec<I::Item>> + 'a>),
}

impl<'a, I: Iterator> Iterator for ExhaustiveVecs<'a, I>
    where I::Item: Clone
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
    where I::Item: Clone
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

pub struct RandomVecs<I>
    where I: Iterator
{
    lengths: NaturalU32sGeometric,
    xs: I,
}

impl<I> Iterator for RandomVecs<I>
    where I: Iterator
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        Some((&mut self.xs).take(self.lengths.next().unwrap() as usize).collect())
    }
}

//TODO test
pub fn random_vecs<I>(seed: &[u32], scale: u32, xs_gen: &Fn(&[u32]) -> I) -> RandomVecs<I>
    where I: Iterator
{
    RandomVecs {
        lengths: natural_u32s_geometric(&scramble(seed, "lengths"), scale),
        xs: xs_gen(&scramble(seed, "xs")),
    }
}
