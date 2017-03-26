extern crate gmp;
extern crate rust_wheels_lib;

use self::gmp::mpz::Mpz;
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

macro_rules! test_is_power_of_two_i {
    ($t: ty, $f: ident, $test: ident, $helper: ident, $fail1: ident, $fail2: ident, $p10: expr,
            $max: expr) => {
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

fn is_power_of_two_mpz_helper(n: &str, out: bool) {
    assert_eq!(is_power_of_two_mpz(&Mpz::from_str(n).unwrap()), out);
}

#[test]
fn test_is_power_of_two_mpz() {
    is_power_of_two_mpz_helper("1", true);
    is_power_of_two_mpz_helper("2", true);
    is_power_of_two_mpz_helper("4", true);
    is_power_of_two_mpz_helper("8", true);
    is_power_of_two_mpz_helper("16", true);
    is_power_of_two_mpz_helper("3", false);
    is_power_of_two_mpz_helper("13", false);
}

fn is_power_of_two_mpz_fail_helper(n: &str) {
    is_power_of_two_mpz(&Mpz::from_str(n).unwrap());
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: 0")]
fn test_is_power_of_two_mpz_fail_1() {
    is_power_of_two_mpz_fail_helper("0");
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: -5")]
fn test_is_power_of_two_mpz_fail_2() {
    is_power_of_two_mpz_fail_helper("-5");
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

fn ceiling_log_2_mpz_helper(n: &str, out: u32) {
    assert_eq!(ceiling_log_2_mpz(&Mpz::from_str(n).unwrap()), out);
}

#[test]
fn test_ceiling_log_2_mpz() {
    ceiling_log_2_mpz_helper("1", 0);
    ceiling_log_2_mpz_helper("2", 1);
    ceiling_log_2_mpz_helper("3", 2);
    ceiling_log_2_mpz_helper("4", 2);
    ceiling_log_2_mpz_helper("5", 3);
    ceiling_log_2_mpz_helper("6", 3);
    ceiling_log_2_mpz_helper("7", 3);
    ceiling_log_2_mpz_helper("8", 3);
    ceiling_log_2_mpz_helper("9", 4);
    ceiling_log_2_mpz_helper("100", 7);
}

fn ceiling_log_2_mpz_fail_helper(n: &str) {
    ceiling_log_2_mpz(&Mpz::from_str(n).unwrap());
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: 0")]
fn test_ceiling_log_2_mpz_fail_1() {
    ceiling_log_2_mpz_fail_helper("0");
}

#[test]
#[should_panic(expected = "n must be positive. Invalid n: -5")]
fn test_ceiling_log_2_mpz_fail_2() {
    ceiling_log_2_mpz_fail_helper("-5");
}