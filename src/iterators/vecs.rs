use std::iter::repeat;
use std::marker::PhantomData;

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::exhaustive::exhaustive_positive_primitive_ints;
use malachite_base::num::logic::traits::BitConvertible;
use malachite_base::tuples::exhaustive::exhaustive_pairs;
use malachite_base::vecs::exhaustive::exhaustive_fixed_length_vecs_from_single;
use malachite_nz::natural::random::special_random_natural_up_to_bits::*;
use malachite_nz::natural::random::special_random_natural_with_bits::*;
use malachite_nz::platform::Limb;
use rand::{IsaacRng, Rng, SeedableRng};

use iterators::common::scramble;
use iterators::dependent_pairs::exhaustive_dependent_pairs_infinite_log;
use iterators::integers_geometric::{
    range_up_geometric_u32, u32s_geometric, RangeUpGeometricU32, U32sGeometric,
};

fn exhaustive_vecs_more_than_one<'a, I: Clone + Iterator + 'a>(
    xs: I,
) -> Box<dyn Iterator<Item = Vec<I::Item>> + 'a>
where
    I::Item: Clone,
{
    let f = move |_: &(), size: &u64| {
        exhaustive_fixed_length_vecs_from_single(usize::exact_from(*size), xs.clone())
            .map(Option::Some)
            .chain(repeat(Option::None))
    };
    Box::new(
        exhaustive_dependent_pairs_infinite_log((), exhaustive_positive_primitive_ints(), f)
            .flat_map(|(_, v)| v),
    )
}

pub enum ExhaustiveVecs<I: Iterator>
where
    I::Item: Clone,
{
    Zero(bool),
    One(I::Item, Vec<I::Item>),
    MoreThanOne(bool, Box<dyn Iterator<Item = Vec<I::Item>>>),
}

impl<I: Iterator> Iterator for ExhaustiveVecs<I>
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
pub fn exhaustive_vecs<I: 'static + Clone + Iterator>(xs: I) -> ExhaustiveVecs<I>
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

pub fn exhaustive_vecs_min_length<I: 'static + Clone + Iterator>(
    min_len: u64,
    xs: I,
) -> Box<dyn Iterator<Item = Vec<I::Item>>>
where
    I::Item: Clone,
{
    match min_len {
        0 => Box::new(exhaustive_vecs(xs)),
        1 => Box::new(exhaustive_vecs(xs).skip(1)),
        _ => Box::new(
            exhaustive_pairs(
                exhaustive_fixed_length_vecs_from_single(usize::exact_from(min_len), xs.clone()),
                exhaustive_vecs(xs),
            )
            .map(|(mut xs, mut ys)| {
                xs.append(&mut ys);
                xs
            }),
        ),
    }
}

pub struct RandomFixedLengthVecs<I: Iterator> {
    length: usize,
    xs: I,
}

impl<I> Iterator for RandomFixedLengthVecs<I>
where
    I: Iterator,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        Some((&mut self.xs).take(self.length).collect())
    }
}

//TODO test
pub fn random_fixed_length_vecs<I>(length: usize, xs: I) -> RandomFixedLengthVecs<I>
where
    I: Iterator,
{
    RandomFixedLengthVecs { length, xs }
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
                .take(usize::exact_from(self.lengths.next().unwrap()))
                .collect(),
        )
    }
}

//TODO test
pub fn random_vecs<I>(seed: &[u32], scale: u32, xs_gen: &dyn Fn(&[u32]) -> I) -> RandomVecs<I>
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
                .take(usize::exact_from(self.lengths.next().unwrap()))
                .collect(),
        )
    }
}

pub fn random_vecs_min_length<I>(
    seed: &[u32],
    scale: u32,
    min_length: u64,
    xs_gen: &dyn Fn(&[u32]) -> I,
) -> RandomVecsMinLength<I>
where
    I: Iterator,
{
    RandomVecsMinLength {
        lengths: range_up_geometric_u32(
            &scramble(seed, "lengths"),
            scale,
            u32::exact_from(min_length),
        ),
        xs: xs_gen(&scramble(seed, "xs")),
    }
}

pub struct SpecialRandomFixedLengthUnsignedVecs<T: PrimitiveUnsigned> {
    length: usize,
    rng: Box<IsaacRng>,
    boo: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> Iterator for SpecialRandomFixedLengthUnsignedVecs<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.length == 0 {
            return Some(Vec::new());
        }
        let limbs: Vec<Limb> = limbs_special_random_up_to_bits(
            &mut self.rng,
            u64::exact_from(self.length) << T::LOG_WIDTH,
        );
        Some(T::vec_from_other_type_slice(&limbs))
    }
}

//TODO test
pub fn special_random_fixed_length_unsigned_vecs<T: PrimitiveUnsigned>(
    seed: &[u32],
    length: usize,
) -> SpecialRandomFixedLengthUnsignedVecs<T> {
    SpecialRandomFixedLengthUnsignedVecs {
        length,
        rng: Box::new(IsaacRng::from_seed(seed)),
        boo: PhantomData,
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
        let limbs: Vec<Limb> =
            limbs_special_random_up_to_bits(&mut self.rng, u64::from(len << T::LOG_WIDTH));
        Some(T::vec_from_other_type_slice(&limbs))
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
        let limbs: Vec<Limb> =
            limbs_special_random_up_to_bits(&mut self.rng, u64::from(len << T::LOG_WIDTH));
        Some(T::vec_from_other_type_slice(&limbs))
    }
}

//TODO test
pub fn special_random_unsigned_vecs_min_length<T: PrimitiveUnsigned>(
    seed: &[u32],
    scale: u32,
    min_length: u64,
) -> SpecialRandomUnsignedVecsMinLength<T> {
    SpecialRandomUnsignedVecsMinLength {
        lengths: range_up_geometric_u32(
            &scramble(seed, "lengths"),
            scale,
            u32::exact_from(min_length),
        ),
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
