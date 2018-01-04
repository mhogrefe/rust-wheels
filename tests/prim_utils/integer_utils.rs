use rust_wheels::prim_utils::integer_utils::*;

use malachite::integer::Integer;
use malachite_base::traits::NegativeOne;
use rust_wheels::io::readers::parse_vec;
use rust_wheels::prim_utils::traits::PrimUnsignedInt;
use std::str::FromStr;

macro_rules! prim_fail {
    (
        $t: ty,
        $ceiling_log_2_fail: ident,
        $digits_fail_1: ident,
        $digits_fail_2: ident,
        $digits_padded_fail_1: ident,
        $digits_padded_fail_2: ident,
        $big_endian_digits_fail_1: ident,
        $big_endian_digits_fail_2: ident,
        $big_endian_digits_padded_fail_1: ident,
        $big_endian_digits_padded_fail_2: ident
    ) => {
        #[test]
        #[should_panic(expected = "n must be positive. Invalid n: 0")]
        fn $ceiling_log_2_fail() {
            ceiling_log_2_u::<$t>(0);
        }

        #[test]
        #[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
        fn $digits_fail_1() {
            digits_u::<$t>(1, 10);
        }

        #[test]
        #[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
        fn $digits_fail_2() {
            digits_u::<$t>(0, 10);
        }

        #[test]
        #[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
        fn $digits_padded_fail_1() {
            digits_padded_u::<$t>(3, 1, 10);
        }

        #[test]
        #[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
        fn $digits_padded_fail_2() {
            digits_padded_u::<$t>(3, 0, 10);
        }

        #[test]
        #[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
        fn $big_endian_digits_fail_1() {
            big_endian_digits_u::<$t>(1, 10);
        }

        #[test]
        #[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
        fn $big_endian_digits_fail_2() {
            big_endian_digits_u::<$t>(0, 10);
        }

        #[test]
        #[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
        fn $big_endian_digits_padded_fail_1() {
            big_endian_digits_padded_u::<$t>(3, 1, 10);
        }

        #[test]
        #[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
        fn $big_endian_digits_padded_fail_2() {
            big_endian_digits_padded_u::<$t>(3, 0, 10);
        }
    }
}

prim_fail!(
    u8,
    ceiling_log_2_u8_fail,
    digits_u8_fail_1,
    digits_u8_fail_2,
    digits_padded_u8_fail_1,
    digits_padded_u8_fail_2,
    big_endian_digits_u8_fail_1,
    big_endian_digits_u8_fail_2,
    big_endian_digits_padded_u8_fail_1,
    big_endian_digits_padded_u8_fail_2
);
prim_fail!(
    u16,
    ceiling_log_2_u16_fail,
    digits_u16_fail_1,
    digits_u16_fail_2,
    digits_padded_u16_fail_1,
    digits_padded_u16_fail_2,
    big_endian_digits_u16_fail_1,
    big_endian_digits_u16_fail_2,
    big_endian_digits_padded_u16_fail_1,
    big_endian_digits_padded_u16_fail_2
);
prim_fail!(
    u32,
    ceiling_log_2_u32_fail,
    digits_u32_fail_1,
    digits_u32_fail_2,
    digits_padded_u32_fail_1,
    digits_padded_u32_fail_2,
    big_endian_digits_u32_fail_1,
    big_endian_digits_u32_fail_2,
    big_endian_digits_padded_u32_fail_1,
    big_endian_digits_padded_u32_fail_2
);
prim_fail!(
    u64,
    ceiling_log_2_u64_fail,
    digits_u64_fail_1,
    digits_u64_fail_2,
    digits_padded_u64_fail_1,
    digits_padded_u64_fail_2,
    big_endian_digits_u64_fail_1,
    big_endian_digits_u64_fail_2,
    big_endian_digits_padded_u64_fail_1,
    big_endian_digits_padded_u64_fail_2
);
prim_fail!(
    usize,
    ceiling_log_2_usize_fail,
    digits_usize_fail_1,
    digits_usize_fail_2,
    digits_padded_usize_fail_1,
    digits_padded_usize_fail_2,
    big_endian_digits_usize_fail_1,
    big_endian_digits_usize_fail_2,
    big_endian_digits_padded_usize_fail_1,
    big_endian_digits_padded_usize_fail_2
);

fn ceiling_log_2_u_helper<T: PrimUnsignedInt>() {
    let test = |n, out| assert_eq!(ceiling_log_2_u(n), out);
    test(T::from_u8(1), 0);
    test(T::from_u8(2), 1);
    test(T::from_u8(3), 2);
    test(T::from_u8(4), 2);
    test(T::from_u8(5), 3);
    test(T::from_u8(6), 3);
    test(T::from_u8(7), 3);
    test(T::from_u8(8), 3);
    test(T::from_u8(9), 4);
    test(T::from_u8(100), 7);
    test(T::max_value(), T::bit_count());
}

#[test]
fn test_ceiling_log_2_u() {
    ceiling_log_2_u_helper::<u8>();
    ceiling_log_2_u_helper::<u16>();
    ceiling_log_2_u_helper::<u32>();
    ceiling_log_2_u_helper::<u64>();
    ceiling_log_2_u_helper::<usize>();
}

#[test]
fn test_ceiling_log_2_integer() {
    let test = |n, out| assert_eq!(ceiling_log_2_integer(&Integer::from_str(n).unwrap()), out);
    test("1", 0);
    test("2", 1);
    test("3", 2);
    test("4", 2);
    test("5", 3);
    test("6", 3);
    test("7", 3);
    test("8", 3);
    test("9", 4);
    test("100", 7);
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

fn bits_u_helper<T: PrimUnsignedInt>(max_bits: Vec<bool>) {
    let test = |n, out| assert_eq!(bits_u(n), out);
    test(T::from_u8(0), vec![]);
    test(T::from_u8(1), vec![true]);
    test(T::from_u8(6), vec![false, true, true]);
    test(
        T::from_u8(105),
        vec![true, false, false, true, false, true, true],
    );
    test(T::max_value(), max_bits);
}

#[test]
fn test_bits_u() {
    bits_u_helper::<u8>(vec![true, true, true, true, true, true, true, true]);
    bits_u_helper::<u16>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ]);
    bits_u_helper::<u32>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true,
    ]);
    bits_u_helper::<u64>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true,
    ]);
}

#[test]
fn test_bits_integer() {
    let test = |n, out| assert_eq!(bits_integer(&Integer::from_str(n).unwrap()), out);
    test("0", vec![]);
    test("1", vec![true]);
    test("6", vec![false, true, true]);
    test("105", vec![true, false, false, true, false, true, true]);
}

#[test]
#[should_panic(expected = "n cannot be negative. Invalid n: -5")]
fn bits_integer_fail() {
    bits_integer(&Integer::from(-5));
}

fn bits_padded_u_helper<T: PrimUnsignedInt>(max_bits: Vec<bool>) {
    let test = |size, n, out| assert_eq!(bits_padded_u(size, n), out);
    test(
        8,
        T::from_u8(0),
        vec![false, false, false, false, false, false, false, false],
    );
    test(
        8,
        T::from_u8(1),
        vec![true, false, false, false, false, false, false, false],
    );
    test(
        8,
        T::from_u8(6),
        vec![false, true, true, false, false, false, false, false],
    );
    test(
        8,
        T::from_u8(105),
        vec![true, false, false, true, false, true, true, false],
    );
    test(2, T::from_u8(104), vec![false, false]);
    test(2, T::from_u8(105), vec![true, false]);
    test(1, T::from_u8(104), vec![false]);
    test(1, T::from_u8(105), vec![true]);
    test(0, T::from_u8(104), vec![]);
    test(
        100,
        T::from_u8(105),
        vec![
            true, false, false, true, false, true, true, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false,
        ],
    );
    test(T::bit_count() as usize, T::max_value(), max_bits);
}

#[test]
fn test_bits_padded_u() {
    bits_padded_u_helper::<u8>(vec![true, true, true, true, true, true, true, true]);
    bits_padded_u_helper::<u16>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ]);
    bits_padded_u_helper::<u32>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true,
    ]);
    bits_padded_u_helper::<u64>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true,
    ]);
}

#[test]
fn test_bits_padded_integer() {
    let test = |size, n, out| {
        assert_eq!(
            bits_padded_integer(size, &Integer::from_str(n).unwrap()),
            out
        )
    };
    test(
        8,
        "0",
        vec![false, false, false, false, false, false, false, false],
    );
    test(
        8,
        "1",
        vec![true, false, false, false, false, false, false, false],
    );
    test(
        8,
        "6",
        vec![false, true, true, false, false, false, false, false],
    );
    test(
        8,
        "105",
        vec![true, false, false, true, false, true, true, false],
    );
    test(2, "104", vec![false, false]);
    test(2, "105", vec![true, false]);
    test(1, "104", vec![false]);
    test(1, "105", vec![true]);
    test(0, "104", vec![]);
    test(
        100,
        "105",
        vec![
            true, false, false, true, false, true, true, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false,
        ],
    );
}

#[test]
#[should_panic(expected = "n cannot be negative. Invalid n: -1")]
fn bits_padded_integer_fail() {
    bits_padded_integer(8, &Integer::NEGATIVE_ONE);
}

fn big_endian_bits_u_helper<T: PrimUnsignedInt>(max_bits: Vec<bool>) {
    let test = |n, out| assert_eq!(big_endian_bits_u(n), out);
    test(T::from_u8(0), vec![]);
    test(T::from_u8(1), vec![true]);
    test(T::from_u8(6), vec![true, true, false]);
    test(
        T::from_u8(105),
        vec![true, true, false, true, false, false, true],
    );
    test(T::max_value(), max_bits);
}

#[test]
fn big_endian_test_bits_u() {
    big_endian_bits_u_helper::<u8>(vec![true, true, true, true, true, true, true, true]);
    big_endian_bits_u_helper::<u16>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ]);
    big_endian_bits_u_helper::<u32>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true,
    ]);
    big_endian_bits_u_helper::<u64>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true,
    ]);
}

#[test]
fn big_endian_bits_integer_helper() {
    let test = |n, out| assert_eq!(big_endian_bits_integer(&Integer::from_str(n).unwrap()), out);
    test("0", vec![]);
    test("1", vec![true]);
    test("6", vec![true, true, false]);
    test("105", vec![true, true, false, true, false, false, true]);
}

fn big_endian_bits_padded_u_helper<T: PrimUnsignedInt>(max_bits: Vec<bool>) {
    let test = |size, n, out| assert_eq!(big_endian_bits_padded_u(size, n), out);
    test(
        8,
        T::from_u8(0),
        vec![false, false, false, false, false, false, false, false],
    );
    test(
        8,
        T::from_u8(1),
        vec![false, false, false, false, false, false, false, true],
    );
    test(
        8,
        T::from_u8(6),
        vec![false, false, false, false, false, true, true, false],
    );
    test(
        8,
        T::from_u8(105),
        vec![false, true, true, false, true, false, false, true],
    );
    test(2, T::from_u8(104), vec![false, false]);
    test(2, T::from_u8(105), vec![false, true]);
    test(1, T::from_u8(104), vec![false]);
    test(1, T::from_u8(105), vec![true]);
    test(0, T::from_u8(104), vec![]);
    test(
        100,
        T::from_u8(105),
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, true, true, false, true,
            false, false, true,
        ],
    );
    test(T::bit_count() as usize, T::max_value(), max_bits);
}

#[test]
fn test_big_endian_bits_padded_u() {
    big_endian_bits_padded_u_helper::<u8>(vec![true, true, true, true, true, true, true, true]);
    big_endian_bits_padded_u_helper::<u16>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ]);
    big_endian_bits_padded_u_helper::<u32>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true,
    ]);
    big_endian_bits_padded_u_helper::<u64>(vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true,
    ]);
}

#[test]
#[should_panic(expected = "n cannot be negative. Invalid n: -5")]
fn big_endian_bits_integer_fail() {
    big_endian_bits_integer(&Integer::from(-5));
}

fn big_endian_bits_padded_integer_helper(size: usize, n: &str, out: &[bool]) {
    assert_eq!(
        big_endian_bits_padded_integer(size, &Integer::from_str(n).unwrap()),
        out
    );
}

#[test]
fn test_big_endian_bits_padded_integer() {
    big_endian_bits_padded_integer_helper(
        8,
        "0",
        &[false, false, false, false, false, false, false, false],
    );
    big_endian_bits_padded_integer_helper(
        8,
        "1",
        &[false, false, false, false, false, false, false, true],
    );
    big_endian_bits_padded_integer_helper(
        8,
        "6",
        &[false, false, false, false, false, true, true, false],
    );
    big_endian_bits_padded_integer_helper(
        8,
        "105",
        &[false, true, true, false, true, false, false, true],
    );
    big_endian_bits_padded_integer_helper(2, "104", &[false, false]);
    big_endian_bits_padded_integer_helper(2, "105", &[false, true]);
    big_endian_bits_padded_integer_helper(1, "104", &[false]);
    big_endian_bits_padded_integer_helper(1, "105", &[true]);
    big_endian_bits_padded_integer_helper(0, "104", &[]);
    big_endian_bits_padded_integer_helper(
        100,
        "105",
        &[
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, true, true, false, true,
            false, false, true,
        ],
    );
}

#[test]
#[should_panic(expected = "n cannot be negative. Invalid n: -1")]
fn big_endian_bits_padded_integer_fail() {
    big_endian_bits_padded_integer(8, &Integer::NEGATIVE_ONE);
}

fn from_big_endian_bits_helper(bits: &[bool], out: &str) {
    assert_eq!(format!("{}", from_big_endian_bits(&bits[..])), out);
}

#[test]
fn test_from_big_endian_bits() {
    from_big_endian_bits_helper(&[], "0");
    from_big_endian_bits_helper(&[false, false], "0");
    from_big_endian_bits_helper(&[false, true], "1");
    from_big_endian_bits_helper(&[false, false, false, false, false, true, true, false], "6");
    from_big_endian_bits_helper(&[true, true, false, true, false, false, true], "105");
}

#[test]
fn test_digit_to_char() {
    let test = |i, out| assert_eq!(digit_to_char(i), out);
    test(0, Some('0'));
    test(6, Some('6'));
    test(10, Some('a'));
    test(20, Some('k'));
    test(35, Some('z'));
    test(36, None);
}

#[test]
fn test_char_to_digit() {
    let test = |c, out| assert_eq!(char_to_digit(c), out);
    test('0', Some(0));
    test('6', Some(6));
    test('a', Some(10));
    test('k', Some(20));
    test('z', Some(35));
    test(' ', None);
    test('A', None);
}

fn digits_u_helper<T: PrimUnsignedInt>(max_digit: &str) {
    let test = |radix, n, out| assert_eq!(format!("{:?}", digits_u(radix, n)), out);
    test(T::from_u8(2), T::from_u8(0), "[]");
    test(T::from_u8(3), T::from_u8(0), "[]");
    test(T::from_u8(8), T::from_u8(0), "[]");
    test(T::from_u8(10), T::from_u8(0), "[]");
    test(T::from_u8(12), T::from_u8(0), "[]");
    test(T::from_u8(57), T::from_u8(0), "[]");
    test(T::from_u8(2), T::from_u8(1), "[1]");
    test(T::from_u8(3), T::from_u8(1), "[1]");
    test(T::from_u8(8), T::from_u8(1), "[1]");
    test(T::from_u8(10), T::from_u8(1), "[1]");
    test(T::from_u8(12), T::from_u8(1), "[1]");
    test(T::from_u8(57), T::from_u8(1), "[1]");
    test(T::from_u8(2), T::from_u8(10), "[0, 1, 0, 1]");
    test(T::from_u8(3), T::from_u8(10), "[1, 0, 1]");
    test(T::from_u8(8), T::from_u8(10), "[2, 1]");
    test(T::from_u8(10), T::from_u8(10), "[0, 1]");
    test(T::from_u8(12), T::from_u8(10), "[10]");
    test(T::from_u8(57), T::from_u8(10), "[10]");
    test(T::from_u8(2), T::from_u8(107), "[1, 1, 0, 1, 0, 1, 1]");
    test(T::from_u8(3), T::from_u8(107), "[2, 2, 2, 0, 1]");
    test(T::from_u8(8), T::from_u8(107), "[3, 5, 1]");
    test(T::from_u8(10), T::from_u8(107), "[7, 0, 1]");
    test(T::from_u8(12), T::from_u8(107), "[11, 8]");
    test(T::from_u8(57), T::from_u8(107), "[50, 1]");
    test(T::max_value(), T::from_u8(0), "[]");
    test(T::max_value(), T::from_u8(107), "[107]");
    test(T::max_value(), T::max_value() - T::from_u8(1), max_digit);
    test(T::max_value(), T::max_value(), "[0, 1]");
}

#[test]
fn test_digits_u() {
    digits_u_helper::<u8>("[254]");
    digits_u_helper::<u16>("[65534]");
    digits_u_helper::<u32>("[4294967294]");
    digits_u_helper::<u64>("[18446744073709551614]");
}

#[test]
fn test_digits_integer() {
    let test = |radix, n, out| {
        assert_eq!(
            format!(
                "{:?}",
                digits_integer(
                    &Integer::from_str(radix).unwrap(),
                    &Integer::from_str(n).unwrap(),
                )
            ),
            out
        )
    };
    test("2", "0", "[]");
    test("3", "0", "[]");
    test("8", "0", "[]");
    test("10", "0", "[]");
    test("12", "0", "[]");
    test("57", "0", "[]");
    test("2", "1", "[1]");
    test("3", "1", "[1]");
    test("8", "1", "[1]");
    test("10", "1", "[1]");
    test("12", "1", "[1]");
    test("57", "1", "[1]");
    test("2", "10", "[0, 1, 0, 1]");
    test("3", "10", "[1, 0, 1]");
    test("8", "10", "[2, 1]");
    test("10", "10", "[0, 1]");
    test("12", "10", "[10]");
    test("57", "10", "[10]");
    test("2", "187", "[1, 1, 0, 1, 1, 1, 0, 1]");
    test("3", "187", "[1, 2, 2, 0, 2]");
    test("8", "187", "[3, 7, 2]");
    test("10", "187", "[7, 8, 1]");
    test("12", "187", "[7, 3, 1]");
    test("57", "187", "[16, 3]");
}

fn digits_integer_fail_helper(radix: &str, n: &str) {
    digits_integer(
        &Integer::from_str(radix).unwrap(),
        &Integer::from_str(n).unwrap(),
    );
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
fn digits_integer_fail_1() {
    digits_integer_fail_helper("1", "10");
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
fn digits_integer_fail_2() {
    digits_integer_fail_helper("0", "10");
}

#[test]
#[should_panic(expected = "n cannot be negative. Invalid n: -1")]
fn digits_integer_fail_3() {
    digits_integer_fail_helper("2", "-1");
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
fn digits_integer_fail_4() {
    digits_integer_fail_helper("0", "-1");
}

fn digits_padded_u_helper<T: PrimUnsignedInt>(max_digit: &str) {
    let test =
        |size, radix, n, out| assert_eq!(format!("{:?}", digits_padded_u(size, radix, n)), out);
    test(0, T::from_u8(2), T::from_u8(0), "[]");
    test(0, T::from_u8(3), T::from_u8(0), "[]");
    test(0, T::from_u8(57), T::from_u8(0), "[]");
    test(0, T::from_u8(2), T::from_u8(1), "[]");
    test(0, T::from_u8(3), T::from_u8(1), "[]");
    test(0, T::from_u8(57), T::from_u8(1), "[]");
    test(0, T::from_u8(2), T::from_u8(10), "[]");
    test(0, T::from_u8(3), T::from_u8(10), "[]");
    test(0, T::from_u8(57), T::from_u8(10), "[]");
    test(0, T::from_u8(2), T::from_u8(107), "[]");
    test(0, T::from_u8(3), T::from_u8(107), "[]");
    test(0, T::from_u8(57), T::from_u8(107), "[]");
    test(1, T::from_u8(2), T::from_u8(0), "[0]");
    test(1, T::from_u8(3), T::from_u8(0), "[0]");
    test(1, T::from_u8(57), T::from_u8(0), "[0]");
    test(1, T::from_u8(2), T::from_u8(1), "[1]");
    test(1, T::from_u8(3), T::from_u8(1), "[1]");
    test(1, T::from_u8(57), T::from_u8(1), "[1]");
    test(1, T::from_u8(2), T::from_u8(10), "[0]");
    test(1, T::from_u8(3), T::from_u8(10), "[1]");
    test(1, T::from_u8(57), T::from_u8(10), "[10]");
    test(1, T::from_u8(2), T::from_u8(107), "[1]");
    test(1, T::from_u8(3), T::from_u8(107), "[2]");
    test(1, T::from_u8(57), T::from_u8(107), "[50]");
    test(2, T::from_u8(2), T::from_u8(0), "[0, 0]");
    test(2, T::from_u8(3), T::from_u8(0), "[0, 0]");
    test(2, T::from_u8(57), T::from_u8(0), "[0, 0]");
    test(2, T::from_u8(2), T::from_u8(1), "[1, 0]");
    test(2, T::from_u8(3), T::from_u8(1), "[1, 0]");
    test(2, T::from_u8(57), T::from_u8(1), "[1, 0]");
    test(2, T::from_u8(2), T::from_u8(10), "[0, 1]");
    test(2, T::from_u8(3), T::from_u8(10), "[1, 0]");
    test(2, T::from_u8(57), T::from_u8(10), "[10, 0]");
    test(2, T::from_u8(2), T::from_u8(107), "[1, 1]");
    test(2, T::from_u8(3), T::from_u8(107), "[2, 2]");
    test(2, T::from_u8(57), T::from_u8(107), "[50, 1]");
    test(8, T::from_u8(2), T::from_u8(0), "[0, 0, 0, 0, 0, 0, 0, 0]");
    test(8, T::from_u8(3), T::from_u8(0), "[0, 0, 0, 0, 0, 0, 0, 0]");
    test(8, T::from_u8(57), T::from_u8(0), "[0, 0, 0, 0, 0, 0, 0, 0]");
    test(8, T::from_u8(2), T::from_u8(1), "[1, 0, 0, 0, 0, 0, 0, 0]");
    test(8, T::from_u8(3), T::from_u8(1), "[1, 0, 0, 0, 0, 0, 0, 0]");
    test(8, T::from_u8(57), T::from_u8(1), "[1, 0, 0, 0, 0, 0, 0, 0]");
    test(8, T::from_u8(2), T::from_u8(10), "[0, 1, 0, 1, 0, 0, 0, 0]");
    test(8, T::from_u8(3), T::from_u8(10), "[1, 0, 1, 0, 0, 0, 0, 0]");
    test(
        8,
        T::from_u8(57),
        T::from_u8(10),
        "[10, 0, 0, 0, 0, 0, 0, 0]",
    );
    test(
        8,
        T::from_u8(2),
        T::from_u8(107),
        "[1, 1, 0, 1, 0, 1, 1, 0]",
    );
    test(
        8,
        T::from_u8(3),
        T::from_u8(107),
        "[2, 2, 2, 0, 1, 0, 0, 0]",
    );
    test(
        8,
        T::from_u8(57),
        T::from_u8(107),
        "[50, 1, 0, 0, 0, 0, 0, 0]",
    );
    test(1, T::max_value(), T::from_u8(0), "[0]");
    test(1, T::max_value(), T::from_u8(107), "[107]");
    test(1, T::max_value(), T::max_value() - T::from_u8(1), max_digit);
    test(2, T::max_value(), T::max_value(), "[0, 1]");
}

#[test]
fn test_digits_padded_u() {
    digits_padded_u_helper::<u8>("[254]");
    digits_padded_u_helper::<u16>("[65534]");
    digits_padded_u_helper::<u32>("[4294967294]");
    digits_padded_u_helper::<u64>("[18446744073709551614]");
}

#[test]
fn test_digits_padded_integer() {
    let test = |size, radix, n, out| {
        assert_eq!(
            format!(
                "{:?}",
                digits_padded_integer(
                    size,
                    &Integer::from_str(radix).unwrap(),
                    &Integer::from_str(n).unwrap(),
                )
            ),
            out
        )
    };
    test(0, "2", "0", "[]");
    test(0, "3", "0", "[]");
    test(0, "57", "0", "[]");
    test(0, "2", "1", "[]");
    test(0, "3", "1", "[]");
    test(0, "57", "1", "[]");
    test(0, "2", "10", "[]");
    test(0, "3", "10", "[]");
    test(0, "57", "10", "[]");
    test(0, "2", "107", "[]");
    test(0, "3", "107", "[]");
    test(0, "57", "107", "[]");
    test(1, "2", "0", "[0]");
    test(1, "3", "0", "[0]");
    test(1, "57", "0", "[0]");
    test(1, "2", "1", "[1]");
    test(1, "3", "1", "[1]");
    test(1, "57", "1", "[1]");
    test(1, "2", "10", "[0]");
    test(1, "3", "10", "[1]");
    test(1, "57", "10", "[10]");
    test(1, "2", "107", "[1]");
    test(1, "3", "107", "[2]");
    test(1, "57", "107", "[50]");
    test(2, "2", "0", "[0, 0]");
    test(2, "3", "0", "[0, 0]");
    test(2, "57", "0", "[0, 0]");
    test(2, "2", "1", "[1, 0]");
    test(2, "3", "1", "[1, 0]");
    test(2, "57", "1", "[1, 0]");
    test(2, "2", "10", "[0, 1]");
    test(2, "3", "10", "[1, 0]");
    test(2, "57", "10", "[10, 0]");
    test(2, "2", "107", "[1, 1]");
    test(2, "3", "107", "[2, 2]");
    test(2, "57", "107", "[50, 1]");
    test(8, "2", "0", "[0, 0, 0, 0, 0, 0, 0, 0]");
    test(8, "3", "0", "[0, 0, 0, 0, 0, 0, 0, 0]");
    test(8, "57", "0", "[0, 0, 0, 0, 0, 0, 0, 0]");
    test(8, "2", "1", "[1, 0, 0, 0, 0, 0, 0, 0]");
    test(8, "3", "1", "[1, 0, 0, 0, 0, 0, 0, 0]");
    test(8, "57", "1", "[1, 0, 0, 0, 0, 0, 0, 0]");
    test(8, "2", "10", "[0, 1, 0, 1, 0, 0, 0, 0]");
    test(8, "3", "10", "[1, 0, 1, 0, 0, 0, 0, 0]");
    test(8, "57", "10", "[10, 0, 0, 0, 0, 0, 0, 0]");
    test(8, "2", "107", "[1, 1, 0, 1, 0, 1, 1, 0]");
    test(8, "3", "107", "[2, 2, 2, 0, 1, 0, 0, 0]");
    test(8, "57", "107", "[50, 1, 0, 0, 0, 0, 0, 0]");
}

fn digits_padded_integer_fail_helper(size: usize, radix: &str, n: &str) {
    digits_padded_integer(
        size,
        &Integer::from_str(radix).unwrap(),
        &Integer::from_str(n).unwrap(),
    );
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
fn digits_padded_integer_fail_1() {
    digits_padded_integer_fail_helper(3, "1", "10");
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
fn digits_padded_integer_fail_2() {
    digits_padded_integer_fail_helper(3, "0", "10");
}

fn big_endian_digits_u_helper<T: PrimUnsignedInt>(max_digit: &str) {
    let test = |radix, n, out| assert_eq!(format!("{:?}", big_endian_digits_u(radix, n)), out);
    test(T::from_u8(2), T::from_u8(0), "[]");
    test(T::from_u8(3), T::from_u8(0), "[]");
    test(T::from_u8(8), T::from_u8(0), "[]");
    test(T::from_u8(10), T::from_u8(0), "[]");
    test(T::from_u8(12), T::from_u8(0), "[]");
    test(T::from_u8(57), T::from_u8(0), "[]");
    test(T::from_u8(2), T::from_u8(1), "[1]");
    test(T::from_u8(3), T::from_u8(1), "[1]");
    test(T::from_u8(8), T::from_u8(1), "[1]");
    test(T::from_u8(10), T::from_u8(1), "[1]");
    test(T::from_u8(12), T::from_u8(1), "[1]");
    test(T::from_u8(57), T::from_u8(1), "[1]");
    test(T::from_u8(2), T::from_u8(10), "[1, 0, 1, 0]");
    test(T::from_u8(3), T::from_u8(10), "[1, 0, 1]");
    test(T::from_u8(8), T::from_u8(10), "[1, 2]");
    test(T::from_u8(10), T::from_u8(10), "[1, 0]");
    test(T::from_u8(12), T::from_u8(10), "[10]");
    test(T::from_u8(57), T::from_u8(10), "[10]");
    test(T::from_u8(2), T::from_u8(107), "[1, 1, 0, 1, 0, 1, 1]");
    test(T::from_u8(3), T::from_u8(107), "[1, 0, 2, 2, 2]");
    test(T::from_u8(8), T::from_u8(107), "[1, 5, 3]");
    test(T::from_u8(10), T::from_u8(107), "[1, 0, 7]");
    test(T::from_u8(12), T::from_u8(107), "[8, 11]");
    test(T::from_u8(57), T::from_u8(107), "[1, 50]");
    test(T::max_value(), T::from_u8(0), "[]");
    test(T::max_value(), T::from_u8(107), "[107]");
    test(T::max_value(), T::max_value() - T::from_u8(1), max_digit);
    test(T::max_value(), T::max_value(), "[1, 0]");
}

#[test]
fn test_big_endian_digits_u() {
    big_endian_digits_u_helper::<u8>("[254]");
    big_endian_digits_u_helper::<u16>("[65534]");
    big_endian_digits_u_helper::<u32>("[4294967294]");
    big_endian_digits_u_helper::<u64>("[18446744073709551614]");
}

#[test]
fn test_big_endian_digits_integer() {
    let test = |radix, n, out| {
        assert_eq!(
            format!(
                "{:?}",
                big_endian_digits_integer(
                    &Integer::from_str(radix).unwrap(),
                    &Integer::from_str(n).unwrap(),
                )
            ),
            out
        )
    };
    test("2", "0", "[]");
    test("3", "0", "[]");
    test("8", "0", "[]");
    test("10", "0", "[]");
    test("12", "0", "[]");
    test("57", "0", "[]");
    test("2", "1", "[1]");
    test("3", "1", "[1]");
    test("8", "1", "[1]");
    test("10", "1", "[1]");
    test("12", "1", "[1]");
    test("57", "1", "[1]");
    test("2", "10", "[1, 0, 1, 0]");
    test("3", "10", "[1, 0, 1]");
    test("8", "10", "[1, 2]");
    test("10", "10", "[1, 0]");
    test("12", "10", "[10]");
    test("57", "10", "[10]");
    test("2", "187", "[1, 0, 1, 1, 1, 0, 1, 1]");
    test("3", "187", "[2, 0, 2, 2, 1]");
    test("8", "187", "[2, 7, 3]");
    test("10", "187", "[1, 8, 7]");
    test("12", "187", "[1, 3, 7]");
    test("57", "187", "[3, 16]");
}

fn big_endian_digits_padded_u_helper<T: PrimUnsignedInt>(max_digit: &str) {
    let test = |size, radix, n, out| {
        assert_eq!(
            format!("{:?}", big_endian_digits_padded_u(size, radix, n)),
            out
        )
    };
    test(0, T::from_u8(2), T::from_u8(0), "[]");
    test(0, T::from_u8(3), T::from_u8(0), "[]");
    test(0, T::from_u8(57), T::from_u8(0), "[]");
    test(0, T::from_u8(2), T::from_u8(1), "[]");
    test(0, T::from_u8(3), T::from_u8(1), "[]");
    test(0, T::from_u8(57), T::from_u8(1), "[]");
    test(0, T::from_u8(2), T::from_u8(10), "[]");
    test(0, T::from_u8(3), T::from_u8(10), "[]");
    test(0, T::from_u8(57), T::from_u8(10), "[]");
    test(0, T::from_u8(2), T::from_u8(107), "[]");
    test(0, T::from_u8(3), T::from_u8(107), "[]");
    test(0, T::from_u8(57), T::from_u8(107), "[]");
    test(1, T::from_u8(2), T::from_u8(0), "[0]");
    test(1, T::from_u8(3), T::from_u8(0), "[0]");
    test(1, T::from_u8(57), T::from_u8(0), "[0]");
    test(1, T::from_u8(2), T::from_u8(1), "[1]");
    test(1, T::from_u8(3), T::from_u8(1), "[1]");
    test(1, T::from_u8(57), T::from_u8(1), "[1]");
    test(1, T::from_u8(2), T::from_u8(10), "[0]");
    test(1, T::from_u8(3), T::from_u8(10), "[1]");
    test(1, T::from_u8(57), T::from_u8(10), "[10]");
    test(1, T::from_u8(2), T::from_u8(107), "[1]");
    test(1, T::from_u8(3), T::from_u8(107), "[2]");
    test(1, T::from_u8(57), T::from_u8(107), "[50]");
    test(2, T::from_u8(2), T::from_u8(0), "[0, 0]");
    test(2, T::from_u8(3), T::from_u8(0), "[0, 0]");
    test(2, T::from_u8(57), T::from_u8(0), "[0, 0]");
    test(2, T::from_u8(2), T::from_u8(1), "[0, 1]");
    test(2, T::from_u8(3), T::from_u8(1), "[0, 1]");
    test(2, T::from_u8(57), T::from_u8(1), "[0, 1]");
    test(2, T::from_u8(2), T::from_u8(10), "[1, 0]");
    test(2, T::from_u8(3), T::from_u8(10), "[0, 1]");
    test(2, T::from_u8(57), T::from_u8(10), "[0, 10]");
    test(2, T::from_u8(2), T::from_u8(107), "[1, 1]");
    test(2, T::from_u8(3), T::from_u8(107), "[2, 2]");
    test(2, T::from_u8(57), T::from_u8(107), "[1, 50]");
    test(8, T::from_u8(2), T::from_u8(0), "[0, 0, 0, 0, 0, 0, 0, 0]");
    test(8, T::from_u8(3), T::from_u8(0), "[0, 0, 0, 0, 0, 0, 0, 0]");
    test(8, T::from_u8(57), T::from_u8(0), "[0, 0, 0, 0, 0, 0, 0, 0]");
    test(8, T::from_u8(2), T::from_u8(1), "[0, 0, 0, 0, 0, 0, 0, 1]");
    test(8, T::from_u8(3), T::from_u8(1), "[0, 0, 0, 0, 0, 0, 0, 1]");
    test(8, T::from_u8(57), T::from_u8(1), "[0, 0, 0, 0, 0, 0, 0, 1]");
    test(8, T::from_u8(2), T::from_u8(10), "[0, 0, 0, 0, 1, 0, 1, 0]");
    test(8, T::from_u8(3), T::from_u8(10), "[0, 0, 0, 0, 0, 1, 0, 1]");
    test(
        8,
        T::from_u8(57),
        T::from_u8(10),
        "[0, 0, 0, 0, 0, 0, 0, 10]",
    );
    test(
        8,
        T::from_u8(2),
        T::from_u8(107),
        "[0, 1, 1, 0, 1, 0, 1, 1]",
    );
    test(
        8,
        T::from_u8(3),
        T::from_u8(107),
        "[0, 0, 0, 1, 0, 2, 2, 2]",
    );
    test(
        8,
        T::from_u8(57),
        T::from_u8(107),
        "[0, 0, 0, 0, 0, 0, 1, 50]",
    );
    test(1, T::max_value(), T::from_u8(0), "[0]");
    test(1, T::max_value(), T::from_u8(107), "[107]");
    test(1, T::max_value(), T::max_value() - T::from_u8(1), max_digit);
    test(2, T::max_value(), T::max_value(), "[1, 0]");
}

#[test]
fn test_big_endian_digits_padded_u() {
    big_endian_digits_padded_u_helper::<u8>("[254]");
    big_endian_digits_padded_u_helper::<u16>("[65534]");
    big_endian_digits_padded_u_helper::<u32>("[4294967294]");
    big_endian_digits_padded_u_helper::<u64>("[18446744073709551614]");
}

#[test]
fn test_big_endian_digits_padded_integer() {
    let test = |size, radix, n, out| {
        assert_eq!(
            format!(
                "{:?}",
                big_endian_digits_padded_integer(
                    size,
                    &Integer::from_str(radix).unwrap(),
                    &Integer::from_str(n).unwrap(),
                )
            ),
            out
        )
    };
    test(0, "2", "0", "[]");
    test(0, "3", "0", "[]");
    test(0, "57", "0", "[]");
    test(0, "2", "1", "[]");
    test(0, "3", "1", "[]");
    test(0, "57", "1", "[]");
    test(0, "2", "10", "[]");
    test(0, "3", "10", "[]");
    test(0, "57", "10", "[]");
    test(0, "2", "107", "[]");
    test(0, "3", "107", "[]");
    test(0, "57", "107", "[]");
    test(1, "2", "0", "[0]");
    test(1, "3", "0", "[0]");
    test(1, "57", "0", "[0]");
    test(1, "2", "1", "[1]");
    test(1, "3", "1", "[1]");
    test(1, "57", "1", "[1]");
    test(1, "2", "10", "[0]");
    test(1, "3", "10", "[1]");
    test(1, "57", "10", "[10]");
    test(1, "2", "107", "[1]");
    test(1, "3", "107", "[2]");
    test(1, "57", "107", "[50]");
    test(2, "2", "0", "[0, 0]");
    test(2, "3", "0", "[0, 0]");
    test(2, "57", "0", "[0, 0]");
    test(2, "2", "1", "[0, 1]");
    test(2, "3", "1", "[0, 1]");
    test(2, "57", "1", "[0, 1]");
    test(2, "2", "10", "[1, 0]");
    test(2, "3", "10", "[0, 1]");
    test(2, "57", "10", "[0, 10]");
    test(2, "2", "107", "[1, 1]");
    test(2, "3", "107", "[2, 2]");
    test(2, "57", "107", "[1, 50]");
    test(8, "2", "0", "[0, 0, 0, 0, 0, 0, 0, 0]");
    test(8, "3", "0", "[0, 0, 0, 0, 0, 0, 0, 0]");
    test(8, "57", "0", "[0, 0, 0, 0, 0, 0, 0, 0]");
    test(8, "2", "1", "[0, 0, 0, 0, 0, 0, 0, 1]");
    test(8, "3", "1", "[0, 0, 0, 0, 0, 0, 0, 1]");
    test(8, "57", "1", "[0, 0, 0, 0, 0, 0, 0, 1]");
    test(8, "2", "10", "[0, 0, 0, 0, 1, 0, 1, 0]");
    test(8, "3", "10", "[0, 0, 0, 0, 0, 1, 0, 1]");
    test(8, "57", "10", "[0, 0, 0, 0, 0, 0, 0, 10]");
    test(8, "2", "107", "[0, 1, 1, 0, 1, 0, 1, 1]");
    test(8, "3", "107", "[0, 0, 0, 1, 0, 2, 2, 2]");
    test(8, "57", "107", "[0, 0, 0, 0, 0, 0, 1, 50]");
}

fn big_endian_digits_padded_integer_fail_helper(size: usize, radix: &str, n: &str) {
    big_endian_digits_padded_integer(
        size,
        &Integer::from_str(radix).unwrap(),
        &Integer::from_str(n).unwrap(),
    );
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
fn big_endian_digits_padded_integer_fail_1() {
    big_endian_digits_padded_integer_fail_helper(3, "1", "10");
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
fn big_endian_digits_padded_integer_fail_2() {
    big_endian_digits_padded_integer_fail_helper(3, "0", "10");
}

#[test]
fn test_from_digits() {
    let test = |radix, digits, out| {
        assert_eq!(
            format!(
                "{:?}",
                from_digits(
                    &Integer::from_str(radix).unwrap(),
                    &parse_vec(digits).unwrap(),
                )
            ),
            out
        )
    };
    test("2", "[]", "0");
    test("3", "[]", "0");
    test("8", "[]", "0");
    test("10", "[]", "0");
    test("12", "[]", "0");
    test("57", "[]", "0");
    test("2", "[1]", "1");
    test("3", "[1]", "1");
    test("8", "[1]", "1");
    test("10", "[1]", "1");
    test("12", "[1]", "1");
    test("57", "[1]", "1");
    test("2", "[0, 1, 0, 1]", "10");
    test("3", "[1, 0, 1]", "10");
    test("8", "[2, 1]", "10");
    test("10", "[0, 1]", "10");
    test("12", "[10]", "10");
    test("57", "[10]", "10");
    test("2", "[1, 1, 0, 1, 1, 1, 0, 1]", "187");
    test("3", "[1, 2, 2, 0, 2]", "187");
    test("8", "[3, 7, 2]", "187");
    test("10", "[7, 8, 1]", "187");
    test("12", "[7, 3, 1]", "187");
    test("57", "[16, 3]", "187");
}

fn from_digits_fail_helper(radix: &str, digits: &str) {
    from_digits(
        &Integer::from_str(radix).unwrap(),
        &parse_vec(digits).unwrap(),
    );
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
fn from_digits_fail_1() {
    from_digits_fail_helper("1", "[1, 2, 3]");
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
fn from_digits_fail_2() {
    from_digits_fail_helper("0", "[1, 2, 3]");
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: -1")]
fn from_digits_fail_3() {
    from_digits_fail_helper("-1", "[1, 2, 3]");
}

#[test]
#[should_panic(expected = "Each element of digits must be non-negative. Invalid digit: -1 in \
                           [-1, 0, 1]")]
fn from_digits_fail_4() {
    from_digits_fail_helper("2", "[-1, 0, 1]");
}

#[test]
#[should_panic(expected = "Each element of digits must be less than radix, which is 2. Invalid \
                           digit: 2 in [1, 0, 2]")]
fn from_digits_fail_5() {
    from_digits_fail_helper("2", "[1, 0, 2]");
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn from_digits_fail_6() {
    from_digits_fail_helper("10", "[-1, 2, 3]");
}

#[test]
#[should_panic(expected = "Result::unwrap()` on an `Err` value: ParseIntegerError { kind: \
                           InvalidDigit }")]
fn from_digits_fail_7() {
    from_digits_fail_helper("10", "[1, 2, 10]");
}

#[test]
#[should_panic(expected = "Each element of digits must be non-negative. Invalid digit: -1 in \
                           [-1, 2, 3]")]
fn from_digits_fail_8() {
    from_digits_fail_helper("100", "[-1, 2, 3]");
}

#[test]
#[should_panic(expected = "Each element of digits must be less than radix, which is 100. Invalid \
                           digit: 100 in [1, 2, 100]")]
fn from_digits_fail_9() {
    from_digits_fail_helper("100", "[1, 2, 100]");
}

#[test]
fn test_from_big_endian_digits() {
    let test = |radix, digits, out| {
        assert_eq!(
            format!(
                "{:?}",
                from_big_endian_digits(
                    &Integer::from_str(radix).unwrap(),
                    &parse_vec(digits).unwrap(),
                )
            ),
            out
        )
    };
    test("2", "[]", "0");
    test("3", "[]", "0");
    test("8", "[]", "0");
    test("10", "[]", "0");
    test("12", "[]", "0");
    test("57", "[]", "0");
    test("2", "[1]", "1");
    test("3", "[1]", "1");
    test("8", "[1]", "1");
    test("10", "[1]", "1");
    test("12", "[1]", "1");
    test("57", "[1]", "1");
    test("2", "[1, 0, 1, 0]", "10");
    test("3", "[1, 0, 1]", "10");
    test("8", "[1, 2]", "10");
    test("10", "[1, 0]", "10");
    test("12", "[10]", "10");
    test("57", "[10]", "10");
    test("2", "[1, 0, 1, 1, 1, 0, 1, 1]", "187");
    test("3", "[2, 0, 2, 2, 1]", "187");
    test("8", "[2, 7, 3]", "187");
    test("10", "[1, 8, 7]", "187");
    test("12", "[1, 3, 7]", "187");
    test("57", "[3, 16]", "187");
}

fn from_big_endian_digits_fail_helper(radix: &str, digits: &str) {
    from_big_endian_digits(
        &Integer::from_str(radix).unwrap(),
        &parse_vec(digits).unwrap(),
    );
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 1")]
fn from_big_endian_digits_fail_1() {
    from_big_endian_digits_fail_helper("1", "[1, 2, 3]");
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: 0")]
fn from_big_endian_digits_fail_2() {
    from_big_endian_digits_fail_helper("0", "[1, 2, 3]");
}

#[test]
#[should_panic(expected = "radix must be at least 2. Invalid radix: -1")]
fn from_big_endian_digits_fail_3() {
    from_big_endian_digits_fail_helper("-1", "[1, 2, 3]");
}

#[test]
#[should_panic(expected = "Each element of digits must be non-negative. Invalid digit: -1 in \
                           [-1, 0, 1]")]
fn from_big_endian_digits_fail_4() {
    from_big_endian_digits_fail_helper("2", "[-1, 0, 1]");
}

#[test]
#[should_panic(expected = "Each element of digits must be less than radix, which is 2. Invalid \
                           digit: 2 in [1, 0, 2]")]
fn from_big_endian_digits_fail_5() {
    from_big_endian_digits_fail_helper("2", "[1, 0, 2]");
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn from_big_endian_digits_fail_6() {
    from_big_endian_digits_fail_helper("10", "[-1, 2, 3]");
}

#[test]
#[should_panic(expected = "Result::unwrap()` on an `Err` value: ParseIntegerError { kind: \
                           InvalidDigit }")]
fn from_big_endian_digits_fail_7() {
    from_big_endian_digits_fail_helper("10", "[1, 2, 10]");
}

#[test]
#[should_panic(expected = "Each element of digits must be non-negative. Invalid digit: -1 in \
                           [-1, 2, 3]")]
fn from_big_endian_digits_fail_8() {
    from_big_endian_digits_fail_helper("100", "[-1, 2, 3]");
}

#[test]
#[should_panic(expected = "Each element of digits must be less than radix, which is 100. Invalid \
                           digit: 100 in [1, 2, 100]")]
fn from_big_endian_digits_fail_9() {
    from_big_endian_digits_fail_helper("100", "[1, 2, 100]");
}
