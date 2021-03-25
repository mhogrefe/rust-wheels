use iterators::common::scramble;
use iterators::general::{random, random_from_vector, Random, RandomFromVector};
use iterators::integers_geometric::{i32s_geometric, I32sGeometric};
use iterators::primitive_ints::{random_range, RandomRange};
use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOfTwo;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::num::logic::traits::{LowMask, SignificantBits};
use malachite_base::rounding_modes::RoundingMode;
use rand::{IsaacRng, Rand, Rng, SeedableRng};
use std::cmp::min;

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
        $u: ident,
        $special_random_positive_mantissas_s: ident,
        $special_random_positive_mantissas_f: ident,
        $special_random_positive_finite_s: ident,
        $special_random_positive_finite_f: ident,
        $special_random_nonzero_finite_s: ident,
        $special_random_nonzero_finite_f: ident,
        $special_random_finite_s: ident,
        $special_random_finite_f: ident,
        $special_random_s: ident,
        $special_random_f: ident
    ) => {
        struct $special_random_positive_mantissas_s {
            range_gen: RandomRange<$u>,
            precision_gen: RandomRange<u32>,
        }

        impl Iterator for $special_random_positive_mantissas_s {
            type Item = $u;

            fn next(&mut self) -> Option<$u> {
                let p = self.precision_gen.next().unwrap();
                self.range_gen.next().map(|m| {
                    let mantissa = (m << 1) + 1;
                    let p = min(u64::from(p), mantissa.significant_bits() - 1);
                    mantissa.round_to_multiple_of_power_of_two(p, RoundingMode::Ceiling)
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
                    $u::low_mask($f::MANTISSA_WIDTH),
                ),
                precision_gen: random_range(
                    &scramble(seed, "precision"),
                    0,
                    u32::exact_from($f::MANTISSA_WIDTH) - 1,
                ),
            }
        }

        struct $special_random_positive_finite_s {
            mantissa_gen: $special_random_positive_mantissas_s,
            exponent_gen: I32sGeometric,
        }

        impl Iterator for $special_random_positive_finite_s {
            type Item = $f;

            fn next(&mut self) -> Option<$f> {
                let mantissa = self.mantissa_gen.next().unwrap();
                loop {
                    let exponent = self.exponent_gen.next().unwrap();
                    if exponent >= i32::wrapping_from($f::MIN_EXPONENT)
                        && exponent <= i32::exact_from($f::MAX_EXPONENT)
                    {
                        let f = $f::from_adjusted_mantissa_and_exponent(
                            $u::wrapping_from(mantissa),
                            i64::exact_from(exponent),
                        );
                        if f.is_some() {
                            return f;
                        }
                    }
                }
            }
        }

        fn $special_random_positive_finite_f(
            seed: &[u32],
            scale: u32,
        ) -> $special_random_positive_finite_s {
            $special_random_positive_finite_s {
                mantissa_gen: $special_random_positive_mantissas_f(&scramble(seed, "mantissa")),
                exponent_gen: i32s_geometric(&scramble(seed, "exponent"), scale),
            }
        }

        struct $special_random_nonzero_finite_s {
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

        fn $special_random_nonzero_finite_f(
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
    u32,
    SpecialRandomPositiveMantissasF32,
    special_random_positive_mantissas_f32,
    SpecialRandomPositiveFiniteF32s,
    special_random_positive_finite_f32s,
    SpecialRandomNonzeroFiniteF32s,
    special_random_nonzero_finite_f32s,
    SpecialRandomFiniteF32s,
    special_random_finite_f32s,
    SpecialRandomF32s,
    special_random_f32s
);

special_random_float_gen!(
    f64,
    u64,
    SpecialRandomPositiveMantissasF64,
    special_random_positive_mantissas_f64,
    SpecialRandomPositiveFiniteF64s,
    special_random_positive_finite_f64s,
    SpecialRandomNonzeroFiniteF64s,
    special_random_nonzero_finite_f64s,
    SpecialRandomFiniteF64s,
    special_random_finite_f64s,
    SpecialRandomF64s,
    special_random_f64s
);
