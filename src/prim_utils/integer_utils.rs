#[cfg(feature = "gmp")]
use gmp_to_flint_adaptor_lib::integer::Integer;
#[cfg(feature = "native")]
use num_to_flint_adaptor_lib::integer::Integer;
use prim_utils::traits::*;
use std::cmp::Ordering;
use std::iter;
use std::mem;

macro_rules! prim_impls {
    ($t: ident, $name: expr, $bit_count: expr) => {
        impl PrimInt for $t {
            fn name() -> &'static str {
                $name
            }

            fn bit_count() -> u32 {
                $bit_count
            }

            fn min_value() -> $t {
                $t::min_value()
            }

            fn max_value() -> $t {
                $t::max_value()
            }

            fn from_u8(i: u8) -> $t {
                i as $t
            }

            fn leading_zeros(&self) -> u32 {
                $t::leading_zeros(*self)
            }
        }

        impl Walkable for $t {
            fn increment(&mut self) {
                *self += 1
            }

            fn decrement(&mut self) {
                *self -= 1
            }
        }
    }
}

prim_impls!(u8, "u8", 8);
prim_impls!(u16, "u16", 16);
prim_impls!(u32, "u32", 32);
prim_impls!(u64, "u64", 64);
prim_impls!(usize, "usize", (0 as usize).count_zeros());
prim_impls!(i8, "i8", 8);
prim_impls!(i16, "i16", 16);
prim_impls!(i32, "i32", 32);
prim_impls!(i64, "i64", 64);
prim_impls!(isize, "isize", (0 as isize).count_zeros());

impl PrimUnsignedInt for u8 {}
impl PrimUnsignedInt for u16 {}
impl PrimUnsignedInt for u32 {}
impl PrimUnsignedInt for u64 {}
impl PrimUnsignedInt for usize {}

macro_rules! prim_impls_i {
    ($t: ident) => {
        impl PrimSignedInt for $t {
            fn from_i8(i: i8) -> $t {
                i as $t
            }
        }
    }
}

prim_impls_i!(i8);
prim_impls_i!(i16);
prim_impls_i!(i32);
prim_impls_i!(i64);
prim_impls_i!(isize);

pub fn is_power_of_two(n: &Integer) -> bool {
    if n.sign() != Ordering::Greater {
        panic!("n must be positive. Invalid n: {}", n);
    }
    n.find_one(0).unwrap() == n.significant_bits() - 1
}

pub fn ceiling_log_2_u<T: PrimUnsignedInt>(n: T) -> u32 {
    let zero = T::from_u8(0);
    let one = T::from_u8(1);
    if n < one {
        panic!("n must be positive. Invalid n: {}", n);
    }
    let bit_length = T::bit_count() - n.leading_zeros();
    if n & (n - one) == zero {
        bit_length - 1
    } else {
        bit_length
    }
}

pub fn ceiling_log_2_integer(n: &Integer) -> u32 {
    if n.sign() != Ordering::Greater {
        panic!("n must be positive. Invalid n: {}", n);
    }
    let bit_length = n.significant_bits();
    if n.count_ones().unwrap() == 1 {
        bit_length - 1
    } else {
        bit_length
    }
}

pub fn bits_u<T: PrimUnsignedInt>(n: T) -> Vec<bool> {
    let zero = T::from_u8(0);
    let one = T::from_u8(1);
    let mut bits = Vec::new();
    let mut remaining = n;
    while remaining != zero {
        bits.push(remaining & one != zero);
        remaining >>= 1;
    }
    bits
}

pub fn bits_integer(n: &Integer) -> Vec<bool> {
    match n.sign() {
        Ordering::Less => panic!("n cannot be negative. Invalid n: {}", n),
        Ordering::Equal => Vec::new(),
        Ordering::Greater => {
            let bit_length = n.significant_bits();
            let mut bits = Vec::with_capacity(bit_length as usize);
            for i in 0..bit_length {
                bits.push(n.get_bit(i));
            }
            bits
        }
    }
}

pub fn bits_padded_u<T: PrimUnsignedInt>(size: usize, n: T) -> Vec<bool> {
    let zero = T::from_u8(0);
    let one = T::from_u8(1);
    let mut bits = Vec::with_capacity(size);
    let mut remaining = n;
    for _ in 0..size {
        bits.push(remaining & one != zero);
        remaining >>= 1;
    }
    bits
}

pub fn bits_padded_integer(size: usize, n: &Integer) -> Vec<bool> {
    if n.sign() == Ordering::Less {
        panic!("n cannot be negative. Invalid n: {}", n);
    }
    let mut bits = Vec::with_capacity(size);
    for i in 0..(size as u32) {
        bits.push(n.get_bit(i));
    }
    bits
}

pub fn big_endian_bits_u<T: PrimUnsignedInt>(n: T) -> Vec<bool> {
    let zero = T::from_u8(0);
    let one = T::from_u8(1);
    let mut bits = Vec::new();
    if n == zero {
        return bits;
    }
    let mut mask: T = one << (T::bit_count() - n.leading_zeros() - 1);
    while mask != zero {
        bits.push(n & mask != zero);
        mask >>= 1;
    }
    bits
}

pub fn big_endian_bits_integer(n: &Integer) -> Vec<bool> {
    match n.sign() {
        Ordering::Less => panic!("n cannot be negative. Invalid n: {}", n),
        Ordering::Equal => Vec::new(),
        Ordering::Greater => {
            let bit_length = n.significant_bits();
            let mut bits = Vec::with_capacity(bit_length as usize);
            let mut i = bit_length - 1;
            loop {
                bits.push(n.get_bit(i));
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            }
            bits
        }
    }
}

pub fn big_endian_bits_padded_u<T: PrimUnsignedInt>(size: usize, n: T) -> Vec<bool> {
    let mut bits = Vec::new();
    if size == 0 {
        return bits;
    }
    let mut size = size;
    let max_bits = T::bit_count();
    while (size as u32) > max_bits {
        bits.push(false);
        size -= 1;
    }
    let zero = T::from_u8(0);
    let one = T::from_u8(1);
    let mut mask = one << (size as u32 - 1);
    while mask != zero {
        bits.push(n & mask != zero);
        mask >>= 1;
    }
    bits
}

pub fn big_endian_bits_padded_integer(size: usize, n: &Integer) -> Vec<bool> {
    if n.sign() == Ordering::Less {
        panic!("n cannot be negative. Invalid n: {}", n);
    }
    let mut bits = Vec::new();
    if size == 0 {
        return bits;
    }
    let mut i = size - 1;
    loop {
        bits.push(n.get_bit(i as u32));
        if i == 0 {
            break;
        } else {
            i -= 1;
        }
    }
    bits
}

pub fn from_big_endian_bits(bits: &[bool]) -> Integer {
    let mut bits = bits.to_vec();
    bits.reverse();
    let mut result = Integer::new();
    result.assign_bits_unsigned(&bits[..]);
    result
}

pub fn digits_u<T: PrimUnsignedInt>(radix: T, n: T) -> Vec<T> {
    if radix < T::from_u8(2) {
        panic!("radix must be at least 2. Invalid radix: {}", radix);
    }
    let zero = T::from_u8(0);
    let one = T::from_u8(1);
    let mut digits = Vec::new();
    let log = ceiling_log_2_u(radix);
    let mut remaining = n;
    if one << log == radix {
        let mask = radix - one;
        while remaining != zero {
            digits.push(remaining & mask);
            remaining >>= log;
        }
    } else {
        while remaining != zero {
            digits.push(remaining % radix);
            remaining /= radix;
        }
    }
    digits
}

pub fn digits_integer(radix: &Integer, n: &Integer) -> Vec<Integer> {
    if *radix < 2 {
        panic!("radix must be at least 2. Invalid radix: {}", radix);
    }
    let sign = n.sign();
    if sign == Ordering::Less {
        panic!("n cannot be negative. Invalid n: {}", n);
    } else if sign == Ordering::Equal {
        return Vec::new();
    } else if is_power_of_two(radix) {
        let log = ceiling_log_2_integer(radix);
        let mut digits = Vec::new();
        let length = n.significant_bits();
        let mut digit = Integer::from(0);
        let mut i = 0;
        let mut j = 0;
        let mut mask = 1;
        for x in n.to_u32s() {
            loop {
                if x & mask != 0 {
                    digit.set_bit(j, true);
                }
                i += 1;
                if i == length {
                    break;
                }
                j += 1;
                if j == log {
                    let last_index = digits.len();
                    digits.push(Integer::from(0));
                    mem::swap(&mut digits[last_index], &mut digit);
                    j = 0;
                }
                if mask == 0 {
                    break;
                }
                mask <<= 1;
            }
        }
        if digit != 0 {
            let last_index = digits.len();
            digits.push(Integer::from(0));
            mem::swap(&mut digits[last_index], &mut digit);
        }
        return digits;
    } else if *radix <= 36 {
        return n.to_string_radix(radix.to_i32().unwrap())
                   .chars()
                   .rev()
                   .map(|c| {
                            Integer::from(c as u32 -
                                          (if c >= '0' && c <= '9' { '0' } else { 'W' } as u32))
                        })
                   .collect();
    } else {
        let mut digits = Vec::new();
        let mut remaining = n.clone();
        while remaining != 0 {
            let last_index = digits.len();
            digits.push(radix.clone());
            remaining.div_rem(&mut digits[last_index]);
        }
        return digits;
    }
}

pub fn digits_padded_u<T: PrimUnsignedInt>(size: usize, radix: T, n: T) -> Vec<T> {
    if radix < T::from_u8(2) {
        panic!("radix must be at least 2. Invalid radix: {}", radix);
    }
    let one = T::from_u8(1);
    let mut digits = Vec::new();
    let log = ceiling_log_2_u(radix);
    let mut remaining = n;
    if one << log == radix {
        let mask = radix - one;
        for _ in 0..size {
            digits.push(remaining & mask);
            remaining >>= log;
        }
    } else {
        for _ in 0..size {
            digits.push(remaining % radix);
            remaining /= radix;
        }
    }
    digits
}

pub fn digits_padded_integer(size: usize, radix: &Integer, n: &Integer) -> Vec<Integer> {
    if *radix < 2 {
        panic!("radix must be at least 2. Invalid radix: {}", radix);
    }
    let sign = n.sign();
    if sign == Ordering::Less {
        panic!("n cannot be negative. Invalid n: {}", n);
    } else if size == 0 {
        return Vec::new();
    } else if sign == Ordering::Equal {
        return iter::repeat(Integer::from(0)).take(size).collect();
    } else if is_power_of_two(radix) {
        let log = ceiling_log_2_integer(radix);
        let mut digits = Vec::new();
        let mut digit = Integer::from(0);
        let mut i = 0;
        let mut j = 0;
        let mut mask = 1;
        for x in n.to_u32s() {
            loop {
                if i == size {
                    break;
                }
                if x & mask != 0 {
                    digit.set_bit(j, true);
                }
                i += 1;
                j += 1;
                if j == log {
                    let last_index = digits.len();
                    digits.push(Integer::from(0));
                    mem::swap(&mut digits[last_index], &mut digit);
                    j = 0;
                }
                if mask == 0 {
                    break;
                }
                mask <<= 1;
            }
        }
        if digit != 0 {
            let last_index = digits.len();
            digits.push(Integer::from(0));
            mem::swap(&mut digits[last_index], &mut digit);
        }
        return digits;
    } else if *radix <= 36 {
        return n.to_string_radix(radix.to_i32().unwrap())
                   .chars()
                   .rev()
                   .map(|c| {
                            Integer::from(c as u32 -
                                          (if c >= '0' && c <= '9' { '0' } else { 'W' } as u32))
                        })
                   .chain(iter::repeat(Integer::from(0)))
                   .take(size)
                   .collect();
    } else {
        let mut digits = Vec::new();
        let mut remaining = n.clone();
        for i in 0..size {
            if remaining == 0 {
                digits.push(Integer::from(0));
            } else {
                digits.push(radix.clone());
                remaining.div_rem(&mut digits[i]);
            }
        }
        return digits;
    }
}
