use iterators::general::RangeIncreasing;
use iterators::primitive_ints::{
    exhaustive_range_signed, range_down_increasing, ExhaustiveRangeSigned,
};
use iterators::tuples::{exhaustive_pairs, ExhaustivePairs};
use itertools::{Interleave, Itertools};
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{One, PrimitiveFloat, PrimitiveSigned, PrimitiveUnsigned};
use malachite_nz::integer::Integer;
use prim_utils::primitive_float_utils::from_mantissa_and_exponent;
use std;
use std::iter::{once, Chain, Once};

struct PositiveMantissas<T: PrimitiveFloat>(
    RangeIncreasing<<T::SignedOfEqualWidth as PrimitiveSigned>::UnsignedOfEqualWidth>,
);

impl<T: PrimitiveFloat> PositiveMantissas<T> {
    const ONE: <T::SignedOfEqualWidth as PrimitiveSigned>::UnsignedOfEqualWidth =
        <<T::SignedOfEqualWidth as PrimitiveSigned>::UnsignedOfEqualWidth as One>::ONE;
}

impl<T: PrimitiveFloat> Iterator for PositiveMantissas<T> {
    type Item = <T::SignedOfEqualWidth as PrimitiveSigned>::UnsignedOfEqualWidth;

    fn next(&mut self) -> Option<<T::SignedOfEqualWidth as PrimitiveSigned>::UnsignedOfEqualWidth> {
        self.0
            .next()
            .map(|m| (m << 1) + PositiveMantissas::<T>::ONE)
    }
}

fn positive_mantissas<T: PrimitiveFloat>() -> PositiveMantissas<T> {
    PositiveMantissas(range_down_increasing(
        (PositiveMantissas::<T>::ONE << T::MANTISSA_WIDTH) - PositiveMantissas::<T>::ONE,
    ))
}

pub struct ExhaustivePositiveOrdinaryPrimitiveFloats<T: PrimitiveFloat>(
    ExhaustivePairs<PositiveMantissas<T>, ExhaustiveRangeSigned<i32>>,
);

impl<T: PrimitiveFloat> Iterator for ExhaustivePositiveOrdinaryPrimitiveFloats<T>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        while let Some((m, e)) = self.0.next() {
            let f = from_mantissa_and_exponent(T::SignedOfEqualWidth::from_unsigned_bitwise(m), e);
            if f.is_some() {
                return f;
            }
        }
        None
    }
}

pub fn exhaustive_positive_ordinary_primitive_floats<T: PrimitiveFloat>(
) -> ExhaustivePositiveOrdinaryPrimitiveFloats<T>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    ExhaustivePositiveOrdinaryPrimitiveFloats(exhaustive_pairs(
        positive_mantissas::<T>(),
        exhaustive_range_signed(T::MIN_EXPONENT, T::MAX_EXPONENT as i32),
    ))
}

pub struct ExhaustiveNegativeOrdinaryPrimitiveFloats<T: PrimitiveFloat>(
    ExhaustivePositiveOrdinaryPrimitiveFloats<T>,
);

impl<T: PrimitiveFloat> Iterator for ExhaustiveNegativeOrdinaryPrimitiveFloats<T>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.next().map(|f| -f)
    }
}

pub fn exhaustive_negative_ordinary_primitive_floats<T: PrimitiveFloat>(
) -> ExhaustiveNegativeOrdinaryPrimitiveFloats<T>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    ExhaustiveNegativeOrdinaryPrimitiveFloats(exhaustive_positive_ordinary_primitive_floats())
}

pub fn exhaustive_nonzero_ordinary_primitive_floats<T: PrimitiveFloat>() -> Interleave<
    ExhaustivePositiveOrdinaryPrimitiveFloats<T>,
    ExhaustiveNegativeOrdinaryPrimitiveFloats<T>,
>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    exhaustive_positive_ordinary_primitive_floats()
        .interleave(exhaustive_negative_ordinary_primitive_floats())
}

pub fn exhaustive_positive_primitive_floats<T: PrimitiveFloat>(
) -> Chain<Once<T>, ExhaustivePositiveOrdinaryPrimitiveFloats<T>>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    once(T::INFINITY).chain(exhaustive_positive_ordinary_primitive_floats())
}

pub fn exhaustive_negative_primitive_floats<T: PrimitiveFloat>(
) -> Chain<Once<T>, ExhaustiveNegativeOrdinaryPrimitiveFloats<T>>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    once(T::NEG_INFINITY).chain(exhaustive_negative_ordinary_primitive_floats())
}

pub fn exhaustive_nonzero_primitive_floats<T: PrimitiveFloat>() -> Chain<
    std::vec::IntoIter<T>,
    Interleave<
        ExhaustivePositiveOrdinaryPrimitiveFloats<T>,
        ExhaustiveNegativeOrdinaryPrimitiveFloats<T>,
    >,
>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    vec![T::NAN, T::INFINITY, T::NEG_INFINITY]
        .into_iter()
        .chain(exhaustive_nonzero_ordinary_primitive_floats())
}

pub fn exhaustive_primitive_floats<T: PrimitiveFloat>() -> Chain<
    std::vec::IntoIter<T>,
    Interleave<
        ExhaustivePositiveOrdinaryPrimitiveFloats<T>,
        ExhaustiveNegativeOrdinaryPrimitiveFloats<T>,
    >,
>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    vec![T::NAN, T::INFINITY, T::NEG_INFINITY, T::ZERO, T::NEG_ZERO]
        .into_iter()
        .chain(exhaustive_nonzero_ordinary_primitive_floats())
}
