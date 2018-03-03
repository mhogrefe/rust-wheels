use iterators::general::RangeIncreasing;
use iterators::primitive_ints::{exhaustive_range_signed, range_down_increasing,
                                ExhaustiveRangeSigned};
use iterators::tuples::{exhaustive_pairs, ExhaustivePairs};
use itertools::{Interleave, Itertools};
use malachite_base::num::{Float, One, PrimitiveSigned, PrimitiveUnsigned};
use malachite_nz::integer::Integer;
use prim_utils::float_utils::from_mantissa_and_exponent;
use std;
use std::iter::{once, Chain, Once};

struct PositiveMantissas<T: 'static + Float>(
    RangeIncreasing<<T::SignedOfEqualWidth as PrimitiveSigned>::UnsignedOfEqualWidth>,
);

impl<T: 'static + Float> PositiveMantissas<T> {
    const ONE: <T::SignedOfEqualWidth as PrimitiveSigned>::UnsignedOfEqualWidth =
        <<T::SignedOfEqualWidth as PrimitiveSigned>::UnsignedOfEqualWidth as One>::ONE;
}

impl<T: 'static + Float> Iterator for PositiveMantissas<T> {
    type Item = <T::SignedOfEqualWidth as PrimitiveSigned>::UnsignedOfEqualWidth;

    fn next(&mut self) -> Option<<T::SignedOfEqualWidth as PrimitiveSigned>::UnsignedOfEqualWidth> {
        self.0
            .next()
            .map(|m| (m << 1) + PositiveMantissas::<T>::ONE)
    }
}

fn positive_mantissas<T: 'static + Float>() -> PositiveMantissas<T> {
    PositiveMantissas(range_down_increasing(
        (PositiveMantissas::<T>::ONE << T::MANTISSA_WIDTH) - PositiveMantissas::<T>::ONE,
    ))
}

pub struct ExhaustivePositiveOrdinaryFloats<T: 'static + Float>(
    ExhaustivePairs<PositiveMantissas<T>, ExhaustiveRangeSigned<i32>>,
);

impl<T: 'static + Float> Iterator for ExhaustivePositiveOrdinaryFloats<T>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: From<Integer>,
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

pub fn exhaustive_positive_ordinary_floats<T: 'static + Float>(
) -> ExhaustivePositiveOrdinaryFloats<T>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: From<Integer>,
{
    ExhaustivePositiveOrdinaryFloats(exhaustive_pairs(
        positive_mantissas::<T>(),
        exhaustive_range_signed(T::MIN_EXPONENT, T::MAX_EXPONENT as i32),
    ))
}

pub struct ExhaustiveNegativeOrdinaryFloats<T: 'static + Float>(
    ExhaustivePositiveOrdinaryFloats<T>,
);

impl<T: 'static + Float> Iterator for ExhaustiveNegativeOrdinaryFloats<T>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: From<Integer>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.next().map(|f| -f)
    }
}

pub fn exhaustive_negative_ordinary_floats<T: 'static + Float>(
) -> ExhaustiveNegativeOrdinaryFloats<T>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: From<Integer>,
{
    ExhaustiveNegativeOrdinaryFloats(exhaustive_positive_ordinary_floats())
}

pub fn exhaustive_nonzero_ordinary_floats<T: 'static + Float>(
) -> Interleave<ExhaustivePositiveOrdinaryFloats<T>, ExhaustiveNegativeOrdinaryFloats<T>>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: From<Integer>,
{
    exhaustive_positive_ordinary_floats().interleave(exhaustive_negative_ordinary_floats())
}

pub fn exhaustive_positive_floats<T: 'static + Float>(
) -> Chain<Once<T>, ExhaustivePositiveOrdinaryFloats<T>>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: From<Integer>,
{
    once(T::INFINITY).chain(exhaustive_positive_ordinary_floats())
}

pub fn exhaustive_negative_floats<T: 'static + Float>(
) -> Chain<Once<T>, ExhaustiveNegativeOrdinaryFloats<T>>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: From<Integer>,
{
    once(T::NEG_INFINITY).chain(exhaustive_negative_ordinary_floats())
}

pub fn exhaustive_nonzero_floats<T: 'static + Float>() -> Chain<
    std::vec::IntoIter<T>,
    Interleave<ExhaustivePositiveOrdinaryFloats<T>, ExhaustiveNegativeOrdinaryFloats<T>>,
>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: From<Integer>,
{
    vec![T::NAN, T::INFINITY, T::NEG_INFINITY]
        .into_iter()
        .chain(exhaustive_nonzero_ordinary_floats())
}

pub fn exhaustive_floats<T: 'static + Float>() -> Chain<
    std::vec::IntoIter<T>,
    Interleave<ExhaustivePositiveOrdinaryFloats<T>, ExhaustiveNegativeOrdinaryFloats<T>>,
>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: From<Integer>,
{
    vec![T::NAN, T::INFINITY, T::NEG_INFINITY, T::ZERO, T::NEG_ZERO]
        .into_iter()
        .chain(exhaustive_nonzero_ordinary_floats())
}
