use iterators::dependent_pairs::exhaustive_dependent_pairs_infinite_log;
use iterators::general::CachedIterator;
use iterators::primitive_ints::exhaustive_positive_x;
use iterators::tuples::ZOrderTupleIndices;
use std::iter::repeat;

pub enum ExhaustiveFixedSizeVecsFromSingle<I: Iterator>
    where I::Item: Clone
{
    Zero(bool),
    One(I),
    MoreThanOne(CachedIterator<I>, ZOrderTupleIndices, bool, Option<ZOrderTupleIndices>),
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
            ExhaustiveFixedSizeVecsFromSingle::MoreThanOne(ref mut xs,
                                                           ref mut i,
                                                           ref mut stop_checking_size,
                                                           ref mut max_indices) => {
                let mut result = Vec::with_capacity(i.size());
                'outer: loop {
                    if max_indices.as_ref() == Some(i) {
                        return None;
                    }
                    for j in 0..i.size() {
                        match xs.get(i.0[j] as usize) {
                            Some(x) => result.push(x),
                            None => {
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
                    i.increment();
                    return Some(result);
                }
            }
        }
    }
}

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
            ExhaustiveFixedSizeVecsFromSingle::MoreThanOne(CachedIterator::new(xs),
                                                           ZOrderTupleIndices::new(size),
                                                           false,
                                                           None)
        }
    }
}

pub fn exhaustive_vecs<'a, I: Clone + Iterator + 'a>(xs: I)
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
