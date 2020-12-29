use std::marker::PhantomData;

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitConvertible;
use malachite_nz::platform::Limb;
use rand::{IsaacRng, Rng, SeedableRng};

use iterators::common::scramble;
use iterators::integers_geometric::{
    range_up_geometric_u32, u32s_geometric, RangeUpGeometricU32, U32sGeometric,
};
use iterators::naturals::{
    limbs_special_random_up_to_bits_old, special_random_natural_with_bits_old,
};

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
        let limbs: Vec<Limb> = limbs_special_random_up_to_bits_old(
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
            limbs_special_random_up_to_bits_old(&mut self.rng, u64::from(len << T::LOG_WIDTH));
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
            limbs_special_random_up_to_bits_old(&mut self.rng, u64::from(len << T::LOG_WIDTH));
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
        let n = special_random_natural_with_bits_old(
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
