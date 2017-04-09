use gmp_to_flint_adaptor_lib::integer::Integer;
use rust_wheels_lib::io::readers::parse_vec;
use rust_wheels_lib::number_utils::integer_utils::*;
use std::str::FromStr;

fn is_power_of_two_helper(n: &str, out: bool) {
    assert_eq!(is_power_of_two(&Integer::from_str(n).unwrap()), out);
}

#[test]
fn test_is_power_of_two() {
    is_power_of_two_helper("1", true);
    is_power_of_two_helper("2", true);
    is_power_of_two_helper("4", true);
    is_power_of_two_helper("8", true);
    is_power_of_two_helper("16", true);
    is_power_of_two_helper("3", false);
    is_power_of_two_helper("13", false);
}

fn is_power_of_two_fail_helper(n: &str) {
    is_power_of_two(&Integer::from_str(n).unwrap());
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: 0")]
fn is_power_of_two_fail_1() {
    is_power_of_two_fail_helper("0");
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: -5")]
fn is_power_of_two_fail_2() {
    is_power_of_two_fail_helper("-5");
}

macro_rules! test_ceiling_log_2 {
    ($t: ty, $f: ident, $test: ident, $helper: ident, $fail: ident, $len: expr, $max: expr) => {
        fn $helper(n: $t, out: u32) {
            assert_eq!($f(n), out);
        }

        #[test]
        fn $test() {
            $helper(1, 0);
            $helper(2, 1);
            $helper(3, 2);
            $helper(4, 2);
            $helper(5, 3);
            $helper(6, 3);
            $helper(7, 3);
            $helper(8, 3);
            $helper(9, 4);
            $helper(100, 7);
            $helper($max, $len);
        }

        #[test]
        #[should_panic(expected = "n must be positive. Invalid n: 0")]
        fn $fail() {
            $f(0);
        }
    }
}

test_ceiling_log_2!(u8,
                    ceiling_log_2_u8,
                    test_ceiling_log_2_u8,
                    ceiling_log_2_u8_helper,
                    test_is_ceiling_log_2_u8_fail,
                    8,
                    u8::max_value());
test_ceiling_log_2!(u16,
                    ceiling_log_2_u16,
                    test_ceiling_log_2_u16,
                    ceiling_log_2_u16_helper,
                    test_is_ceiling_log_2_u16_fail,
                    16,
                    u16::max_value());
test_ceiling_log_2!(u32,
                    ceiling_log_2_u32,
                    test_ceiling_log_2_u32,
                    ceiling_log_2_u32_helper,
                    test_is_ceiling_log_2_u32_fail,
                    32,
                    u32::max_value());
test_ceiling_log_2!(u64,
                    ceiling_log_2_u64,
                    test_ceiling_log_2_u64,
                    ceiling_log_2_u64_helper,
                    test_is_ceiling_log_2_u64_fail,
                    64,
                    u64::max_value());
test_ceiling_log_2!(usize,
                    ceiling_log_2_usize,
                    test_ceiling_log_2_usize,
                    ceiling_log_2_usize_helper,
                    test_is_ceiling_log_2_usize_fail,
                    usize_bit_count(),
                    usize::max_value());

fn ceiling_log_2_integer_helper(n: &str, out: u32) {
    assert_eq!(ceiling_log_2_integer(&Integer::from_str(n).unwrap()), out);
}

#[test]
fn test_ceiling_log_2_integer() {
    ceiling_log_2_integer_helper("1", 0);
    ceiling_log_2_integer_helper("2", 1);
    ceiling_log_2_integer_helper("3", 2);
    ceiling_log_2_integer_helper("4", 2);
    ceiling_log_2_integer_helper("5", 3);
    ceiling_log_2_integer_helper("6", 3);
    ceiling_log_2_integer_helper("7", 3);
    ceiling_log_2_integer_helper("8", 3);
    ceiling_log_2_integer_helper("9", 4);
    ceiling_log_2_integer_helper("100", 7);
}

fn ceiling_log_2_integer_fail_helper(n: &str) {
    ceiling_log_2_integer(&Integer::from_str(n).unwrap());
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: 0")]
fn ceiling_log_2_integer_fail_1() {
    ceiling_log_2_integer_fail_helper("0");
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: -5")]
fn ceiling_log_2_integer_fail_2() {
    ceiling_log_2_integer_fail_helper("-5");
}

macro_rules! test_bits {
    ($t: ty, $f: ident, $test: ident, $helper: ident, $max: expr, $max_bits: expr) => {
        fn $helper(n: $t, out: &str) {
            assert_eq!(format!("{:?}", $f(n)), out);
        }

        #[test]
        fn $test() {
            $helper(0, "[]");
            $helper(1, "[true]");
            $helper(6, "[false, true, true]");
            $helper(105, "[true, false, false, true, false, true, true]");
            $helper($max, $max_bits);
        }
    }
}

test_bits!(u8,
           bits_u8,
           test_bits_u8,
           bits_u8_helper,
           u8::max_value(),
           "[true, true, true, true, true, true, true, true]");
test_bits!(u16,
           bits_u16,
           test_bits_u16,
           bits_u16_helper,
           u16::max_value(),
           "[true, true, true, true, true, true, true, true, true, true, true, true, true, true, \
             true, true]");
test_bits!(u32,
           bits_u32,
           test_bits_u32,
           bits_u32_helper,
           u32::max_value(),
           "[true, true, true, true, true, true, true, true, true, true, true, true, true, true, \
             true, true, true, true, true, true, true, true, true, true, true, true, true, true, \
             true, true, true, true]");
test_bits!(u64,
           bits_u64,
           test_bits_u64,
           bits_u64_helper,
           u64::max_value(),
           "[true, true, true, true, true, true, true, true, true, true, true, true, true, true, \
             true, true, true, true, true, true, true, true, true, true, true, true, true, true, \
             true, true, true, true, true, true, true, true, true, true, true, true, true, true, \
             true, true, true, true, true, true, true, true, true, true, true, true, true, true, \
             true, true, true, true, true, true, true, true]");
test_bits!(i8,
           bits_i8,
           test_bits_i8,
           bits_i8_helper,
           i8::max_value(),
           "[true, true, true, true, true, true, true]");
test_bits!(i16,
           bits_i16,
           test_bits_i16,
           bits_i16_helper,
           i16::max_value(),
           "[true, true, true, true, true, true, true, true, true, true, true, true, true, true, \
             true]");
test_bits!(i32,
           bits_i32,
           test_bits_i32,
           bits_i32_helper,
           i32::max_value(),
           "[true, true, true, true, true, true, true, true, true, true, true, true, true, true, \
             true, true, true, true, true, true, true, true, true, true, true, true, true, true, \
             true, true, true]");
test_bits!(i64,
           bits_i64,
           test_bits_i64,
           bits_i64_helper,
           i64::max_value(),
           "[true, true, true, true, true, true, true, true, true, true, true, true, true, true, \
             true, true, true, true, true, true, true, true, true, true, true, true, true, true, \
             true, true, true, true, true, true, true, true, true, true, true, true, true, true, \
             true, true, true, true, true, true, true, true, true, true, true, true, true, true, \
             true, true, true, true, true, true, true]");

macro_rules! test_bits_i {
    ($t: ty, $f: ident, $fail: ident) => {
        #[test]
        #[should_panic(expected = "n cannot be negative. Invalid n: -5")]
        fn $fail() {
            $f(-5);
        }
    }
}

test_bits_i!(i8, bits_i8, bits_i8_fail);
test_bits_i!(i16, bits_i16, bits_i16_fail);
test_bits_i!(i32, bits_i32, bits_i32_fail);
test_bits_i!(i64, bits_i64, bits_i64_fail);
test_bits_i!(isize, bits_isize, bits_isize_fail);

fn bits_integer_helper(n: &str, out: &str) {
    assert_eq!(format!("{:?}", bits_integer(&Integer::from_str(n).unwrap())),
               out);
}

#[test]
fn test_bits_integer() {
    bits_integer_helper("0", "[]");
    bits_integer_helper("1", "[true]");
    bits_integer_helper("6", "[false, true, true]");
    bits_integer_helper("105", "[true, false, false, true, false, true, true]");
}

#[test]
#[should_panic(expected = "n cannot be negative. Invalid n: -5")]
fn bits_integer_fail() {
    bits_integer(&Integer::from(-5));
}

macro_rules! test_bits_padded_u {
    (
            $t: ty,
            $f: ident,
            $test: ident,
            $helper: ident,
            $max: expr,
            $max_pos_bit_length: expr,
            $max_bits: expr
    ) => {
        fn $helper(size: usize, n: $t, out: &str) {
            assert_eq!(format!("{:?}", $f(size, n)), out);
        }

        #[test]
        fn $test() {
            $helper(8, 0, "[false, false, false, false, false, false, false, false]");
            $helper(8, 1, "[true, false, false, false, false, false, false, false]");
            $helper(8, 6, "[false, true, true, false, false, false, false, false]");
            $helper(8, 105, "[true, false, false, true, false, true, true, false]");
            $helper(2, 104, "[false, false]");
            $helper(2, 105, "[true, false]");
            $helper(1, 104, "[false]");
            $helper(1, 105, "[true]");
            $helper(0, 104, "[]");
            $helper(100, 105,
                    "[true, false, false, true, false, true, true, false, false, false, false, \
                     false, false, false, false, false, false, false, false, false, false, false, \
                     false, false, false, false, false, false, false, false, false, false, false, \
                     false, false, false, false, false, false, false, false, false, false, false, \
                     false, false, false, false, false, false, false, false, false, false, false, \
                     false, false, false, false, false, false, false, false, false, false, false, \
                     false, false, false, false, false, false, false, false, false, false, false, \
                     false, false, false, false, false, false, false, false, false, false, false, \
                     false, false, false, false, false, false, false, false, false, false, false, \
                     false]");
            $helper($max_pos_bit_length, $max, $max_bits);
        }
    }
}

test_bits_padded_u!(u8,
                    bits_padded_u8,
                    test_bits_padded_u8,
                    bits_padded_u8_helper,
                    u8::max_value(),
                    8,
                    "[true, true, true, true, true, true, true, true]");
test_bits_padded_u!(u16,
                    bits_padded_u16,
                    test_bits_padded_u16,
                    bits_padded_u16_helper,
                    u16::max_value(),
                    16,
                    "[true, true, true, true, true, true, true, true, true, true, true, true, \
                      true, true, true, true]");
test_bits_padded_u!(u32,
                    bits_padded_u32,
                    test_bits_padded_u32,
                    bits_padded_u32_helper,
                    u32::max_value(),
                    32,
                    "[true, true, true, true, true, true, true, true, true, true, true, true, \
                      true, true, true, true, true, true, true, true, true, true, true, true, \
                      true, true, true, true, true, true, true, true]");
test_bits_padded_u!(u64,
                    bits_padded_u64,
                    test_bits_padded_u64,
                    bits_padded_u64_helper,
                    u64::max_value(),
                    64,
                    "[true, true, true, true, true, true, true, true, true, true, true, true, \
                      true, true, true, true, true, true, true, true, true, true, true, true, \
                      true, true, true, true, true, true, true, true, true, true, true, true, \
                      true, true, true, true, true, true, true, true, true, true, true, true, \
                      true, true, true, true, true, true, true, true, true, true, true, true, \
                      true, true, true, true]");

macro_rules! test_bits_padded_i {
    (
            $t: ty,
            $f: ident,
            $test: ident,
            $helper: ident,
            $fail: ident,
            $max: expr,
            $max_pos_bit_length: expr,
            $max_bits: expr
    ) => {
        fn $helper(size: usize, n: $t, out: &str) {
            assert_eq!(format!("{:?}", $f(size, n)), out);
        }

        #[test]
        fn $test() {
            $helper(8, 0, "[false, false, false, false, false, false, false, false]");
            $helper(8, 1, "[true, false, false, false, false, false, false, false]");
            $helper(8, 6, "[false, true, true, false, false, false, false, false]");
            $helper(8, 105, "[true, false, false, true, false, true, true, false]");
            $helper(2, 104, "[false, false]");
            $helper(2, 105, "[true, false]");
            $helper(1, 104, "[false]");
            $helper(1, 105, "[true]");
            $helper(0, 104, "[]");
            $helper(100, 105,
                    "[true, false, false, true, false, true, true, false, false, false, false, \
                     false, false, false, false, false, false, false, false, false, false, false, \
                     false, false, false, false, false, false, false, false, false, false, false, \
                     false, false, false, false, false, false, false, false, false, false, false, \
                     false, false, false, false, false, false, false, false, false, false, false, \
                     false, false, false, false, false, false, false, false, false, false, false, \
                     false, false, false, false, false, false, false, false, false, false, false, \
                     false, false, false, false, false, false, false, false, false, false, false, \
                     false, false, false, false, false, false, false, false, false, false, false, \
                     false]");
            $helper($max_pos_bit_length, $max, $max_bits);
        }

        #[test]
        #[should_panic(expected = "n cannot be negative. Invalid n: -1")]
        fn $fail() {
            $f(8, -1);
        }
    }
}

test_bits_padded_i!(i8,
                    bits_padded_i8,
                    test_bits_padded_i8,
                    bits_padded_i8_helper,
                    test_bits_padded_i8_fail,
                    i8::max_value(),
                    7,
                    "[true, true, true, true, true, true, true]");
test_bits_padded_i!(i16,
                    bits_padded_i16,
                    test_bits_padded_i16,
                    bits_padded_i16_helper,
                    test_bits_padded_i16_fail,
                    i16::max_value(),
                    15,
                    "[true, true, true, true, true, true, true, true, true, true, true, true, \
                      true, true, true]");
test_bits_padded_i!(i32,
                    bits_padded_i32,
                    test_bits_padded_i32,
                    bits_padded_i32_helper,
                    test_bits_padded_i32_fail,
                    i32::max_value(),
                    31,
                    "[true, true, true, true, true, true, true, true, true, true, true, true, \
                      true, true, true, true, true, true, true, true, true, true, true, true, \
                      true, true, true, true, true, true, true]");
test_bits_padded_i!(i64,
                    bits_padded_i64,
                    test_bits_padded_i64,
                    bits_padded_i64_helper,
                    test_bits_padded_i64_fail,
                    i64::max_value(),
                    63,
                    "[true, true, true, true, true, true, true, true, true, true, true, true, \
                      true, true, true, true, true, true, true, true, true, true, true, true, \
                      true, true, true, true, true, true, true, true, true, true, true, true, \
                      true, true, true, true, true, true, true, true, true, true, true, true, \
                      true, true, true, true, true, true, true, true, true, true, true, true, \
                      true, true, true]");

fn bits_padded_integer_helper(size: usize, n: &str, out: &str) {
    assert_eq!(format!("{:?}",
                       bits_padded_integer(size, &Integer::from_str(n).unwrap())),
               out);
}

#[test]
fn test_bits_padded_integer() {
    bits_padded_integer_helper(8,
                               "0",
                               "[false, false, false, false, false, false, false, false]");
    bits_padded_integer_helper(8,
                               "1",
                               "[true, false, false, false, false, false, false, false]");
    bits_padded_integer_helper(8,
                               "6",
                               "[false, true, true, false, false, false, false, false]");
    bits_padded_integer_helper(8,
                               "105",
                               "[true, false, false, true, false, true, true, false]");
    bits_padded_integer_helper(2, "104", "[false, false]");
    bits_padded_integer_helper(2, "105", "[true, false]");
    bits_padded_integer_helper(1, "104", "[false]");
    bits_padded_integer_helper(1, "105", "[true]");
    bits_padded_integer_helper(0, "104", "[]");
    bits_padded_integer_helper(100,
                               "105",
                               "[true, false, false, true, false, true, true, false, false, false, \
                                 false, false, false, false, false, false, false, false, false, \
                                 false, false, false, false, false, false, false, false, false, \
                                 false, false, false, false, false, false, false, false, false, \
                                 false, false, false, false, false, false, false, false, false, \
                                 false, false, false, false, false, false, false, false, false, \
                                 false, false, false, false, false, false, false, false, false, \
                                 false, false, false, false, false, false, false, false, false, \
                                 false, false, false, false, false, false, false, false, false, \
                                 false, false, false, false, false, false, false, false, false, \
                                 false, false, false, false, false, false, false, false, false]");
}

#[test]
#[should_panic(expected = "n cannot be negative. Invalid n: -1")]
fn bits_padded_integer_fail() {
    bits_padded_integer(8, &Integer::from(-1));
}

macro_rules! test_big_endian_bits {
    ($t: ty, $f: ident, $test: ident, $helper: ident, $max: expr, $max_bits: expr) => {
        fn $helper(n: $t, out: &str) {
            assert_eq!(format!("{:?}", $f(n)), out);
        }

        #[test]
        fn $test() {
            $helper(0, "[]");
            $helper(1, "[true]");
            $helper(6, "[true, true, false]");
            $helper(105, "[true, true, false, true, false, false, true]");
            $helper($max, $max_bits);
        }
    }
}

test_big_endian_bits!(u8,
                      big_endian_bits_u8,
                      test_big_endian_bits_u8,
                      big_endian_bits_u8_helper,
                      u8::max_value(),
                      "[true, true, true, true, true, true, true, true]");
test_big_endian_bits!(u16,
                      big_endian_bits_u16,
                      test_big_endian_bits_u16,
                      big_endian_bits_u16_helper,
                      u16::max_value(),
                      "[true, true, true, true, true, true, true, true, true, true, true, true, \
                      true, true, true, true]");
test_big_endian_bits!(u32,
                      big_endian_bits_u32,
                      test_big_endian_bits_u32,
                      big_endian_bits_u32_helper,
                      u32::max_value(),
                      "[true, true, true, true, true, true, true, true, true, true, true, true, \
                        true, true, true, true, true, true, true, true, true, true, true, true, \
                        true, true, true, true, true, true, true, true]");
test_big_endian_bits!(u64,
                      big_endian_bits_u64,
                      test_big_endian_bits_u64,
                      big_endian_bits_u64_helper,
                      u64::max_value(),
                      "[true, true, true, true, true, true, true, true, true, true, true, true, \
                        true, true, true, true, true, true, true, true, true, true, true, true, \
                        true, true, true, true, true, true, true, true, true, true, true, true, \
                        true, true, true, true, true, true, true, true, true, true, true, true, \
                        true, true, true, true, true, true, true, true, true, true, true, true, \
                        true, true, true, true]");
test_big_endian_bits!(i8,
                      big_endian_bits_i8,
                      test_big_endian_bits_i8,
                      big_endian_bits_i8_helper,
                      i8::max_value(),
                      "[true, true, true, true, true, true, true]");
test_big_endian_bits!(i16,
                      big_endian_bits_i16,
                      test_big_endian_bits_i16,
                      big_endian_bits_i16_helper,
                      i16::max_value(),
                      "[true, true, true, true, true, true, true, true, true, true, true, true, \
                        true, true, true]");
test_big_endian_bits!(i32,
                      big_endian_bits_i32,
                      test_big_endian_bits_i32,
                      big_endian_bits_i32_helper,
                      i32::max_value(),
                      "[true, true, true, true, true, true, true, true, true, true, true, true, \
                        true, true, true, true, true, true, true, true, true, true, true, true, \
                        true, true, true, true, true, true, true]");
test_big_endian_bits!(i64,
                      big_endian_bits_i64,
                      test_big_endian_bits_i64,
                      big_endian_bits_i64_helper,
                      i64::max_value(),
                      "[true, true, true, true, true, true, true, true, true, true, true, true, \
                        true, true, true, true, true, true, true, true, true, true, true, true, \
                        true, true, true, true, true, true, true, true, true, true, true, true, \
                        true, true, true, true, true, true, true, true, true, true, true, true, \
                        true, true, true, true, true, true, true, true, true, true, true, true, \
                        true, true, true]");

macro_rules! test_big_endian_bits_i {
    ($t: ty, $f: ident, $fail: ident) => {
        #[test]
        #[should_panic(expected = "n cannot be negative. Invalid n: -5")]
        fn $fail() {
            $f(-5);
        }
    }
}

test_big_endian_bits_i!(i8, big_endian_bits_i8, big_endian_bits_i8_fail);
test_big_endian_bits_i!(i16, big_endian_bits_i16, big_endian_bits_i16_fail);
test_big_endian_bits_i!(i32, big_endian_bits_i32, big_endian_bits_i32_fail);
test_big_endian_bits_i!(i64, big_endian_bits_i64, big_endian_bits_i64_fail);
test_big_endian_bits_i!(isize, big_endian_bits_isize, big_endian_bits_isize_fail);

fn big_endian_bits_integer_helper(n: &str, out: &str) {
    assert_eq!(format!("{:?}",
                       big_endian_bits_integer(&Integer::from_str(n).unwrap())),
               out);
}

#[test]
fn test_big_endian_bits_integer() {
    big_endian_bits_integer_helper("0", "[]");
    big_endian_bits_integer_helper("1", "[true]");
    big_endian_bits_integer_helper("6", "[true, true, false]");
    big_endian_bits_integer_helper("105", "[true, true, false, true, false, false, true]");
}

#[test]
#[should_panic(expected = "n cannot be negative. Invalid n: -5")]
fn big_endian_bits_integer_fail() {
    big_endian_bits_integer(&Integer::from(-5));
}

macro_rules! test_big_endian_bits_padded_u {
    (
            $t: ty,
            $f: ident,
            $test: ident,
            $helper: ident,
            $max: expr,
            $max_pos_bit_length: expr,
            $max_bits: expr
    ) => {
        fn $helper(size: usize, n: $t, out: &str) {
            assert_eq!(format!("{:?}", $f(size, n)), out);
        }

        #[test]
        fn $test() {
            $helper(8, 0, "[false, false, false, false, false, false, false, false]");
            $helper(8, 1, "[false, false, false, false, false, false, false, true]");
            $helper(8, 6, "[false, false, false, false, false, true, true, false]");
            $helper(8, 105, "[false, true, true, false, true, false, false, true]");
            $helper(2, 104, "[false, false]");
            $helper(2, 105, "[false, true]");
            $helper(1, 104, "[false]");
            $helper(1, 105, "[true]");
            $helper(0, 104, "[]");
            $helper(100, 105,
                    "[false, false, false, false, false, false, false, false, false, false, false, \
                      false, false, false, false, false, false, false, false, false, false, false, \
                      false, false, false, false, false, false, false, false, false, false, false, \
                      false, false, false, false, false, false, false, false, false, false, false, \
                      false, false, false, false, false, false, false, false, false, false, false, \
                      false, false, false, false, false, false, false, false, false, false, false, \
                      false, false, false, false, false, false, false, false, false, false, false, \
                      false, false, false, false, false, false, false, false, false, false, false, \
                      false, false, false, false, false, true, true, false, true, false, false, \
                      true]");
            $helper($max_pos_bit_length, $max, $max_bits);
        }
    }
}

test_big_endian_bits_padded_u!(u8,
                               big_endian_bits_padded_u8,
                               test_big_endian_bits_padded_u8,
                               big_endian_bits_padded_u8_helper,
                               u8::max_value(),
                               8,
                               "[true, true, true, true, true, true, true, true]");
test_big_endian_bits_padded_u!(u16,
                               big_endian_bits_padded_u16,
                               test_big_endian_bits_padded_u16,
                               big_endian_bits_padded_u16_helper,
                               u16::max_value(),
                               16,
                               "[true, true, true, true, true, true, true, true, true, true, true, \
                                 true, true, true, true, true]");
test_big_endian_bits_padded_u!(u32,
                               big_endian_bits_padded_u32,
                               test_big_endian_bits_padded_u32,
                               big_endian_bits_padded_u32_helper,
                               u32::max_value(),
                               32,
                               "[true, true, true, true, true, true, true, true, true, true, true, \
                                 true, true, true, true, true, true, true, true, true, true, true, \
                                 true, true, true, true, true, true, true, true, true, true]");
test_big_endian_bits_padded_u!(u64,
                               big_endian_bits_padded_u64,
                               test_big_endian_bits_padded_u64,
                               big_endian_bits_padded_u64_helper,
                               u64::max_value(),
                               64,
                               "[true, true, true, true, true, true, true, true, true, true, true, \
                                 true, true, true, true, true, true, true, true, true, true, true, \
                                 true, true, true, true, true, true, true, true, true, true, true, \
                                 true, true, true, true, true, true, true, true, true, true, true, \
                                 true, true, true, true, true, true, true, true, true, true, true, \
                                 true, true, true, true, true, true, true, true, true]");

macro_rules! test_big_endian_bits_padded_i {
    (
            $t: ty,
            $f: ident,
            $test: ident,
            $helper: ident,
            $fail: ident,
            $max: expr,
            $max_pos_bit_length: expr,
            $max_bits: expr
    ) => {
        fn $helper(size: usize, n: $t, out: &str) {
            assert_eq!(format!("{:?}", $f(size, n)), out);
        }

        #[test]
        fn $test() {
            $helper(8, 0, "[false, false, false, false, false, false, false, false]");
            $helper(8, 1, "[false, false, false, false, false, false, false, true]");
            $helper(8, 6, "[false, false, false, false, false, true, true, false]");
            $helper(8, 105, "[false, true, true, false, true, false, false, true]");
            $helper(2, 104, "[false, false]");
            $helper(2, 105, "[false, true]");
            $helper(1, 104, "[false]");
            $helper(1, 105, "[true]");
            $helper(0, 104, "[]");
            $helper(100, 105,
                    "[false, false, false, false, false, false, false, false, false, false, false, \
                      false, false, false, false, false, false, false, false, false, false, false, \
                      false, false, false, false, false, false, false, false, false, false, false, \
                      false, false, false, false, false, false, false, false, false, false, false, \
                      false, false, false, false, false, false, false, false, false, false, false, \
                      false, false, false, false, false, false, false, false, false, false, false, \
                      false, false, false, false, false, false, false, false, false, false, false, \
                      false, false, false, false, false, false, false, false, false, false, false, \
                      false, false, false, false, false, true, true, false, true, false, false, \
                      true]");
            $helper($max_pos_bit_length, $max, $max_bits);
        }

        #[test]
        #[should_panic(expected = "n cannot be negative. Invalid n: -1")]
        fn $fail() {
            $f(8, -1);
        }
    }
}

test_big_endian_bits_padded_i!(i8,
                               big_endian_bits_padded_i8,
                               test_big_endian_bits_padded_i8,
                               big_endian_bits_padded_i8_helper,
                               test_big_endian_bits_padded_i8_fail,
                               i8::max_value(),
                               7,
                               "[true, true, true, true, true, true, true]");
test_big_endian_bits_padded_i!(i16,
                               big_endian_bits_padded_i16,
                               test_big_endian_bits_padded_i16,
                               big_endian_bits_padded_i16_helper,
                               test_big_endian_bits_padded_i16_fail,
                               i16::max_value(),
                               15,
                               "[true, true, true, true, true, true, true, true, true, true, true, \
                                 true, true, true, true]");
test_big_endian_bits_padded_i!(i32,
                               big_endian_bits_padded_i32,
                               test_big_endian_bits_padded_i32,
                               big_endian_bits_padded_i32_helper,
                               test_big_endian_bits_padded_i32_fail,
                               i32::max_value(),
                               31,
                               "[true, true, true, true, true, true, true, true, true, true, true, \
                                 true, true, true, true, true, true, true, true, true, true, true, \
                                 true, true, true, true, true, true, true, true, true]");
test_big_endian_bits_padded_i!(i64,
                               big_endian_bits_padded_i64,
                               test_big_endian_bits_padded_i64,
                               big_endian_bits_padded_i64_helper,
                               test_big_endian_bits_padded_i64_fail,
                               i64::max_value(),
                               63,
                               "[true, true, true, true, true, true, true, true, true, true, true, \
                                 true, true, true, true, true, true, true, true, true, true, true, \
                                 true, true, true, true, true, true, true, true, true, true, true, \
                                 true, true, true, true, true, true, true, true, true, true, true, \
                                 true, true, true, true, true, true, true, true, true, true, true, \
                                 true, true, true, true, true, true, true, true]");

fn big_endian_bits_padded_integer_helper(size: usize, n: &str, out: &str) {
    assert_eq!(format!("{:?}",
                       big_endian_bits_padded_integer(size, &Integer::from_str(n).unwrap())),
               out);
}

#[test]
fn test_big_endian_bits_padded_integer() {
    big_endian_bits_padded_integer_helper(8,
                                          "0",
                                          "[false, false, false, false, false, false, false, \
                                            false]");
    big_endian_bits_padded_integer_helper(8,
                                          "1",
                                          "[false, false, false, false, false, false, false, \
                                            true]");
    big_endian_bits_padded_integer_helper(8,
                                          "6",
                                          "[false, false, false, false, false, true, true, \
                                            false]");
    big_endian_bits_padded_integer_helper(8,
                                          "105",
                                          "[false, true, true, false, true, false, false, true]");
    big_endian_bits_padded_integer_helper(2, "104", "[false, false]");
    big_endian_bits_padded_integer_helper(2, "105", "[false, true]");
    big_endian_bits_padded_integer_helper(1, "104", "[false]");
    big_endian_bits_padded_integer_helper(1, "105", "[true]");
    big_endian_bits_padded_integer_helper(0, "104", "[]");
    big_endian_bits_padded_integer_helper(100,
                                          "105",
                                          "[false, false, false, false, false, false, false, \
                                            false, false, false, false, false, false, false, \
                                            false, false, false, false, false, false, false, \
                                            false, false, false, false, false, false, false, \
                                            false, false, false, false, false, false, false, \
                                            false, false, false, false, false, false, false, \
                                            false, false, false, false, false, false, false, \
                                            false, false, false, false, false, false, false, \
                                            false, false, false, false, false, false, false, \
                                            false, false, false, false, false, false, false, \
                                            false, false, false, false, false, false, false, \
                                            false, false, false, false, false, false, false, \
                                            false, false, false, false, false, false, false, \
                                            false, false, true, true, false, true, false, false, \
                                            true]");
}

#[test]
#[should_panic(expected = "n cannot be negative. Invalid n: -1")]
fn big_endian_bits_padded_integer_fail() {
    big_endian_bits_padded_integer(8, &Integer::from(-1));
}

fn from_big_endian_bits_helper(bits: &str, out: &str) {
    let bits: Vec<bool> = parse_vec(bits).unwrap();
    assert_eq!(format!("{}", from_big_endian_bits(&bits[..])), out);
}

#[test]
fn test_from_big_endian_bits() {
    from_big_endian_bits_helper("[]", "0");
    from_big_endian_bits_helper("[false, false]", "0");
    from_big_endian_bits_helper("[false, true]", "1");
    from_big_endian_bits_helper("[false, false, false, false, false, true, true, false]",
                                "6");
    from_big_endian_bits_helper("[true, true, false, true, false, false, true]", "105");
}

macro_rules! test_digits {
    (
        $t: ty,
        $f: ident,
        $test: ident,
        $helper: ident,
        $fail_1: ident,
        $fail_2: ident,
        $max: expr,
        $max_digit: expr
    ) => {
        fn $helper(radix: $t, n: $t, out: &str) {
            assert_eq!(format!("{:?}", $f(radix, n)), out);
        }

        #[test]
        fn $test() {
            $helper(2, 0, "[]");
            $helper(3, 0, "[]");
            $helper(8, 0, "[]");
            $helper(10, 0, "[]");
            $helper(12, 0, "[]");
            $helper(57, 0, "[]");
            $helper(2, 1, "[1]");
            $helper(3, 1, "[1]");
            $helper(8, 1, "[1]");
            $helper(10, 1, "[1]");
            $helper(12, 1, "[1]");
            $helper(57, 1, "[1]");
            $helper(2, 10, "[0, 1, 0, 1]");
            $helper(3, 10, "[1, 0, 1]");
            $helper(8, 10, "[2, 1]");
            $helper(10, 10, "[0, 1]");
            $helper(12, 10, "[10]");
            $helper(57, 10, "[10]");
            $helper(2, 107, "[1, 1, 0, 1, 0, 1, 1]");
            $helper(3, 107, "[2, 2, 2, 0, 1]");
            $helper(8, 107, "[3, 5, 1]");
            $helper(10, 107, "[7, 0, 1]");
            $helper(12, 107, "[11, 8]");
            $helper(57, 107, "[50, 1]");
            $helper($max, 0, "[]");
            $helper($max, 107, "[107]");
            $helper($max, $max - 1, $max_digit);
            $helper($max, $max, "[0, 1]");
        }

        #[test]
        #[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
        fn $fail_1() {
            $f(1, 10);
        }

        #[test]
        #[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
        fn $fail_2() {
            $f(0, 10);
        }
    }
}

test_digits!(u8,
             digits_u8,
             test_digits_u8,
             digits_u8_helper,
             digits_u8_fail_1,
             digits_u8_fail_2,
             u8::max_value(),
             "[254]");
test_digits!(u16,
             digits_u16,
             test_digits_u16,
             digits_u16_helper,
             digits_u16_fail_1,
             digits_u16_fail_2,
             u16::max_value(),
             "[65534]");
test_digits!(u32,
             digits_u32,
             test_digits_u32,
             digits_u32_helper,
             digits_u32_fail_1,
             digits_u32_fail_2,
             u32::max_value(),
             "[4294967294]");
test_digits!(u64,
             digits_u64,
             test_digits_u64,
             digits_u64_helper,
             digits_u64_fail_1,
             digits_u64_fail_2,
             u64::max_value(),
             "[18446744073709551614]");
test_digits!(i8,
             digits_i8,
             test_digits_i8,
             digits_i8_helper,
             digits_i8_fail_1,
             digits_i8_fail_2,
             i8::max_value(),
             "[126]");
test_digits!(i16,
             digits_i16,
             test_digits_i16,
             digits_i16_helper,
             digits_i16_fail_1,
             digits_i16_fail_2,
             i16::max_value(),
             "[32766]");
test_digits!(i32,
             digits_i32,
             test_digits_i32,
             digits_i32_helper,
             digits_i32_fail_1,
             digits_i32_fail_2,
             i32::max_value(),
             "[2147483646]");
test_digits!(i64,
             digits_i64,
             test_digits_i64,
             digits_i64_helper,
             digits_i64_fail_1,
             digits_i64_fail_2,
             i64::max_value(),
             "[9223372036854775806]");

macro_rules! test_digits_i {
    ($t: ty, $f: ident, $fail_3: ident, $fail_4: ident) => {
        #[test]
        #[should_panic(expected = "n cannot be negative. Invalid n: -1")]
        fn $fail_3() {
            $f(2, -1);
        }

        #[test]
        #[should_panic(expected = "n cannot be negative. Invalid n: -1")]
        fn $fail_4() {
            $f(0, -1);
        }
    }
}

test_digits_i!(i8, digits_i8, digits_i8_fail_3, digits_i8_fail_4);
test_digits_i!(i16, digits_i16, digits_i16_fail_3, digits_i16_fail_4);
test_digits_i!(i32, digits_i32, digits_i32_fail_3, digits_i32_fail_4);
test_digits_i!(i64, digits_i64, digits_i64_fail_3, digits_i64_fail_4);
test_digits_i!(isize,
               digits_isize,
               digits_isize_fail_3,
               digits_isize_fail_4);
