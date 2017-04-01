extern crate rugint;
extern crate rust_wheels_lib;

use self::rugint::Integer;
use self::rust_wheels_lib::number_utils::integer_utils::*;
use std::str::FromStr;

macro_rules! test_is_power_of_two_u {
    ($t: ty, $f: ident, $test: ident, $helper: ident, $fail: ident, $p10: expr, $max: expr) => {
        fn $helper(n: $t, out: bool) {
            assert_eq!($f(n), out);
        }

        #[test]
        fn $test() {
            $helper(1, true);
            $helper(2, true);
            $helper(4, true);
            $helper(8, true);
            $helper(16, true);
            $helper(3, false);
            $helper(13, false);
            $helper($p10, false);
            $helper($max, false);
        }

        #[test]
        #[should_panic(expected = "n must be positive. Invalid n: 0")]
        fn $fail() {
            $f(0);
        }
    }
}

test_is_power_of_two_u!(u8,
                        is_power_of_two_u8,
                        test_is_power_of_two_u8,
                        is_power_of_two_u8_helper,
                        test_is_power_of_two_u8_fail,
                        100,
                        u8::max_value());
test_is_power_of_two_u!(u16,
                        is_power_of_two_u16,
                        test_is_power_of_two_u16,
                        is_power_of_two_u16_helper,
                        test_is_power_of_two_u16_fail,
                        10000,
                        u16::max_value());
test_is_power_of_two_u!(u32,
                        is_power_of_two_u32,
                        test_is_power_of_two_u32,
                        is_power_of_two_u32_helper,
                        test_is_power_of_two_u32_fail,
                        1000000000,
                        u32::max_value());
test_is_power_of_two_u!(u64,
                        is_power_of_two_u64,
                        test_is_power_of_two_u64,
                        is_power_of_two_u64_helper,
                        test_is_power_of_two_u64_fail,
                        10000000000000000000,
                        u64::max_value());
test_is_power_of_two_u!(usize,
                        is_power_of_two_usize,
                        test_is_power_of_two_usize,
                        is_power_of_two_usize_helper,
                        test_is_power_of_two_usize_fail,
                        100,
                        usize::max_value());

macro_rules! test_is_power_of_two_i {
    (
            $t: ty,
            $f: ident,
            $test: ident,
            $helper: ident,
            $fail1: ident,
            $fail2: ident,
            $p10: expr,
            $max: expr
    ) => {
        fn $helper(n: $t, out: bool) {
            assert_eq!($f(n), out);
        }

        #[test]
        fn $test() {
            $helper(1, true);
            $helper(2, true);
            $helper(4, true);
            $helper(8, true);
            $helper(16, true);
            $helper(3, false);
            $helper(13, false);
            $helper($p10, false);
            $helper($max, false);
        }

        #[test]
        #[should_panic(expected = "n must be positive. Invalid n: 0")]
        fn $fail1() {
            $f(0);
        }

        #[test]
        #[should_panic(expected = "n must be positive. Invalid n: -5")]
        fn $fail2() {
            $f(-5);
        }
    }
}

test_is_power_of_two_i!(i8,
                        is_power_of_two_i8,
                        test_is_power_of_two_i8,
                        is_power_of_two_i8_helper,
                        test_is_power_of_two_i8_fail_1,
                        test_is_power_of_two_i8_fail_2,
                        100,
                        i8::max_value());
test_is_power_of_two_i!(i16,
                        is_power_of_two_i16,
                        test_is_power_of_two_i16,
                        is_power_of_two_i16_helper,
                        test_is_power_of_two_i16_fail_1,
                        test_is_power_of_two_i16_fail_2,
                        10000,
                        i16::max_value());
test_is_power_of_two_i!(i32,
                        is_power_of_two_i32,
                        test_is_power_of_two_i32,
                        is_power_of_two_i32_helper,
                        test_is_power_of_two_i32_fail_1,
                        test_is_power_of_two_i32_fail_2,
                        1000000000,
                        i32::max_value());
test_is_power_of_two_i!(i64,
                        is_power_of_two_i64,
                        test_is_power_of_two_i64,
                        is_power_of_two_i64_helper,
                        test_is_power_of_two_i64_fail_1,
                        test_is_power_of_two_i64_fail_2,
                        1000000000000000000,
                        i64::max_value());
test_is_power_of_two_i!(isize,
                        is_power_of_two_isize,
                        test_is_power_of_two_isize,
                        is_power_of_two_isize_helper,
                        test_is_power_of_two_isize_fail_1,
                        test_is_power_of_two_isize_fail_2,
                        100,
                        isize::max_value());

fn is_power_of_two_integer_helper(n: &str, out: bool) {
    assert_eq!(is_power_of_two_integer(&Integer::from_str(n).unwrap()), out);
}

#[test]
fn test_is_power_of_two_integer() {
    is_power_of_two_integer_helper("1", true);
    is_power_of_two_integer_helper("2", true);
    is_power_of_two_integer_helper("4", true);
    is_power_of_two_integer_helper("8", true);
    is_power_of_two_integer_helper("16", true);
    is_power_of_two_integer_helper("3", false);
    is_power_of_two_integer_helper("13", false);
}

fn is_power_of_two_integer_fail_helper(n: &str) {
    is_power_of_two_integer(&Integer::from_str(n).unwrap());
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: 0")]
fn is_power_of_two_integer_fail_1() {
    is_power_of_two_integer_fail_helper("0");
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: -5")]
fn is_power_of_two_integer_fail_2() {
    is_power_of_two_integer_fail_helper("-5");
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

macro_rules! test_bits_s {
    ($t: ty, $f: ident, $fail: ident) => {
        #[test]
        #[should_panic(expected = "n cannot be negative. Invalid n: -5")]
        fn $fail() {
            $f(-5);
        }
    }
}

test_bits_s!(i8, bits_i8, bits_i8_fail);
test_bits_s!(i16, bits_i16, bits_i16_fail);
test_bits_s!(i32, bits_i32, bits_i32_fail);
test_bits_s!(i64, bits_i64, bits_i64_fail);
test_bits_s!(isize, bits_isize, bits_isize_fail);

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

macro_rules! test_big_endian_bits_s {
    ($t: ty, $f: ident, $fail: ident) => {
        #[test]
        #[should_panic(expected = "n cannot be negative. Invalid n: -5")]
        fn $fail() {
            $f(-5);
        }
    }
}

test_big_endian_bits_s!(i8, big_endian_bits_i8, big_endian_bits_i8_fail);
test_big_endian_bits_s!(i16, big_endian_bits_i16, big_endian_bits_i16_fail);
test_big_endian_bits_s!(i32, big_endian_bits_i32, big_endian_bits_i32_fail);
test_big_endian_bits_s!(i64, big_endian_bits_i64, big_endian_bits_i64_fail);
test_big_endian_bits_s!(isize, big_endian_bits_isize, big_endian_bits_isize_fail);

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
