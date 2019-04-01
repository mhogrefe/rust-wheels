use iterators::common::scramble;
use iterators::general::{random, random_from_vector, Random, RandomFromVector, RangeIncreasing};
use iterators::integers_geometric::{i32s_geometric, I32sGeometric};
use iterators::primitive_ints::{
    exhaustive_range_signed, random_range, range_down_increasing, ExhaustiveRangeSigned,
    RandomRange,
};
use iterators::tuples::{exhaustive_pairs, ExhaustivePairs};
use itertools::{Interleave, Itertools};
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{
    One, PrimitiveFloat, PrimitiveSigned, PrimitiveUnsigned, SignificantBits, Zero,
};
use malachite_nz::integer::Integer;
use prim_utils::primitive_float_utils::{
    checked_from_mantissa_and_exponent, from_mantissa_and_exponent,
};
use rand::{IsaacRng, Rand, Rng, SeedableRng};
use std::cmp::min;
use std::iter::{once, Chain, Once};

struct ExhaustivePositiveMantissas<T: PrimitiveFloat>(
    RangeIncreasing<<T::SignedOfEqualWidth as PrimitiveSigned>::UnsignedOfEqualWidth>,
);

impl<T: PrimitiveFloat> ExhaustivePositiveMantissas<T> {
    const ONE: <T::SignedOfEqualWidth as PrimitiveSigned>::UnsignedOfEqualWidth =
        <<T::SignedOfEqualWidth as PrimitiveSigned>::UnsignedOfEqualWidth as One>::ONE;
}

impl<T: PrimitiveFloat> Iterator for ExhaustivePositiveMantissas<T> {
    type Item = <T::SignedOfEqualWidth as PrimitiveSigned>::UnsignedOfEqualWidth;

    fn next(&mut self) -> Option<<T::SignedOfEqualWidth as PrimitiveSigned>::UnsignedOfEqualWidth> {
        self.0
            .next()
            .map(|m| (m << 1) + ExhaustivePositiveMantissas::<T>::ONE)
    }
}

fn exhaustive_positive_mantissas<T: PrimitiveFloat>() -> ExhaustivePositiveMantissas<T> {
    ExhaustivePositiveMantissas(range_down_increasing(
        (ExhaustivePositiveMantissas::<T>::ONE << T::MANTISSA_WIDTH.into())
            - ExhaustivePositiveMantissas::<T>::ONE,
    ))
}

pub struct ExhaustivePositiveFinitePrimitiveFloats<T: PrimitiveFloat>(
    ExhaustivePairs<ExhaustivePositiveMantissas<T>, ExhaustiveRangeSigned<i32>>,
);

impl<T: PrimitiveFloat> Iterator for ExhaustivePositiveFinitePrimitiveFloats<T>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        while let Some((m, e)) = self.0.next() {
            let f = checked_from_mantissa_and_exponent(
                T::SignedOfEqualWidth::from_unsigned_bitwise(m),
                e,
            );
            if f.is_some() {
                return f;
            }
        }
        None
    }
}

pub fn exhaustive_positive_finite_primitive_floats<T: PrimitiveFloat>(
) -> ExhaustivePositiveFinitePrimitiveFloats<T>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    ExhaustivePositiveFinitePrimitiveFloats(exhaustive_pairs(
        exhaustive_positive_mantissas::<T>(),
        exhaustive_range_signed(T::MIN_EXPONENT, T::MAX_EXPONENT as i32),
    ))
}

pub struct ExhaustiveNegativeFinitePrimitiveFloats<T: PrimitiveFloat>(
    ExhaustivePositiveFinitePrimitiveFloats<T>,
);

impl<T: PrimitiveFloat> Iterator for ExhaustiveNegativeFinitePrimitiveFloats<T>
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

pub fn exhaustive_negative_finite_primitive_floats<T: PrimitiveFloat>(
) -> ExhaustiveNegativeFinitePrimitiveFloats<T>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    ExhaustiveNegativeFinitePrimitiveFloats(exhaustive_positive_finite_primitive_floats())
}

pub fn exhaustive_nonzero_finite_primitive_floats<T: PrimitiveFloat>() -> Interleave<
    ExhaustivePositiveFinitePrimitiveFloats<T>,
    ExhaustiveNegativeFinitePrimitiveFloats<T>,
>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    exhaustive_positive_finite_primitive_floats()
        .interleave(exhaustive_negative_finite_primitive_floats())
}

pub fn exhaustive_positive_primitive_floats<T: PrimitiveFloat>(
) -> Chain<Once<T>, ExhaustivePositiveFinitePrimitiveFloats<T>>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    once(T::POSITIVE_INFINITY).chain(exhaustive_positive_finite_primitive_floats())
}

pub fn exhaustive_negative_primitive_floats<T: PrimitiveFloat>(
) -> Chain<Once<T>, ExhaustiveNegativeFinitePrimitiveFloats<T>>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    once(T::NEGATIVE_INFINITY).chain(exhaustive_negative_finite_primitive_floats())
}

pub fn exhaustive_nonzero_primitive_floats<T: PrimitiveFloat>() -> Chain<
    std::vec::IntoIter<T>,
    Interleave<
        ExhaustivePositiveFinitePrimitiveFloats<T>,
        ExhaustiveNegativeFinitePrimitiveFloats<T>,
    >,
>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    vec![T::NAN, T::POSITIVE_INFINITY, T::NEGATIVE_INFINITY]
        .into_iter()
        .chain(exhaustive_nonzero_finite_primitive_floats())
}

pub fn exhaustive_finite_primitive_floats<T: PrimitiveFloat>() -> Chain<
    std::vec::IntoIter<T>,
    Interleave<
        ExhaustivePositiveFinitePrimitiveFloats<T>,
        ExhaustiveNegativeFinitePrimitiveFloats<T>,
    >,
>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    vec![T::ZERO, T::NEGATIVE_ZERO]
        .into_iter()
        .chain(exhaustive_nonzero_finite_primitive_floats())
}

pub fn exhaustive_primitive_floats<T: PrimitiveFloat>() -> Chain<
    std::vec::IntoIter<T>,
    Interleave<
        ExhaustivePositiveFinitePrimitiveFloats<T>,
        ExhaustiveNegativeFinitePrimitiveFloats<T>,
    >,
>
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    vec![
        T::NAN,
        T::POSITIVE_INFINITY,
        T::NEGATIVE_INFINITY,
        T::ZERO,
        T::NEGATIVE_ZERO,
    ]
    .into_iter()
    .chain(exhaustive_nonzero_finite_primitive_floats())
}

pub struct RandomFinitePrimitiveFloats<T: PrimitiveFloat>(RandomPrimitiveFloats<T>)
where
    T::UnsignedOfEqualWidth: Rand;

impl<T: PrimitiveFloat> Iterator for RandomFinitePrimitiveFloats<T>
where
    T::UnsignedOfEqualWidth: Rand,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        loop {
            let opt_f = self.0.next();
            if opt_f.unwrap().is_finite() {
                return opt_f;
            }
        }
    }
}

pub fn random_finite_primitive_floats<T: PrimitiveFloat>(
    seed: &[u32],
) -> RandomFinitePrimitiveFloats<T>
where
    T::UnsignedOfEqualWidth: Rand,
{
    RandomFinitePrimitiveFloats(random_primitive_floats(seed))
}

pub struct RandomPrimitiveFloats<T: PrimitiveFloat>(Random<T::UnsignedOfEqualWidth>)
where
    T::UnsignedOfEqualWidth: Rand;

impl<T: PrimitiveFloat> Iterator for RandomPrimitiveFloats<T>
where
    T::UnsignedOfEqualWidth: Rand,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.next().map(T::from_bits)
    }
}

pub fn random_primitive_floats<T: PrimitiveFloat>(seed: &[u32]) -> RandomPrimitiveFloats<T>
where
    T::UnsignedOfEqualWidth: Rand,
{
    RandomPrimitiveFloats(random(seed))
}

macro_rules! special_random_float_gen {
    (
        $f: ident,
        $special_random_positive_mantissas_s: ident,
        $special_random_positive_mantissas_f: ident,
        $special_random_positive_finite_s: ident,
        $special_random_positive_finite_f: ident,
        $special_random_negative_finite_s: ident,
        $special_random_negative_finite_f: ident,
        $special_random_nonzero_finite_s: ident,
        $special_random_nonzero_finite_f: ident,
        $special_random_finite_s: ident,
        $special_random_finite_f: ident,
        $special_random_s: ident,
        $special_random_f: ident
    ) => {
        struct $special_random_positive_mantissas_s {
            range_gen: RandomRange<<$f as PrimitiveFloat>::UnsignedOfEqualWidth>,
            precision_gen: RandomRange<u32>,
        }

        impl Iterator for $special_random_positive_mantissas_s {
            type Item = <$f as PrimitiveFloat>::UnsignedOfEqualWidth;

            fn next(&mut self) -> Option<<$f as PrimitiveFloat>::UnsignedOfEqualWidth> {
                let p = self.precision_gen.next().unwrap();
                self.range_gen.next().map(|m| {
                    let mantissa = (m << 1) + 1;
                    let p = min(p.into(), mantissa.significant_bits() - 1);
                    mantissa >> p << p
                })
            }
        }

        fn $special_random_positive_mantissas_f(
            seed: &[u32],
        ) -> $special_random_positive_mantissas_s {
            $special_random_positive_mantissas_s {
                range_gen: random_range(
                    &scramble(seed, "mantissa"),
                    0,
                    (1 << $f::MANTISSA_WIDTH) - 1,
                ),
                precision_gen: random_range(
                    &scramble(seed, "precision"),
                    0,
                    $f::MANTISSA_WIDTH - 1,
                ),
            }
        }

        pub struct $special_random_positive_finite_s {
            mantissa_gen: $special_random_positive_mantissas_s,
            exponent_gen: I32sGeometric,
        }

        impl Iterator for $special_random_positive_finite_s {
            type Item = $f;

            fn next(&mut self) -> Option<$f> {
                let mantissa = self.mantissa_gen.next().unwrap();
                loop {
                    let exponent = self.exponent_gen.next().unwrap();
                    if exponent >= $f::MIN_EXPONENT
                        && exponent <= i32::checked_from($f::MAX_EXPONENT).unwrap()
                    {
                        let f = from_mantissa_and_exponent(
                            <$f as PrimitiveFloat>::SignedOfEqualWidth::from_unsigned_bitwise(
                                mantissa,
                            ),
                            exponent,
                        );
                        if f.is_some() {
                            return f;
                        }
                    }
                }
            }
        }

        pub fn $special_random_positive_finite_f(
            seed: &[u32],
            scale: u32,
        ) -> $special_random_positive_finite_s {
            $special_random_positive_finite_s {
                mantissa_gen: $special_random_positive_mantissas_f(&scramble(seed, "mantissa")),
                exponent_gen: i32s_geometric(&scramble(seed, "exponent"), scale),
            }
        }

        pub struct $special_random_negative_finite_s($special_random_positive_finite_s);

        impl Iterator for $special_random_negative_finite_s {
            type Item = $f;

            fn next(&mut self) -> Option<$f> {
                self.0.next().map(|f| -f)
            }
        }

        pub fn $special_random_negative_finite_f(
            seed: &[u32],
            scale: u32,
        ) -> $special_random_negative_finite_s {
            $special_random_negative_finite_s($special_random_positive_finite_f(seed, scale))
        }

        pub struct $special_random_nonzero_finite_s {
            sign_gen: Random<bool>,
            abs_gen: $special_random_positive_finite_s,
        }

        impl Iterator for $special_random_nonzero_finite_s {
            type Item = $f;

            fn next(&mut self) -> Option<$f> {
                let abs = self.abs_gen.next();
                if self.sign_gen.next().unwrap() {
                    abs
                } else {
                    abs.map(|f| -f)
                }
            }
        }

        pub fn $special_random_nonzero_finite_f(
            seed: &[u32],
            scale: u32,
        ) -> $special_random_nonzero_finite_s {
            $special_random_nonzero_finite_s {
                sign_gen: random(&scramble(seed, "sign")),
                abs_gen: $special_random_positive_finite_f(&scramble(seed, "abs"), scale),
            }
        }

        pub struct $special_random_finite_s {
            scale: u32,
            switch_gen: Box<IsaacRng>,
            special_gen: RandomFromVector<$f>,
            finite_gen: $special_random_nonzero_finite_s,
        }

        impl Iterator for $special_random_finite_s {
            type Item = $f;

            fn next(&mut self) -> Option<$f> {
                if self.switch_gen.gen_weighted_bool(self.scale) {
                    self.special_gen.next()
                } else {
                    self.finite_gen.next()
                }
            }
        }

        pub fn $special_random_finite_f(seed: &[u32], scale: u32) -> $special_random_finite_s {
            $special_random_finite_s {
                scale: scale + 2,
                switch_gen: Box::new(IsaacRng::from_seed(&scramble(seed, "switch"))),
                special_gen: random_from_vector(
                    &scramble(seed, "special"),
                    vec![$f::ZERO, $f::NEGATIVE_ZERO],
                ),
                finite_gen: $special_random_nonzero_finite_f(&scramble(seed, "finite"), scale),
            }
        }

        pub struct $special_random_s {
            scale: u32,
            switch_gen: Box<IsaacRng>,
            special_gen: RandomFromVector<$f>,
            finite_gen: $special_random_nonzero_finite_s,
        }

        impl Iterator for $special_random_s {
            type Item = $f;

            fn next(&mut self) -> Option<$f> {
                if self.switch_gen.gen_weighted_bool(self.scale) {
                    self.special_gen.next()
                } else {
                    self.finite_gen.next()
                }
            }
        }

        pub fn $special_random_f(seed: &[u32], scale: u32) -> $special_random_s {
            $special_random_s {
                scale: scale + 2,
                switch_gen: Box::new(IsaacRng::from_seed(&scramble(seed, "switch"))),
                special_gen: random_from_vector(
                    &scramble(seed, "special"),
                    vec![
                        $f::NAN,
                        $f::POSITIVE_INFINITY,
                        $f::NEGATIVE_INFINITY,
                        $f::ZERO,
                        $f::NEGATIVE_ZERO,
                    ],
                ),
                finite_gen: $special_random_nonzero_finite_f(&scramble(seed, "finite"), scale),
            }
        }
    };
}

special_random_float_gen!(
    f32,
    SpecialRandomPositiveMantissasF32,
    special_random_positive_mantissas_f32,
    SpecialRandomPositiveFiniteF32s,
    special_random_positive_finite_f32s,
    SpecialRandomNegativeFiniteF32s,
    special_random_negative_finite_f32s,
    SpecialRandomNonzeroFiniteF32s,
    special_random_nonzero_finite_f32s,
    SpecialRandomFiniteF32s,
    special_random_finite_f32s,
    SpecialRandomF32s,
    special_random_f32s
);

special_random_float_gen!(
    f64,
    SpecialRandomPositiveMantissasF64,
    special_random_positive_mantissas_f64,
    SpecialRandomPositiveFiniteF64s,
    special_random_positive_finite_f64s,
    SpecialRandomNegativeFiniteF64s,
    special_random_negative_finite_f64s,
    SpecialRandomNonzeroFiniteF64s,
    special_random_nonzero_finite_f64s,
    SpecialRandomFiniteF64s,
    special_random_finite_f64s,
    SpecialRandomF64s,
    special_random_f64s
);
