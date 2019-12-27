use std::cmp::Ordering;
use std::ops::{Add, Neg, Shl, Shr, Sub};

use malachite_base::num::arithmetic::traits::{NegAssign, Parity, Sign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, ExactInto, WrappingFrom};
use malachite_base::num::floats::PrimitiveFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;

#[derive(Debug, Eq, PartialEq)]
pub struct BinaryFraction {
    mantissa: Integer,
    exponent: i32,
}

impl Zero for BinaryFraction {
    const ZERO: BinaryFraction = BinaryFraction {
        mantissa: Integer::ZERO,
        exponent: 0,
    };
}

impl One for BinaryFraction {
    const ONE: BinaryFraction = BinaryFraction {
        mantissa: Integer::ONE,
        exponent: 0,
    };
}

impl Neg for BinaryFraction {
    type Output = BinaryFraction;

    fn neg(self) -> BinaryFraction {
        BinaryFraction {
            mantissa: -self.mantissa,
            exponent: self.exponent,
        }
    }
}

impl<'a> Neg for &'a BinaryFraction {
    type Output = BinaryFraction;

    fn neg(self) -> BinaryFraction {
        BinaryFraction {
            mantissa: -&self.mantissa,
            exponent: self.exponent,
        }
    }
}

impl PartialOrd for BinaryFraction {
    fn partial_cmp(&self, other: &BinaryFraction) -> Option<Ordering> {
        let sign_cmp = self.sign().cmp(&other.sign());
        Some(if sign_cmp != Ordering::Equal {
            sign_cmp
        } else {
            match self.exponent.cmp(&other.exponent) {
                Ordering::Equal => self.mantissa.cmp(&other.mantissa),
                Ordering::Less => self
                    .mantissa
                    .cmp(&(&other.mantissa << (other.exponent - self.exponent))),
                Ordering::Greater => {
                    (&self.mantissa << (self.exponent - other.exponent)).cmp(&other.mantissa)
                }
            }
        })
    }
}

impl Shl<i32> for BinaryFraction {
    type Output = BinaryFraction;

    fn shl(self, rhs: i32) -> BinaryFraction {
        BinaryFraction {
            mantissa: self.mantissa,
            exponent: self.exponent + rhs,
        }
    }
}

impl<'a> Shl<i32> for &'a BinaryFraction {
    type Output = BinaryFraction;

    fn shl(self, rhs: i32) -> BinaryFraction {
        BinaryFraction {
            mantissa: self.mantissa.clone(),
            exponent: self.exponent + rhs,
        }
    }
}

impl Shr<i32> for BinaryFraction {
    type Output = BinaryFraction;

    fn shr(self, rhs: i32) -> BinaryFraction {
        BinaryFraction {
            mantissa: self.mantissa,
            exponent: self.exponent - rhs,
        }
    }
}

impl<'a> Shr<i32> for &'a BinaryFraction {
    type Output = BinaryFraction;

    fn shr(self, rhs: i32) -> BinaryFraction {
        BinaryFraction {
            mantissa: self.mantissa.clone(),
            exponent: self.exponent - rhs,
        }
    }
}

impl Add<BinaryFraction> for BinaryFraction {
    type Output = BinaryFraction;

    fn add(self, rhs: BinaryFraction) -> BinaryFraction {
        if self == BinaryFraction::ZERO {
            rhs
        } else if rhs == BinaryFraction::ZERO {
            self
        } else {
            match self.exponent.cmp(&rhs.exponent) {
                Ordering::Equal => BinaryFraction::new(self.mantissa + rhs.mantissa, self.exponent),
                Ordering::Less => BinaryFraction::new(
                    (rhs.mantissa << (rhs.exponent - self.exponent)) + self.mantissa,
                    self.exponent,
                ),
                Ordering::Greater => BinaryFraction::new(
                    (self.mantissa << (self.exponent - rhs.exponent)) + rhs.mantissa,
                    rhs.exponent,
                ),
            }
        }
    }
}

impl Sub<BinaryFraction> for BinaryFraction {
    type Output = BinaryFraction;

    fn sub(self, rhs: BinaryFraction) -> BinaryFraction {
        if self == BinaryFraction::ZERO {
            -rhs
        } else if rhs == BinaryFraction::ZERO {
            self
        } else {
            match self.exponent.cmp(&rhs.exponent) {
                Ordering::Equal => BinaryFraction::new(self.mantissa - rhs.mantissa, self.exponent),
                Ordering::Less => BinaryFraction::new(
                    self.mantissa - (rhs.mantissa << (rhs.exponent - self.exponent)),
                    self.exponent,
                ),
                Ordering::Greater => BinaryFraction::new(
                    (self.mantissa << (self.exponent - rhs.exponent)) - rhs.mantissa,
                    rhs.exponent,
                ),
            }
        }
    }
}

macro_rules! binary_fraction_funs {
    (
        $f: ident,
        $s: ident,
        $from_float: ident,
        $largest_finite_float: ident,
        $to_float: ident
    ) => {
        pub fn $from_float(mut f: $f) -> Option<BinaryFraction>
        {
            if f == 0.0 {
                Some(BinaryFraction::ZERO)
            } else if f == 1.0 {
                Some(BinaryFraction::ONE)
            } else if !f.is_finite() {
                None
            } else {
                let positive = f.is_sign_positive();
                if !positive {
                    f.neg_assign();
                }
                let (mut mantissa, offset_exponent) = f.to_adjusted_mantissa_and_exponent();
                let mut exponent = i32::wrapping_from(offset_exponent);
                if exponent == 0 {
                    exponent = $f::MIN_EXPONENT;
                } else {
                    mantissa += 1 << $f::MANTISSA_WIDTH;
                    exponent += $f::MIN_EXPONENT - 1;
                }
                let mantissa = $s::exact_from(mantissa);
                let signed_mantissa = if positive { mantissa } else { -mantissa };
                Some(BinaryFraction::new(
                    Integer::from(signed_mantissa),
                    exponent,
                ))
            }
        }

        pub fn $largest_finite_float() -> BinaryFraction
        {
            BinaryFraction::$from_float($f::MAX_FINITE).unwrap()
        }

        pub fn $to_float(&self) -> Option<$f>
        {
            if *self == BinaryFraction::ZERO {
                return Some(0.0);
            }
            if self.mantissa < 0 as Limb {
                return (-self).$to_float().map(|f| -f);
            }
            let fp_exponent = i32::exact_from(self.mantissa.significant_bits())
                + self.exponent
                - 1;
            let signed_max_exponent = i32::exact_from($f::MAX_EXPONENT);
            if fp_exponent > signed_max_exponent
                || fp_exponent == signed_max_exponent
                    && *self > BinaryFraction::$largest_finite_float()
            {
                return None;
            }
            let (adjusted_mantissa, adjusted_exponent) = if fp_exponent < $f::MIN_NORMAL_EXPONENT {
                (self >> $f::MIN_EXPONENT, 0)
            } else {
                (
                    ((self >> fp_exponent) - BinaryFraction::ONE) <<
                        i32::wrapping_from($f::MANTISSA_WIDTH),
                    fp_exponent + i32::wrapping_from($f::MAX_EXPONENT),
                )
            };
            adjusted_mantissa.into_integer().map(|i| {
                $f::from_adjusted_mantissa_and_exponent(
                    (&i).exact_into(),
                    u32::exact_from(adjusted_exponent),
                )
            })
        }
    }
}

impl BinaryFraction {
    pub fn new(mantissa: Integer, exponent: i32) -> BinaryFraction {
        if let Some(trailing_zeros) = mantissa.trailing_zeros() {
            let trailing_zeros = i32::wrapping_from(trailing_zeros);
            BinaryFraction {
                mantissa: mantissa >> trailing_zeros,
                exponent: exponent + trailing_zeros,
            }
        } else {
            BinaryFraction::ZERO
        }
    }

    fn sign(&self) -> Ordering {
        self.mantissa.sign()
    }

    fn into_integer(self) -> Option<Integer> {
        if self.exponent >= 0 {
            Some(self.mantissa << self.exponent)
        } else {
            None
        }
    }

    binary_fraction_funs!(f32, i32, from_f32, largest_finite_f32, to_f32);
    binary_fraction_funs!(f64, i64, from_f64, largest_finite_f64, to_f64);
}

macro_rules! additional_funs {
    (
        $f: ident,
        $s: ident,
        $checked_from_mantissa_and_exponent: ident,
        $from_mantissa_and_exponent: ident,
        $to_float: ident
    ) => {
        pub fn $checked_from_mantissa_and_exponent(mantissa: $s, exponent: i32) -> Option<$f> {
            if mantissa == 0 && exponent != 0 || mantissa.even() {
                None
            } else {
                $from_mantissa_and_exponent(mantissa, exponent)
            }
        }

        pub fn $from_mantissa_and_exponent(mantissa: $s, exponent: i32) -> Option<$f> {
            BinaryFraction::new(Integer::from(mantissa), exponent).$to_float()
        }
    };
}

additional_funs!(
    f32,
    i32,
    f32_checked_from_mantissa_and_exponent,
    f32_from_mantissa_and_exponent,
    to_f32
);
additional_funs!(
    f64,
    i64,
    f64_checked_from_mantissa_and_exponent,
    f64_from_mantissa_and_exponent,
    to_f64
);
